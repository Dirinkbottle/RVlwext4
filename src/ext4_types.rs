//! EXT4 数据结构定义
//! 对应 ext4_types.h

#![allow(non_camel_case_types)]
#![allow(dead_code)]

// ============================================================================
// 常量定义
// ============================================================================

pub const UUID_SIZE: usize = 16;

// 超级块相关
pub const EXT4_SUPERBLOCK_MAGIC: u16 = 0xEF53;
pub const EXT4_SUPERBLOCK_SIZE: usize = 1024;
pub const EXT4_SUPERBLOCK_OFFSET: usize = 1024;

pub const EXT4_SUPERBLOCK_OS_LINUX: u32 = 0;
pub const EXT4_SUPERBLOCK_OS_HURD: u32 = 1;

// 超级块标志
pub const EXT4_SUPERBLOCK_FLAGS_SIGNED_HASH: u32 = 0x0001;
pub const EXT4_SUPERBLOCK_FLAGS_UNSIGNED_HASH: u32 = 0x0002;
pub const EXT4_SUPERBLOCK_FLAGS_TEST_FILESYS: u32 = 0x0004;

// 文件系统状态
pub const EXT4_SUPERBLOCK_STATE_VALID_FS: u16 = 0x0001; // 正常卸载
pub const EXT4_SUPERBLOCK_STATE_ERROR_FS: u16 = 0x0002; // 检测到错误
pub const EXT4_SUPERBLOCK_STATE_ORPHAN_FS: u16 = 0x0004; // 正在恢复孤儿文件

// 错误处理行为
pub const EXT4_SUPERBLOCK_ERRORS_CONTINUE: u16 = 1; // 继续执行
pub const EXT4_SUPERBLOCK_ERRORS_RO: u16 = 2; // 重新挂载为只读
pub const EXT4_SUPERBLOCK_ERRORS_PANIC: u16 = 3; // 内核恐慌

// 兼容特性
pub const EXT4_FCOM_DIR_PREALLOC: u32 = 0x0001;
pub const EXT4_FCOM_IMAGIC_INODES: u32 = 0x0002;
pub const EXT4_FCOM_HAS_JOURNAL: u32 = 0x0004;
pub const EXT4_FCOM_EXT_ATTR: u32 = 0x0008;
pub const EXT4_FCOM_RESIZE_INODE: u32 = 0x0010;
pub const EXT4_FCOM_DIR_INDEX: u32 = 0x0020;

// 只读兼容特性
pub const EXT4_FRO_COM_SPARSE_SUPER: u32 = 0x0001;
pub const EXT4_FRO_COM_LARGE_FILE: u32 = 0x0002;
pub const EXT4_FRO_COM_BTREE_DIR: u32 = 0x0004;
pub const EXT4_FRO_COM_HUGE_FILE: u32 = 0x0008;
pub const EXT4_FRO_COM_GDT_CSUM: u32 = 0x0010;
pub const EXT4_FRO_COM_DIR_NLINK: u32 = 0x0020;
pub const EXT4_FRO_COM_EXTRA_ISIZE: u32 = 0x0040;
pub const EXT4_FRO_COM_QUOTA: u32 = 0x0100;
pub const EXT4_FRO_COM_BIGALLOC: u32 = 0x0200;
pub const EXT4_FRO_COM_METADATA_CSUM: u32 = 0x0400;

// 不兼容特性
pub const EXT4_FINCOM_COMPRESSION: u32 = 0x0001;
pub const EXT4_FINCOM_FILETYPE: u32 = 0x0002;
pub const EXT4_FINCOM_RECOVER: u32 = 0x0004;
pub const EXT4_FINCOM_JOURNAL_DEV: u32 = 0x0008;
pub const EXT4_FINCOM_META_BG: u32 = 0x0010;
pub const EXT4_FINCOM_EXTENTS: u32 = 0x0040;
pub const EXT4_FINCOM_64BIT: u32 = 0x0080;
pub const EXT4_FINCOM_MMP: u32 = 0x0100;
pub const EXT4_FINCOM_FLEX_BG: u32 = 0x0200;
pub const EXT4_FINCOM_EA_INODE: u32 = 0x0400;
pub const EXT4_FINCOM_DIRDATA: u32 = 0x1000;
pub const EXT4_FINCOM_BG_USE_META_CSUM: u32 = 0x2000;
pub const EXT4_FINCOM_LARGEDIR: u32 = 0x4000;
pub const EXT4_FINCOM_INLINE_DATA: u32 = 0x8000;

// 块组标志
pub const EXT4_BLOCK_GROUP_INODE_UNINIT: u16 = 0x0001;
pub const EXT4_BLOCK_GROUP_BLOCK_UNINIT: u16 = 0x0002;
pub const EXT4_BLOCK_GROUP_ITABLE_ZEROED: u16 = 0x0004;

pub const EXT4_MIN_BLOCK_GROUP_DESCRIPTOR_SIZE: usize = 32;
pub const EXT4_MAX_BLOCK_GROUP_DESCRIPTOR_SIZE: usize = 64;

pub const EXT4_MIN_BLOCK_SIZE: usize = 1024;
pub const EXT4_MAX_BLOCK_SIZE: usize = 65536;
pub const EXT4_REV0_INODE_SIZE: usize = 128;
pub const EXT4_INODE_BLOCK_SIZE: usize = 512;

// Inode 块指针
pub const EXT4_INODE_DIRECT_BLOCK_COUNT: usize = 12;
pub const EXT4_INODE_INDIRECT_BLOCK: usize = 12;
pub const EXT4_INODE_DOUBLE_INDIRECT_BLOCK: usize = 13;
pub const EXT4_INODE_TRIPPLE_INDIRECT_BLOCK: usize = 14;
pub const EXT4_INODE_BLOCKS: usize = 15;

// Inode 模式
pub const EXT4_INODE_MODE_FIFO: u16 = 0x1000;
pub const EXT4_INODE_MODE_CHARDEV: u16 = 0x2000;
pub const EXT4_INODE_MODE_DIRECTORY: u16 = 0x4000;
pub const EXT4_INODE_MODE_BLOCKDEV: u16 = 0x6000;
pub const EXT4_INODE_MODE_FILE: u16 = 0x8000;
pub const EXT4_INODE_MODE_SOFTLINK: u16 = 0xA000;
pub const EXT4_INODE_MODE_SOCKET: u16 = 0xC000;
pub const EXT4_INODE_MODE_TYPE_MASK: u16 = 0xF000;

// Inode 标志
pub const EXT4_INODE_FLAG_SECRM: u32 = 0x00000001;
pub const EXT4_INODE_FLAG_UNRM: u32 = 0x00000002;
pub const EXT4_INODE_FLAG_COMPR: u32 = 0x00000004;
pub const EXT4_INODE_FLAG_SYNC: u32 = 0x00000008;
pub const EXT4_INODE_FLAG_IMMUTABLE: u32 = 0x00000010;
pub const EXT4_INODE_FLAG_APPEND: u32 = 0x00000020;
pub const EXT4_INODE_FLAG_NODUMP: u32 = 0x00000040;
pub const EXT4_INODE_FLAG_NOATIME: u32 = 0x00000080;
pub const EXT4_INODE_FLAG_DIRTY: u32 = 0x00000100;
pub const EXT4_INODE_FLAG_COMPRBLK: u32 = 0x00000200;
pub const EXT4_INODE_FLAG_NOCOMPR: u32 = 0x00000400;
pub const EXT4_INODE_FLAG_ECOMPR: u32 = 0x00000800;
pub const EXT4_INODE_FLAG_INDEX: u32 = 0x00001000;
pub const EXT4_INODE_FLAG_IMAGIC: u32 = 0x00002000;
pub const EXT4_INODE_FLAG_JOURNAL_DATA: u32 = 0x00004000;
pub const EXT4_INODE_FLAG_NOTAIL: u32 = 0x00008000;
pub const EXT4_INODE_FLAG_DIRSYNC: u32 = 0x00010000;
pub const EXT4_INODE_FLAG_TOPDIR: u32 = 0x00020000;
pub const EXT4_INODE_FLAG_HUGE_FILE: u32 = 0x00040000;
pub const EXT4_INODE_FLAG_EXTENTS: u32 = 0x00080000;
pub const EXT4_INODE_FLAG_EA_INODE: u32 = 0x00200000;
pub const EXT4_INODE_FLAG_EOFBLOCKS: u32 = 0x00400000;
pub const EXT4_INODE_FLAG_RESERVED: u32 = 0x80000000;

pub const EXT4_INODE_ROOT_INDEX: u32 = 2;

// 目录项
pub const EXT4_DIRECTORY_FILENAME_LEN: usize = 255;
pub const EXT4_DIRENTRY_DIR_CSUM: u8 = 0xDE;

// 保留 Inode
pub const EXT4_BAD_INO: u32 = 1;
pub const EXT4_ROOT_INO: u32 = 2;
pub const EXT4_BOOT_LOADER_INO: u32 = 5;
pub const EXT4_UNDEL_DIR_INO: u32 = 6;
pub const EXT4_RESIZE_INO: u32 = 7;
pub const EXT4_JOURNAL_INO: u32 = 8;
pub const EXT4_GOOD_OLD_FIRST_INO: u32 = 11;

pub const EXT4_LINK_MAX: u32 = 65000;

// HTree 哈希版本
pub const EXT2_HTREE_LEGACY: u8 = 0;
pub const EXT2_HTREE_HALF_MD4: u8 = 1;
pub const EXT2_HTREE_TEA: u8 = 2;
pub const EXT2_HTREE_LEGACY_UNSIGNED: u8 = 3;
pub const EXT2_HTREE_HALF_MD4_UNSIGNED: u8 = 4;
pub const EXT2_HTREE_TEA_UNSIGNED: u8 = 5;
pub const EXT2_HTREE_EOF: u32 = 0x7FFFFFFF;

pub const EXT4_GOOD_OLD_INODE_SIZE: usize = 128;

// CRC32
pub const EXT4_CRC32_INIT: u32 = 0xFFFFFFFF;
pub const EXT4_CHECKSUM_CRC32C: u8 = 1;

// JBD
pub const JBD_MAGIC_NUMBER: u32 = 0xc03b3998;

// ============================================================================
// 类型别名
// ============================================================================

pub type ext4_lblk_t = u32;
pub type ext4_fsblk_t = u64;

// ============================================================================
// 数据结构
// ============================================================================

/// EXT4 超级块
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ext4_sblock {
    pub inodes_count: u32,
    pub blocks_count_lo: u32,
    pub reserved_blocks_count_lo: u32,
    pub free_blocks_count_lo: u32,
    pub free_inodes_count: u32,
    pub first_data_block: u32,
    pub log_block_size: u32,
    pub log_cluster_size: u32,
    pub blocks_per_group: u32,
    pub frags_per_group: u32,
    pub inodes_per_group: u32,
    pub mount_time: u32,
    pub write_time: u32,
    pub mount_count: u16,
    pub max_mount_count: u16,
    pub magic: u16,
    pub state: u16,
    pub errors: u16,
    pub minor_rev_level: u16,
    pub last_check_time: u32,
    pub check_interval: u32,
    pub creator_os: u32,
    pub rev_level: u32,
    pub def_resuid: u16,
    pub def_resgid: u16,

    // EXT4_DYNAMIC_REV
    pub first_inode: u32,
    pub inode_size: u16,
    pub block_group_index: u16,
    pub features_compatible: u32,
    pub features_incompatible: u32,
    pub features_read_only: u32,
    pub uuid: [u8; UUID_SIZE],
    pub volume_name: [u8; 16],
    pub last_mounted: [u8; 64],
    pub algorithm_usage_bitmap: u32,

    // 性能提示
    pub s_prealloc_blocks: u8,
    pub s_prealloc_dir_blocks: u8,
    pub s_reserved_gdt_blocks: u16,

    // 日志支持
    pub journal_uuid: [u8; UUID_SIZE],
    pub journal_inode_number: u32,
    pub journal_dev: u32,
    pub last_orphan: u32,
    pub hash_seed: [u32; 4],
    pub default_hash_version: u8,
    pub journal_backup_type: u8,
    pub desc_size: u16,
    pub default_mount_opts: u32,
    pub first_meta_bg: u32,
    pub mkfs_time: u32,
    pub journal_blocks: [u32; 17],

    // 64位支持
    pub blocks_count_hi: u32,
    pub reserved_blocks_count_hi: u32,
    pub free_blocks_count_hi: u32,
    pub min_extra_isize: u16,
    pub want_extra_isize: u16,
    pub flags: u32,
    pub raid_stride: u16,
    pub mmp_interval: u16,
    pub mmp_block: u64,
    pub raid_stripe_width: u32,
    pub log_groups_per_flex: u8,
    pub checksum_type: u8,
    pub reserved_pad: u16,
    pub kbytes_written: u64,
    pub snapshot_inum: u32,
    pub snapshot_id: u32,
    pub snapshot_r_blocks_count: u64,
    pub snapshot_list: u32,

    // 错误统计
    pub error_count: u32,
    pub first_error_time: u32,
    pub first_error_ino: u32,
    pub first_error_block: u64,
    pub first_error_func: [u8; 32],
    pub first_error_line: u32,
    pub last_error_time: u32,
    pub last_error_ino: u32,
    pub last_error_line: u32,
    pub last_error_block: u64,
    pub last_error_func: [u8; 32],
    pub mount_opts: [u8; 64],
    pub usr_quota_inum: u32,
    pub grp_quota_inum: u32,
    pub overhead_clusters: u32,
    pub backup_bgs: [u32; 2],
    pub encrypt_algos: [u8; 4],
    pub encrypt_pw_salt: [u8; 16],
    pub lpf_ino: u32,
    pub padding: [u32; 100],
    pub checksum: u32,
}

/// 块组描述符
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ext4_bgroup {
    pub block_bitmap_lo: u32,
    pub inode_bitmap_lo: u32,
    pub inode_table_first_block_lo: u32,
    pub free_blocks_count_lo: u16,
    pub free_inodes_count_lo: u16,
    pub used_dirs_count_lo: u16,
    pub flags: u16,
    pub exclude_bitmap_lo: u32,
    pub block_bitmap_csum_lo: u16,
    pub inode_bitmap_csum_lo: u16,
    pub itable_unused_lo: u16,
    pub checksum: u16,

    // 64位扩展
    pub block_bitmap_hi: u32,
    pub inode_bitmap_hi: u32,
    pub inode_table_first_block_hi: u32,
    pub free_blocks_count_hi: u16,
    pub free_inodes_count_hi: u16,
    pub used_dirs_count_hi: u16,
    pub itable_unused_hi: u16,
    pub exclude_bitmap_hi: u32,
    pub block_bitmap_csum_hi: u16,
    pub inode_bitmap_csum_hi: u16,
    pub reserved: u32,
}

/// Inode OSD2 Linux 变体
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ext4_inode_osd2_linux {
    pub blocks_high: u16,
    pub file_acl_high: u16,
    pub uid_high: u16,
    pub gid_high: u16,
    pub checksum_lo: u16,
    pub reserved2: u16,
}

/// Inode OSD2 Hurd 变体
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ext4_inode_osd2_hurd {
    pub reserved1: u16,
    pub mode_high: u16,
    pub uid_high: u16,
    pub gid_high: u16,
    pub author: u32,
}

/// Inode OSD2 联合体
#[repr(C)]
#[derive(Copy, Clone)]
pub union ext4_inode_osd2 {
    pub linux2: ext4_inode_osd2_linux,
    pub hurd2: ext4_inode_osd2_hurd,
}

/// EXT4 Inode
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ext4_inode {
    pub mode: u16,
    pub uid: u16,
    pub size_lo: u32,
    pub access_time: u32,
    pub change_inode_time: u32,
    pub modification_time: u32,
    pub deletion_time: u32,
    pub gid: u16,
    pub links_count: u16,
    pub blocks_count_lo: u32,
    pub flags: u32,
    pub unused_osd1: u32,
    pub blocks: [u32; EXT4_INODE_BLOCKS], // 15个块指针
    pub generation: u32,
    pub file_acl_lo: u32,
    pub size_hi: u32,
    pub obso_faddr: u32,

    pub osd2: ext4_inode_osd2,

    pub extra_isize: u16,
    pub checksum_hi: u16,
    pub ctime_extra: u32,
    pub mtime_extra: u32,
    pub atime_extra: u32,
    pub crtime: u32,
    pub crtime_extra: u32,
    pub version_hi: u32,
}

/// 目录项类型
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Ext4DirEntryType {
    Unknown = 0,
    RegFile = 1,
    Dir = 2,
    Chrdev = 3,
    Blkdev = 4,
    Fifo = 5,
    Sock = 6,
    Symlink = 7,
}

/// 目录项内部字段（union）
#[repr(C)]
#[derive(Copy, Clone)]
pub union ext4_dir_en_internal {
    pub name_length_high: u8,
    pub inode_type: u8,
}

/// 目录项（注意：name 是柔性数组）
#[repr(C, packed)]
pub struct ext4_dir_en {
    pub inode: u32,
    pub entry_len: u16,
    pub name_len: u8,
    pub in_: ext4_dir_en_internal,
    // name: [u8; 0] 柔性数组，需要通过不安全指针访问
}

/// HTree 索引计数/限制
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ext4_dir_idx_climit {
    pub limit: u16,
    pub count: u16,
}

/// HTree 索引点目录项
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ext4_dir_idx_dot_en {
    pub inode: u32,
    pub entry_length: u16,
    pub name_length: u8,
    pub inode_type: u8,
    pub name: [u8; 4],
}

/// HTree 根信息
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ext4_dir_idx_rinfo {
    pub reserved_zero: u32,
    pub hash_version: u8,
    pub info_length: u8,
    pub indirect_levels: u8,
    pub unused_flags: u8,
}

/// HTree 索引条目
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ext4_dir_idx_entry {
    pub hash: u32,
    pub block: u32,
}

/// HTree 索引根
#[repr(C, packed)]
pub struct ext4_dir_idx_root {
    pub dots: [ext4_dir_idx_dot_en; 2],
    pub info: ext4_dir_idx_rinfo,
    // en: [ext4_dir_idx_entry; 0] 柔性数组
}

/// 假目录项（用于 HTree 节点）
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ext4_fake_dir_entry {
    pub inode: u32,
    pub entry_length: u16,
    pub name_length: u8,
    pub inode_type: u8,
}

/// HTree 索引节点
#[repr(C, packed)]
pub struct ext4_dir_idx_node {
    pub fake: ext4_fake_dir_entry,
    // entries: [ext4_dir_idx_entry; 0] 柔性数组
}

/// HTree 块尾部
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ext4_dir_idx_tail {
    pub reserved: u32,
    pub checksum: u32,
}

/// 目录项尾部（用于校验和）
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ext4_dir_entry_tail {
    pub reserved_zero1: u32,
    pub rec_len: u16,
    pub reserved_zero2: u8,
    pub reserved_ft: u8,
    pub checksum: u32,
}

// ============================================================================
// 测试
// ============================================================================
#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn test_superblock_size() {
        assert_eq!(size_of::<ext4_sblock>(), 1024, "超级块大小必须是 1024 字节");
    }

    #[test]
    fn test_bgroup_size() {
        assert_eq!(size_of::<ext4_bgroup>(), 64, "块组描述符大小必须是 64 字节");
    }

    #[test]
    fn test_inode_size() {
        let size = size_of::<ext4_inode>();

        // EXT4 Inode 的标准大小：
        // - 128 字节：EXT2 兼容（仅到 osd2）
        // - 156 字节：EXT4 扩展（包含额外时间戳等）
        // - 160 字节：某些实现会填充到 160
        // - 256 字节：某些实现使用更大的 Inode

        assert_eq!(size, 156, "EXT4 扩展 Inode 大小应该是 156 字节");
    }

    #[test]
    fn test_alignment() {
        // packed 结构体应该是 1 字节对齐
        assert_eq!(align_of::<ext4_sblock>(), 1);
        assert_eq!(align_of::<ext4_bgroup>(), 1);
        assert_eq!(align_of::<ext4_inode>(), 1);
    }

    #[test]
    fn test_inode_field_offsets() {
        // 验证关键字段的偏移量
        use core::mem::offset_of;
        assert_eq!(offset_of!(ext4_inode, mode), 0);
        assert_eq!(offset_of!(ext4_inode, blocks), 40);
        assert_eq!(offset_of!(ext4_inode, generation), 100);
        assert_eq!(offset_of!(ext4_inode, osd2), 116);
        assert_eq!(offset_of!(ext4_inode, extra_isize), 128);
    }

    #[test]
    fn test_dir_entry_sizes() {
        // 目录项最小大小（不含 name）
        assert_eq!(size_of::<ext4_fake_dir_entry>(), 8);

        // HTree 结构
        assert_eq!(size_of::<ext4_dir_idx_climit>(), 4);
        assert_eq!(size_of::<ext4_dir_idx_dot_en>(), 12);
        assert_eq!(size_of::<ext4_dir_idx_rinfo>(), 8);
        assert_eq!(size_of::<ext4_dir_idx_entry>(), 8);
        assert_eq!(size_of::<ext4_dir_idx_tail>(), 8);
        assert_eq!(size_of::<ext4_dir_entry_tail>(), 12);
    }

    #[test]
    fn test_constants() {
        assert_eq!(EXT4_SUPERBLOCK_MAGIC, 0xEF53);
        assert_eq!(EXT4_SUPERBLOCK_SIZE, 1024);
        assert_eq!(EXT4_INODE_BLOCKS, 15);
        assert_eq!(EXT4_INODE_DIRECT_BLOCK_COUNT, 12);
        assert_eq!(UUID_SIZE, 16);
    }
}
