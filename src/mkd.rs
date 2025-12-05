//创建文件夹功能模块

use core::{error::Error};

use alloc::string::String;
use alloc::vec::Vec;
use crate::BLOCK_SIZE;
use crate::blockdev::{BlockDev, BlockDevice, BlockDevResult, BlockDevError};
use crate::{
    disknode::Ext4Inode,
    entries::Ext4DirEntry2,
    ext4::{Ext4FileSystem, file_entry_exisr},
};
use crate::alloc::string::ToString;
use log::debug;
use crate::endian::DiskFormat;
use crate::disknode::{Ext4Extent, Ext4ExtentHeader};
#[derive(Debug)]
pub enum FileError {
    DirExist,
    FileExist,
    DirNotFound,
    FileNotFound,
}

pub fn split_paren_child(pat:&str)->(String,String){
    let _pos:usize = pat.rfind("/").expect("Can't spilit path");
    let (parent,child) = pat.split_at(_pos);
    (parent.to_string(),child[1..].to_string())
}
/// 尚未实现Extend树
/// 为目录构建平坦extend映射：
/// 如果超级块启用 extents，则在 i_block 中写入 extent header + 一个叶子 extent；
/// 否则使用传统的直接块指针（i_block[0] = data_block）。
fn build_single_block_dir_mapping(fs: &Ext4FileSystem, data_block: u64) -> (u32, [u32; 15]) {
    // 基础标志始终包含目录同步
    let mut flags = Ext4Inode::EXT4_DIRSYNC_FL;
    let mut iblock: [u32; 15] = [0; 15];

    if fs.superblock.has_extents() {
        // 启用 extents：设置 extent 标志，并在 i_block 中写入一个 extent header + 1 个 extent
        flags |= Ext4Inode::EXT4_EXTENTS_FL;

        let mut header = Ext4ExtentHeader::new();
        // 这里只放 1 个叶子 extent
        header.eh_entries = 1;

        let extent = Ext4Extent::new(0, data_block, 1);
        let mut exts: Vec<&Ext4Extent> = Vec::new();
        exts.push(&extent);

        Ext4Extent::write_extend_to_iblock(&mut iblock, exts, &header);
    } else {
        // 传统模式：直接块指针
        iblock[0] = data_block as u32;
    }

    (flags, iblock)
}

///尚未完成！
///通用文件夹创建
pub fn mkd<B: BlockDevice>(device: &mut BlockDev<B>, fs: &mut Ext4FileSystem, path: &str) -> Option<Ext4Inode> {
    let (parent, child) = split_paren_child(path);
    // 根目录创建
    if (parent == "" || parent == "/") && (child == "") {
        debug!("Creating root directory");
        if let Err(e) = create_root_directory_entry(fs, device) {
            debug!("create_root_directory_entry failed: {:?}", e);
            return None;
        }
        return fs.get_root(device).ok();
    }

    // /lost+found 目录创建
    if (parent == "" || parent == "/") && (child == "lost+found") {
        debug!("Creating /lost+found directory");
        if let Err(e) = create_lost_found_directory(fs, device) {
            debug!("create_lost_found_directory failed: {:?}", e);
            return None;
        }
        return fs.find_file_line(device, "/lost+found");
    }


    None
}

/// 根目录创建实现
pub fn create_root_directory_entry<B: BlockDevice>(
    fs: &mut Ext4FileSystem,
    block_dev: &mut BlockDev<B>,
) -> BlockDevResult<()> {
    debug!("Initializing root directory...");
    // 是否需要创建根目录由挂载流程基于 inode 内容判断，这里只负责真正的创建

    //  为根目录分配一个数据块
    let root_inode_num = fs.root_inode;
    let group_idx = fs
        .find_group_with_free_blocks()
        .ok_or(BlockDevError::NoSpace)?;
    let data_block = fs.alloc_block(block_dev, group_idx)?;

    //  写入目录项 . 和 ..
    {
        let cached = fs.datablock_cache.create_new(data_block);
        let data = &mut cached.data;

        // . 目录项
        let dot_name = b".";
        let dot_rec_len = Ext4DirEntry2::entry_len(dot_name.len() as u8);
        let dot = Ext4DirEntry2::new(
            root_inode_num,
            dot_rec_len,
            Ext4DirEntry2::EXT4_FT_DIR,
            dot_name,
        );

        // ..目录项（根的父目录仍为自己）
        let dotdot_name = b"..";
        let dotdot_rec_len = (BLOCK_SIZE as u16).saturating_sub(dot_rec_len);
        let dotdot = Ext4DirEntry2::new(
            root_inode_num,
            dotdot_rec_len,
            Ext4DirEntry2::EXT4_FT_DIR,
            dotdot_name,
        );

        {
            dot.to_disk_bytes(&mut data[0..8]);
            let name_len = dot.name_len as usize;
            data[8..8 + name_len].copy_from_slice(&dot.name[..name_len]);
        }

        {
            let offset = dot_rec_len as usize;
            dotdot.to_disk_bytes(&mut data[offset..offset + 8]);
            let name_len = dotdot.name_len as usize;
            data[offset + 8..offset + 8 + name_len]
                .copy_from_slice(&dotdot.name[..name_len]);
        }
    }

    //  初始化根目录 inode：inode 表起始块号从块组描述符读取
    let inode_table_start = match fs.group_descs.get(0) {
        Some(desc) => desc.inode_table() as u64,
        None => return Err(BlockDevError::Corrupted),
    };
    let (block_num, offset, _group_idx) = fs.inodetable_cahce.calc_inode_location(
        fs.root_inode,
        fs.superblock.s_inodes_per_group,
        inode_table_start,
        BLOCK_SIZE,
    );

    // 根据是否启用 extents 构建 i_flags 增量和 i_block 内容
    let (flags, iblock) = build_single_block_dir_mapping(fs, data_block);

    fs.inodetable_cahce.modify(
        block_dev,
        fs.root_inode as u64,
        block_num,
        offset,
        |inode| {
            inode.i_mode = Ext4Inode::S_IFDIR | 0o755; // 目录 + 权限
            inode.i_links_count = 2; // . 和 ..
            inode.i_size_lo = BLOCK_SIZE as u32;
            inode.i_size_high = 0;
            // i_blocks 以 512 字节为单位
            inode.i_blocks_lo = (BLOCK_SIZE / 512) as u32;
            inode.l_i_blocks_high = 0;
            inode.i_flags |= flags;
            inode.i_block = iblock;
        },
    )?;

    //块组描述符更新 目录数
    if let Some(desc) = fs.get_group_desc_mut(0) {
        let newc = desc.used_dirs_count().saturating_add(1);
        desc.bg_used_dirs_count_lo = (newc & 0xFFFF) as u16;
        desc.bg_used_dirs_count_hi = ((newc >> 16) & 0xFFFF) as u16;
    }

    debug!("Root directory created: inode={}, data_block={}", fs.root_inode, data_block);
    Ok(())
}

/// 创建 /lost+found 目录，并将其挂到根目录下
pub fn create_lost_found_directory<B: BlockDevice>(
    fs: &mut Ext4FileSystem,
    block_dev: &mut BlockDev<B>,
) -> BlockDevResult<()> {
    // 如果已经存在则直接返回
    if file_entry_exisr(fs, block_dev, "/lost+found") {
        return Ok(());
    }

    let root_inode_num = fs.root_inode;

    //  分配 inode
    let inode_group = fs
        .find_group_with_free_inodes()
        .ok_or(BlockDevError::NoSpace)?;
    let lost_ino = fs.alloc_inode(block_dev, inode_group)?;
    debug!("lost+found inode: {}", lost_ino);

    //  分配数据块
    let block_group = fs
        .find_group_with_free_blocks()
        .ok_or(BlockDevError::NoSpace)?;
    let data_block = fs.alloc_block(block_dev, block_group)?;

    //  初始化 lost+found 目录块（".", ".."）
    {
        let cached = fs.datablock_cache.create_new(data_block);
        let data = &mut cached.data;

        let dot_name = b".";
        let dot_rec_len = Ext4DirEntry2::entry_len(dot_name.len() as u8);
        let dot = Ext4DirEntry2::new(
            lost_ino,
            dot_rec_len,
            Ext4DirEntry2::EXT4_FT_DIR,
            dot_name,
        );

        let dotdot_name = b"..";
        let dotdot_rec_len = (BLOCK_SIZE as u16).saturating_sub(dot_rec_len);
        let dotdot = Ext4DirEntry2::new(
            root_inode_num,
            dotdot_rec_len,
            Ext4DirEntry2::EXT4_FT_DIR,
            dotdot_name,
        );

        {
            dot.to_disk_bytes(&mut data[0..8]);
            let name_len = dot.name_len as usize;
            data[8..8 + name_len].copy_from_slice(&dot.name[..name_len]);
        }

        {
            let offset = dot_rec_len as usize;
            dotdot.to_disk_bytes(&mut data[offset..offset + 8]);
            let name_len = dotdot.name_len as usize;
            data[offset + 8..offset + 8 + name_len]
                .copy_from_slice(&dotdot.name[..name_len]);
        }
    }

    //  写 lost+found inode
    let (lf_group, _idx) = fs.inode_allocator.global_to_group(lost_ino);
    let inode_table_start = match fs.group_descs.get(lf_group as usize) {
        Some(desc) => desc.inode_table() as u64,
        None => return Err(BlockDevError::Corrupted),
    };
    let (block_num, offset, _group_idx) = fs.inodetable_cahce.calc_inode_location(
        lost_ino,
        fs.superblock.s_inodes_per_group,
        inode_table_start,
        BLOCK_SIZE,
    );

    // lost+found 的数据块映射与根目录保持一致：单块目录，按特性选择 extent 或直接块
    let (flags, iblock) = build_single_block_dir_mapping(fs, data_block);

    fs.inodetable_cahce.modify(
        block_dev,
        lost_ino as u64,
        block_num,
        offset,
        |inode| {
            inode.i_mode = Ext4Inode::S_IFDIR | 0o755;
            inode.i_links_count = 2;
            inode.i_size_lo = BLOCK_SIZE as u32;
            inode.i_size_high = 0;
            inode.i_blocks_lo = (BLOCK_SIZE / 512) as u32;
            inode.l_i_blocks_high = 0;
            inode.i_flags |= flags;
            inode.i_block = iblock;
        },
    )?;

    if let Some(desc) = fs.get_group_desc_mut(lf_group) {
        let newc = desc.used_dirs_count().saturating_add(1);
        desc.bg_used_dirs_count_lo = (newc & 0xFFFF) as u16;
        desc.bg_used_dirs_count_hi = ((newc >> 16) & 0xFFFF) as u16;
    }

    //  更新根目录数据块：加入 lost+found 目录项
    let root_inode = fs.get_root(block_dev)?;
    let root_block = root_inode.i_block[0];
    if root_block == 0 {
        return Err(BlockDevError::Corrupted);
    }

    fs.datablock_cache.modify(block_dev, root_block as u64, move |data| {
        let dot_name = b".";
        let dot_rec_len = Ext4DirEntry2::entry_len(dot_name.len() as u8);
        let dot = Ext4DirEntry2::new(
            root_inode_num,
            dot_rec_len,
            Ext4DirEntry2::EXT4_FT_DIR,
            dot_name,
        );

        let dotdot_name = b"..";
        let dotdot_rec_len = Ext4DirEntry2::entry_len(dotdot_name.len() as u8);
        let dotdot = Ext4DirEntry2::new(
            root_inode_num,
            dotdot_rec_len,
            Ext4DirEntry2::EXT4_FT_DIR,
            dotdot_name,
        );

        let lf_name = b"lost+found";
        let lf_rec_len = (BLOCK_SIZE as u16).saturating_sub(dot_rec_len + dotdot_rec_len);
        let lost = Ext4DirEntry2::new(
            lost_ino,
            lf_rec_len,
            Ext4DirEntry2::EXT4_FT_DIR,
            lf_name,
        );

        // 清零整个块
        for b in data.iter_mut() {
            *b = 0;
        }

        // 写 .
        dot.to_disk_bytes(&mut data[0..8]);
        let name_len = dot.name_len as usize;
        data[8..8 + name_len].copy_from_slice(&dot.name[..name_len]);

        // 写 ..
        let mut offset = dot_rec_len as usize;
        dotdot.to_disk_bytes(&mut data[offset..offset + 8]);
        let dd_len = dotdot.name_len as usize;
        data[offset + 8..offset + 8 + dd_len]
            .copy_from_slice(&dotdot.name[..dd_len]);

        // 写 lost+found
        offset += dotdot_rec_len as usize;
        lost.to_disk_bytes(&mut data[offset..offset + 8]);
        let lf_len = lost.name_len as usize;
        data[offset + 8..offset + 8 + lf_len]
            .copy_from_slice(&lost.name[..lf_len]);
    })?;

    //  更新根 inode 的链接计数（多了一个子目录）
    let inode_table_start = match fs.group_descs.get(0) {
        Some(desc) => desc.inode_table() as u64,
        None => return Err(BlockDevError::Corrupted),
    };
    let (block_num, offset, _group_idx) = fs.inodetable_cahce.calc_inode_location(
        fs.root_inode,
        fs.superblock.s_inodes_per_group,
        inode_table_start,
        BLOCK_SIZE,
    );

    fs.inodetable_cahce.modify(
        block_dev,
        fs.root_inode as u64,
        block_num,
        offset,
        |inode| {
            inode.i_links_count = inode.i_links_count.saturating_add(1);
        },
    )?;


    //  记录到超级块
    fs.superblock.s_lpf_ino = lost_ino;

    debug!(
        "lost+found directory created: inode={}, data_block={}",
        lost_ino,
        data_block
    );



    Ok(())
}
