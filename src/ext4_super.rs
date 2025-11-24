//! EXT4 超级块操作
//! 对应 ext4_super.h 和 ext4_super.c

use crate::{
    ext4_crc32::struct_bytes_before_filed,
    ext4_misc::{to_le16, to_le32},
    ext4_types::{
        EXT4_CHECKSUM_CRC32C, EXT4_FINCOM_META_BG, EXT4_FRO_COM_METADATA_CSUM,
        EXT4_FRO_COM_SPARSE_SUPER, EXT4_MIN_BLOCK_GROUP_DESCRIPTOR_SIZE, ext4_sblock,
    },
};

/// 获取超级块中的总块数（64位）
///
/// 合并 `blocks_count_lo` 和 `blocks_count_hi` 两个字段
///
/// # 参数
/// * `sb` - 超级块引用
///
/// # 返回值
/// 64位总块数
#[inline]
pub fn ext4_sb_get_blocks_cnt(sb: &ext4_sblock) -> u64 {
    ((to_le32(sb.blocks_count_hi) as u64) << 32) | (to_le32(sb.blocks_count_lo) as u64)
}

/// 设置超级块中的总块数（64位）
///
/// 将64位值拆分为 `blocks_count_lo` 和 `blocks_count_hi`
///
/// # 参数
/// * `sb` - 可变超级块引用
/// * `cnt` - 要设置的块数
#[inline]
pub fn ext4_sb_set_blocks_cnt(sb: &mut ext4_sblock, cnt: u64) {
    sb.blocks_count_lo = to_le32(((cnt << 32) >> 32) as u32);
    sb.blocks_count_hi = to_le32((cnt >> 32) as u32);
}

/// 获取超级块中的空闲块数（64位）
///
/// 合并 `free_blocks_count_lo` 和 `free_blocks_count_hi`
///
/// # 参数
/// * `sb` - 超级块引用
///
/// # 返回值
/// 64位空闲块数
#[inline]
pub fn ext4_sb_get_free_blocks_cnt(sb: &mut ext4_sblock) -> u64 {
    ((to_le32(sb.free_blocks_count_hi) as u64) << 32)
        | ((to_le32(sb.free_blocks_count_lo) as u64) as u64)
}

/// 设置超级块中的空闲块数（64位）
///
/// # 参数
/// * `sb` - 可变超级块引用
/// * `cnt` - 要设置的空闲块数
#[inline]
pub fn ext4_sb_set_free_blocks_cnt(sb: &mut ext4_sblock, cnt: u64) {
    sb.free_blocks_count_lo = to_le32(((cnt << 32) >> 32) as u32);
    sb.free_blocks_count_hi = to_le32((cnt >> 32) as u32);
}

/// 获取块大小（字节）
///
/// 根据 `log_block_size` 计算：`块大小 = 1024 << log_block_size`
///
/// # 示例
/// - `log_block_size = 0` → 1024 字节 (1KB)
/// - `log_block_size = 2` → 4096 字节 (4KB)
///
/// # 参数
/// * `sb` - 超级块引用
#[inline]
pub fn ext4_sb_get_block_size(sb: &ext4_sblock) -> u32 {
    1024 << to_le32(sb.log_block_size)
}

/// 获取块组描述符大小
///
/// 如果小于最小值 32，返回 32 以保证兼容性
///
/// # 参数
/// * `sb` - 超级块引用
#[inline]
pub fn ext4_sb_get_desc_size(sb: &ext4_sblock) -> u16 {
    let des_size = to_le16(sb.desc_size);
    if des_size < EXT4_MIN_BLOCK_GROUP_DESCRIPTOR_SIZE as u16 {
        EXT4_MIN_BLOCK_GROUP_DESCRIPTOR_SIZE as u16
    } else {
        des_size
    }
}

// ============================================================================
// 特性和标志检查
// ============================================================================

/// 检查超级块标志是否设置
///
/// # 参数
/// * `sb` - 超级块引用
/// * `v` - 要检查的标志
#[inline]
pub fn ext4_sb_check_flag(sb: &ext4_sblock, v: u32) -> bool {
    to_le32(sb.flags) & v == v
}

/// 检查兼容特性（Compatible Feature）
///
/// 兼容特性：不支持此特性的驱动可以挂载文件系统
///
/// # 参数
/// * `sb` - 超级块引用
/// * `v` - 要检查的特性标志
#[inline]
pub fn ext4_sb_feature_com(sb: &ext4_sblock, v: u32) -> bool {
    to_le32(sb.features_compatible) & v == v
}

/// 检查不兼容特性（Incompatible Feature）
///
/// 不兼容特性：不支持此特性的驱动不能挂载文件系统
///
/// # 参数
/// * `sb` - 超级块引用
/// * `v` - 要检查的特性标志
#[inline]
pub fn ext4_sb_feature_incom(sb: &ext4_sblock, v: u32) -> bool {
    to_le32(sb.features_incompatible) & v == v
}

/// 检查只读兼容特性（Read-Only Compatible Feature）
///
/// 只读兼容特性：不支持此特性的驱动只能以只读方式挂载
///
/// # 参数
/// * `sb` - 超级块引用
/// * `v` - 要检查的特性标志
#[inline]
pub fn ext4_sb_feature_ro_com(sb: &ext4_sblock, v: u32) -> bool {
    to_le32(sb.features_read_only) & v == v
}

/// 将块组号转换为弹性块组号（Flex Block Group）
///
/// 弹性块组是多个连续块组的集合：
/// `flex_bg_id = block_group >> log_groups_per_flex`
///
/// # 参数
/// * `sb` - 超级块引用
/// * `block_group` - 块组号
#[inline]
pub fn ext4_sb_bg_to_flex(sb: &ext4_sblock, block_group: u32) -> u32 {
    block_group >> to_le32(sb.log_groups_per_flex as u32)
}

/// 获取弹性块组大小（包含多少个块组）
///
/// `flex_bg_size = 2^log_groups_per_flex`
///
/// # 参数
/// * `sb` - 超级块引用
#[inline]
pub fn ext4_sb_flex_bg_size(sb: &ext4_sblock) -> u32 {
    1 << to_le32(sb.log_groups_per_flex as u32)
}

/// 获取第一个元块组（Meta Block Group）ID
///
/// META_BG 特性用于支持更大的文件系统
///
/// # 参数
/// * `sb` - 超级块引用
#[inline]
pub fn ext4_sb_first_meta_bg(sb: &ext4_sblock) -> u32 {
    to_le32(sb.first_meta_bg)
}

/// 计算整个文件系统的块组总数
///
/// `块组数 = ceil(总块数 / 每组块数)`
///
/// # 参数
/// * `sb` - 超级块引用
pub fn ext4_block_group_cnt(sb: &ext4_sblock) -> u32 {
    let block_count = ext4_sb_get_blocks_cnt(sb); //文件系统里面的block总数
    let block_per_group = to_le32(sb.blocks_per_group) as u64;
    if block_count % block_per_group != 0 {
        (block_count / block_per_group + 1) as u32
    } else {
        (block_count / block_per_group) as u32
    }
}

/// 计算指定块组中的块数
///
/// 最后一个块组可能不满，需要特殊处理
///
/// # 参数
/// * `sb` - 超级块引用
/// * `bgid` - 块组ID
pub fn ext4_blocks_in_group_cnt(sb: &ext4_sblock, bgid: u32) -> u32 {
    let block_group_count = ext4_block_group_cnt(sb);
    let block_per_group = to_le32(sb.blocks_per_group);
    let total_blocks = ext4_sb_get_blocks_cnt(sb);
    if bgid < block_group_count - 1 {
        block_per_group
    } else {
        (total_blocks - ((block_group_count - 1) * block_per_group) as u64) as u32
    }
}

/// 计算指定块组中的 Inode 数
///
/// 最后一个块组可能不满，需要特殊处理
///
/// # 参数
/// * `sb` - 超级块引用
/// * `bgid` - 块组ID
pub fn ext4_inodes_in_group_cnt(sb: &ext4_sblock, bgid: u32) -> u32 {
    let block_group_count = ext4_block_group_cnt(sb);
    let inodes_per_group = to_le32(sb.inodes_per_group);
    let total_inode = to_le32(sb.inodes_count);

    if bgid < block_group_count - 1 {
        inodes_per_group
    } else {
        total_inode - ((block_group_count - 1) * inodes_per_group)
    }
}

// ============================================================================
// 辅助函数
// ============================================================================

/// 计算超级块的 CRC32C 校验和
///
/// 仅在启用 `CONFIG_META_CSUM_ENABLE` 特性时计算
/// 计算从超级块开始到 `checksum` 字段前的所有数据
///
/// # 参数
/// * `sb` - 超级块引用
pub fn ext4_sb_csum(sb: &ext4_sblock) -> u32 {
    if cfg!(feature = "CONFIG_META_CSUM_ENABLE") {
        use crate::{ext4_crc32::ext4_crc32c, ext4_types::EXT4_CRC32_INIT, offsetof};
        let filed_offset = offsetof!(ext4_sblock, checksum);
        let check_buffer: &[u8] = struct_bytes_before_filed(sb, filed_offset);
        ext4_crc32c(EXT4_CRC32_INIT, check_buffer)
    } else {
        0_u32
    }
}

/// 验证超级块的 CRC32C 校验和
///
/// 仅在超级块启用 METADATA_CSUM 特性时验证
///
/// # 参数
/// * `sb` - 超级块引用
pub fn ext4_sb_verify_csum(sb: &ext4_sblock) -> bool {
    if !ext4_sb_feature_ro_com(sb, EXT4_FRO_COM_METADATA_CSUM) {
        return true;
    }

    if to_le32(sb.checksum_type as u32) != EXT4_CHECKSUM_CRC32C as u32 {
        return false;
    }

    to_le32(sb.checksum) == ext4_sb_csum(sb)
}

/// 设置超级块的 CRC32C 校验和
///
/// 在写入超级块到磁盘前调用
///
/// # 参数
/// * `sb` - 可变超级块引用
pub fn ext4_sb_set_csum(sb: &mut ext4_sblock) {
    if !ext4_sb_feature_ro_com(sb, EXT4_FRO_COM_METADATA_CSUM) {
        return;
    }
    sb.checksum = to_le32(ext4_sb_csum(sb));
}

/// 判断 a 是否为 b 的幂
///
/// 用于稀疏超级块的判断
///
/// # 示例
/// - `is_power_of(27, 3)` → `true` (27 = 3³)
/// - `is_power_of(25, 5)` → `true` (25 = 5²)
///
/// # 参数
/// * `aa` - 被判断的数
/// * `bb` - 基数
#[inline]
pub fn is_power_of(aa: u32, bb: u32) -> bool {
    let (mut a, b) = (aa, bb);
    loop {
        if a < b {
            return false;
        }
        if a == b {
            return true;
        }
        if (a % b) != 0 {
            return false;
        }
        a /= b;
    }
}

/// 判断块组是否为稀疏超级块位置
///
/// 稀疏超级块只在特定块组存储备份：
/// - 块组 0, 1
/// - 3, 5, 7 的幂次块组 (3, 9, 27, ..., 5, 25, 125, ..., 7, 49, ...)
///
/// # 参数
/// * `group` - 块组号
pub fn ext4_sb_sparse(group: u32) -> bool {
    if group <= 1 {
        return true;
    }

    if group & 1 == 0 {
        return false;
    }

    is_power_of(group, 3) | is_power_of(group, 5) | is_power_of(group, 7)
}

/// 计算 META_BG 特性下的 GDT 块数
///
/// META_BG 将块组描述符分散存储在各个元块组
///
/// # 参数
/// * `sb` - 超级块引用
/// * `group` - 块组号
pub fn ext4_bg_num_gdb_meta(sb: &ext4_sblock, group: u32) -> u32 {
    let dsc_per_group = ext4_sb_get_block_size(sb) / ext4_sb_get_desc_size(sb) as u32;
    let metagroup = group / dsc_per_group;
    let first = metagroup * dsc_per_group;
    let last = first + dsc_per_group - 1;

    if group == first || group == first + 1 || group == last {
        return 1;
    }

    0
}

/// 判断指定块组是否包含超级块
///
/// 考虑稀疏超级块特性的影响
///
/// # 参数
/// * `sb` - 超级块引用
/// * `group` - 块组号
pub fn ext4_sb_is_super_in_bg(sb: &ext4_sblock, group: u32) -> bool {
    if ext4_sb_feature_ro_com(sb, EXT4_FRO_COM_SPARSE_SUPER) && !ext4_sb_sparse(group) {
        return false;
    }
    true
}

/// 计算非 META_BG 特性下的 GDT 块数
///
/// 传统方式：所有块组描述符集中存储在文件系统前面
///
/// # 参数
/// * `sb` - 超级块引用
/// * `group` - 块组号
pub fn ext4_bg_num_gdb_nometa(sb: &ext4_sblock, group: u32) -> u32 {
    // 1. 检查当前块组是否有超级块（稀疏超级块特性）
    if !ext4_sb_is_super_in_bg(sb, group) {
        return 0; // 这个块组没有超级块，也就不需要GDT
    }

    // 2. 计算每个块能存放的描述符数量
    let dsc_per_block = ext4_sb_get_block_size(sb) / ext4_sb_get_desc_size(sb) as u32;

    // 3. 计算存储所有块组描述符所需的总块数（向上取整）
    let total_groups = ext4_block_group_cnt(sb);
    let db_count = total_groups.div_ceil(dsc_per_block);

    // 4. 如果启用了META_BG特性，返回第一个元块组编号
    if ext4_sb_feature_incom(sb, EXT4_FINCOM_META_BG) {
        return ext4_sb_first_meta_bg(sb);
    }

    // 5. 传统模式：返回整个GDT需要的块数
    db_count
}

/// 计算块组的 GDT (Group Descriptor Table) 块数
///
/// 根据 META_BG 特性选择不同算法
///
/// # 参数
/// * `sb` - 超级块引用
/// * `group` - 块组号
pub fn ext4_bg_num_gdb(sb: &ext4_sblock, group: u32) -> u32 {
    let dsc_per_block = ext4_sb_get_block_size(sb) / ext4_sb_get_desc_size(sb) as u32;
    let first_meta_bg = ext4_sb_first_meta_bg(sb);
    let metagroup = group / dsc_per_block;

    if !ext4_sb_feature_incom(sb, EXT4_FINCOM_META_BG) || metagroup < first_meta_bg {
        return ext4_bg_num_gdb_nometa(sb, group);
    }

    ext4_bg_num_gdb_meta(sb, group)
}

/// 计算块组的基础元数据簇数
///
/// 包括：
/// - 超级块
/// - GDT (组描述符表)
/// - 保留 GDT 块
///
/// 最后转换为簇（Cluster）单位
///
/// # 参数
/// * `sb` - 超级块引用
/// * `block_group` - 块组号
pub fn ext4_num_base_meta_clusters(sb: &ext4_sblock, block_group: u32) -> u32 {
    let dsc_per_block = ext4_sb_get_block_size(sb) / ext4_sb_get_desc_size(sb) as u32;

    let mut num = if ext4_sb_is_super_in_bg(sb, block_group) {
        1
    } else {
        0
    };

    if !ext4_sb_feature_incom(sb, EXT4_FINCOM_META_BG)
        || block_group < ext4_sb_first_meta_bg(sb) * dsc_per_block
    {
        if num > 0 {
            num += ext4_bg_num_gdb(sb, block_group);
            num += to_le16(sb.s_reserved_gdt_blocks) as u32;
        }
    } else {
        num += ext4_bg_num_gdb(sb, block_group);
    }

    let clustersize = 1024_u32 << to_le32(sb.log_cluster_size);
    let cluster_ratio = clustersize / ext4_sb_get_block_size(sb);

    (num + cluster_ratio - 1) >> to_le32(sb.log_cluster_size)
}

/// 验证超级块的合法性
///
/// 检查：
/// - 魔数是否为 `0xEF53`
/// - 各种计数字段是否非零
/// - Inode 大小是否 >= 128
/// - 块组描述符大小是否在合法范围
/// - CRC32C 校验和是否正确
///
/// # 参数
/// * `sb` - 超级块引用
///
/// # 返回值
/// - `true` - 超级块有效
/// - `false` - 超级块无效
pub fn ext4_sb_check(sb: &ext4_sblock) -> bool {
    use crate::ext4_types::{
        EXT4_MAX_BLOCK_GROUP_DESCRIPTOR_SIZE, EXT4_MIN_BLOCK_GROUP_DESCRIPTOR_SIZE,
        EXT4_SUPERBLOCK_MAGIC,
    };

    if to_le16(sb.magic) != EXT4_SUPERBLOCK_MAGIC {
        return false;
    }

    if to_le32(sb.inodes_count) == 0 {
        return false;
    }

    if ext4_sb_get_blocks_cnt(sb) == 0 {
        return false;
    }

    if to_le32(sb.blocks_per_group) == 0 {
        return false;
    }

    if to_le32(sb.inodes_per_group) == 0 {
        return false;
    }

    if to_le16(sb.inode_size) < 128 {
        return false;
    }

    if to_le32(sb.first_inode) < 11 {
        return false;
    }

    if (ext4_sb_get_desc_size(sb) as usize) < EXT4_MIN_BLOCK_GROUP_DESCRIPTOR_SIZE {
        return false;
    }

    if (ext4_sb_get_desc_size(sb) as usize) > EXT4_MAX_BLOCK_GROUP_DESCRIPTOR_SIZE {
        return false;
    }

    if !ext4_sb_verify_csum(sb) {
        return false;
    }

    true
}

// TODO: 以下函数需要 ext4_blockdev 完整实现后才能完成
// 暂时注释掉，避免编译错误

// 写入超级块到块设备
//
// 先设置 CRC32C 校验和，再写入偏移 1024 处
//
// # 参数
// * `bdev` - 块设备引用
// * `sb` - 可变超级块引用
// pub fn ext4_sb_write(bdev: &mut ext4_blockdev, sb: &mut ext4_sblock) -> i32 {
// use crate::ext4_types::{EXT4_SUPERBLOCK_OFFSET, EXT4_SUPERBLOCK_SIZE};
//
// ext4_sb_set_csum(sb);
// ext4_block_writebytes(bdev, EXT4_SUPERBLOCK_OFFSET, sb, EXT4_SUPERBLOCK_SIZE)
// }
//
// 从块设备读取超级块
//
// 从偏移 1024 处读取 1024 字节
//
// # 参数
// * `bdev` - 块设备引用
// * `sb` - 可变超级块引用
// pub fn ext4_sb_read(bdev: &ext4_blockdev, sb: &mut ext4_sblock) -> i32 {
// use crate::ext4_types::{EXT4_SUPERBLOCK_OFFSET, EXT4_SUPERBLOCK_SIZE};
//
// ext4_block_readbytes(bdev, EXT4_SUPERBLOCK_OFFSET, sb, EXT4_SUPERBLOCK_SIZE)
// }

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use core::mem::size_of;

    use super::*;

    /// 创建一个测试用的超级块
    fn create_test_superblock() -> ext4_sblock {
        use crate::ext4_types::EXT4_SUPERBLOCK_MAGIC;

        let mut sb: ext4_sblock = unsafe { core::mem::zeroed() };

        // 设置基本字段
        sb.magic = EXT4_SUPERBLOCK_MAGIC;
        sb.blocks_count_lo = to_le32(1000000);
        sb.blocks_count_hi = to_le32(0);
        sb.free_blocks_count_lo = to_le32(500000);
        sb.free_blocks_count_hi = to_le32(0);
        sb.blocks_per_group = to_le32(8192);
        sb.inodes_per_group = to_le32(2048);
        sb.inodes_count = to_le32(250000);
        sb.log_block_size = to_le32(2); // 4KB 块
        sb.log_cluster_size = to_le32(2);
        sb.desc_size = to_le16(64);
        sb.inode_size = to_le16(256);
        sb.first_inode = to_le32(11);

        sb
    }

    #[test]
    fn test_ext4_sb_get_set_blocks_cnt() {
        let mut sb = create_test_superblock();

        // 测试获取
        let blocks = ext4_sb_get_blocks_cnt(&sb);
        assert_eq!(blocks, 1000000);

        // 测试设置小值
        ext4_sb_set_blocks_cnt(&mut sb, 2000000);
        assert_eq!(ext4_sb_get_blocks_cnt(&sb), 2000000);

        // 测试设置大值（超过32位）
        let big_value = 0x1_0000_0000_u64 + 12345;
        ext4_sb_set_blocks_cnt(&mut sb, big_value);
        assert_eq!(ext4_sb_get_blocks_cnt(&sb), big_value);

        // 验证高低位分离正确
        assert_eq!(to_le32(sb.blocks_count_lo), 12345);
        assert_eq!(to_le32(sb.blocks_count_hi), 1);
    }

    #[test]
    fn test_ext4_sb_get_set_free_blocks_cnt() {
        let mut sb = create_test_superblock();

        // 测试获取
        let free_blocks = ext4_sb_get_free_blocks_cnt(&mut sb);
        assert_eq!(free_blocks, 500000);

        // 测试设置
        ext4_sb_set_free_blocks_cnt(&mut sb, 750000);
        assert_eq!(ext4_sb_get_free_blocks_cnt(&mut sb), 750000);

        // 测试大值
        let big_value = 0x2_0000_0000_u64 + 54321;
        ext4_sb_set_free_blocks_cnt(&mut sb, big_value);
        assert_eq!(ext4_sb_get_free_blocks_cnt(&mut sb), big_value);
    }

    #[test]
    fn test_ext4_sb_get_block_size() {
        let mut sb = create_test_superblock();

        // log_block_size = 2 → 4KB
        assert_eq!(ext4_sb_get_block_size(&sb), 4096);

        // 测试其他大小
        sb.log_block_size = to_le32(0);
        assert_eq!(ext4_sb_get_block_size(&sb), 1024); // 1KB

        sb.log_block_size = to_le32(1);
        assert_eq!(ext4_sb_get_block_size(&sb), 2048); // 2KB

        sb.log_block_size = to_le32(3);
        assert_eq!(ext4_sb_get_block_size(&sb), 8192); // 8KB
    }

    #[test]
    fn test_ext4_sb_get_desc_size() {
        let mut sb = create_test_superblock();

        // 正常大小
        assert_eq!(ext4_sb_get_desc_size(&sb), 64);

        // 小于最小值，应该返回32
        sb.desc_size = to_le16(16);
        assert_eq!(ext4_sb_get_desc_size(&sb), 32);

        // 等于最小值
        sb.desc_size = to_le16(32);
        assert_eq!(ext4_sb_get_desc_size(&sb), 32);
    }

    #[test]
    fn test_ext4_sb_check_flag() {
        let mut sb = create_test_superblock();

        sb.flags = to_le32(0x0003); // 设置位0和位1

        assert!(ext4_sb_check_flag(&sb, 0x0001));
        assert!(ext4_sb_check_flag(&sb, 0x0002));
        assert!(ext4_sb_check_flag(&sb, 0x0003));
        assert!(!ext4_sb_check_flag(&sb, 0x0004));
    }

    #[test]
    fn test_ext4_sb_feature_checks() {
        let mut sb = create_test_superblock();

        // 测试兼容特性
        sb.features_compatible = to_le32(0x0008); // EXT4_FCOM_EXT_ATTR
        assert!(ext4_sb_feature_com(&sb, 0x0008));
        assert!(!ext4_sb_feature_com(&sb, 0x0001));

        // 测试不兼容特性
        sb.features_incompatible = to_le32(0x0040); // EXT4_FINCOM_EXTENTS
        assert!(ext4_sb_feature_incom(&sb, 0x0040));
        assert!(!ext4_sb_feature_incom(&sb, 0x0080));

        // 测试只读兼容特性
        sb.features_read_only = to_le32(0x0001); // EXT4_FRO_COM_SPARSE_SUPER
        assert!(ext4_sb_feature_ro_com(&sb, 0x0001));
        assert!(!ext4_sb_feature_ro_com(&sb, 0x0002));
    }

    #[test]
    fn test_ext4_sb_bg_to_flex() {
        let mut sb = create_test_superblock();

        sb.log_groups_per_flex = 4; // 2^4 = 16 块组per flex

        assert_eq!(ext4_sb_bg_to_flex(&sb, 0), 0);
        assert_eq!(ext4_sb_bg_to_flex(&sb, 15), 0);
        assert_eq!(ext4_sb_bg_to_flex(&sb, 16), 1);
        assert_eq!(ext4_sb_bg_to_flex(&sb, 31), 1);
        assert_eq!(ext4_sb_bg_to_flex(&sb, 32), 2);
    }

    #[test]
    fn test_ext4_sb_flex_bg_size() {
        let mut sb = create_test_superblock();

        sb.log_groups_per_flex = 0;
        assert_eq!(ext4_sb_flex_bg_size(&sb), 1);

        sb.log_groups_per_flex = 4;
        assert_eq!(ext4_sb_flex_bg_size(&sb), 16);

        sb.log_groups_per_flex = 5;
        assert_eq!(ext4_sb_flex_bg_size(&sb), 32);
    }

    #[test]
    fn test_ext4_block_group_cnt() {
        let mut sb = create_test_superblock();

        // 1000000 / 8192 = 122.07... → 123 块组
        assert_eq!(ext4_block_group_cnt(&sb), 123);

        // 恰好整除
        ext4_sb_set_blocks_cnt(&mut sb, 8192 * 100);
        assert_eq!(ext4_block_group_cnt(&sb), 100);

        // 加1块，应该多一个块组
        ext4_sb_set_blocks_cnt(&mut sb, 8192 * 100 + 1);
        assert_eq!(ext4_block_group_cnt(&sb), 101);
    }

    #[test]
    fn test_ext4_blocks_in_group_cnt() {
        let mut sb = create_test_superblock();

        // 前面的块组都是满的
        assert_eq!(ext4_blocks_in_group_cnt(&sb, 0), 8192);
        assert_eq!(ext4_blocks_in_group_cnt(&sb, 50), 8192);
        assert_eq!(ext4_blocks_in_group_cnt(&sb, 121), 8192);

        // 最后一个块组：1000000 - 122*8192 = 1000000 - 999424 = 576
        assert_eq!(ext4_blocks_in_group_cnt(&sb, 122), 576);
    }

    #[test]
    fn test_ext4_inodes_in_group_cnt() {
        let mut sb = create_test_superblock();

        // 250000 / 2048 = 122.07... → 123 块组
        // 前122个块组都是满的
        assert_eq!(ext4_inodes_in_group_cnt(&sb, 0), 2048);
        assert_eq!(ext4_inodes_in_group_cnt(&sb, 121), 2048);

        // 最后一个块组：250000 - 122*2048 = 250000 - 249856 = 144
        assert_eq!(ext4_inodes_in_group_cnt(&sb, 122), 144);
    }

    #[test]
    fn test_is_power_of() {
        // 3的幂
        assert!(is_power_of(3, 3));
        assert!(is_power_of(9, 3));
        assert!(is_power_of(27, 3));
        assert!(is_power_of(81, 3));

        // 5的幂
        assert!(is_power_of(5, 5));
        assert!(is_power_of(25, 5));
        assert!(is_power_of(125, 5));

        // 7的幂
        assert!(is_power_of(7, 7));
        assert!(is_power_of(49, 7));

        // 非幂
        assert!(!is_power_of(10, 3));
        assert!(!is_power_of(26, 5));
        assert!(!is_power_of(50, 7));
        assert!(!is_power_of(2, 3));
    }

    #[test]
    fn test_ext4_sb_sparse() {
        // 块组 0, 1 总是稀疏
        assert!(ext4_sb_sparse(0));
        assert!(ext4_sb_sparse(1));

        // 偶数块组（除了0）不是稀疏
        assert!(!ext4_sb_sparse(2));
        assert!(!ext4_sb_sparse(4));
        assert!(!ext4_sb_sparse(6));

        // 3, 5, 7的幂是稀疏
        assert!(ext4_sb_sparse(3));
        assert!(ext4_sb_sparse(5));
        assert!(ext4_sb_sparse(7));
        assert!(ext4_sb_sparse(9)); // 3²
        assert!(ext4_sb_sparse(25)); // 5²
        assert!(ext4_sb_sparse(27)); // 3³
        assert!(ext4_sb_sparse(49)); // 7²

        // 其他奇数不是稀疏
        assert!(!ext4_sb_sparse(11));
        assert!(!ext4_sb_sparse(13));
        assert!(!ext4_sb_sparse(15));
    }

    #[test]
    fn test_ext4_sb_is_super_in_bg() {
        let mut sb = create_test_superblock();

        // 没有稀疏超级块特性，所有块组都有超级块
        sb.features_read_only = to_le32(0);
        assert!(ext4_sb_is_super_in_bg(&sb, 0));
        assert!(ext4_sb_is_super_in_bg(&sb, 1));
        assert!(ext4_sb_is_super_in_bg(&sb, 2));
        assert!(ext4_sb_is_super_in_bg(&sb, 10));

        // 启用稀疏超级块特性
        sb.features_read_only = to_le32(EXT4_FRO_COM_SPARSE_SUPER);
        assert!(ext4_sb_is_super_in_bg(&sb, 0)); // 总是有
        assert!(ext4_sb_is_super_in_bg(&sb, 1)); // 总是有
        assert!(!ext4_sb_is_super_in_bg(&sb, 2)); // 偶数，没有
        assert!(ext4_sb_is_super_in_bg(&sb, 3)); // 3的幂
        assert!(!ext4_sb_is_super_in_bg(&sb, 4)); // 偶数
        assert!(ext4_sb_is_super_in_bg(&sb, 5)); // 5的幂
        assert!(ext4_sb_is_super_in_bg(&sb, 9)); // 3²
    }

    #[test]
    fn test_ext4_sb_check() {
        use crate::ext4_types::{EXT4_MIN_BLOCK_GROUP_DESCRIPTOR_SIZE, EXT4_SUPERBLOCK_MAGIC};

        let mut sb = create_test_superblock();

        // 正常的超级块应该通过验证
        assert!(ext4_sb_check(&sb));

        // 错误的魔数
        let original_magic = sb.magic;
        sb.magic = 0x0000;
        assert!(!ext4_sb_check(&sb));
        sb.magic = original_magic;

        // inodes_count 为 0
        let original_inodes = sb.inodes_count;
        sb.inodes_count = to_le32(0);
        assert!(!ext4_sb_check(&sb));
        sb.inodes_count = original_inodes;

        // blocks_count 为 0
        sb.blocks_count_lo = to_le32(0);
        sb.blocks_count_hi = to_le32(0);
        assert!(!ext4_sb_check(&sb));
        sb.blocks_count_lo = to_le32(1000000);

        // inode_size 小于 128
        let original_inode_size = sb.inode_size;
        sb.inode_size = to_le16(127);
        assert!(!ext4_sb_check(&sb));
        sb.inode_size = original_inode_size;

        // first_inode 小于 11
        let original_first_inode = sb.first_inode;
        sb.first_inode = to_le32(10);
        assert!(!ext4_sb_check(&sb));
        sb.first_inode = original_first_inode;
    }

    #[test]
    fn test_ext4_bg_num_gdb_meta() {
        let sb = create_test_superblock();

        // 块大小 4096, 描述符大小 64
        // 每块可存放: 4096 / 64 = 64 个描述符

        // 每个元组的第一个、第二个和最后一个块组有GDT
        assert_eq!(ext4_bg_num_gdb_meta(&sb, 0), 1); // 第一个
        assert_eq!(ext4_bg_num_gdb_meta(&sb, 1), 1); // 第二个
        assert_eq!(ext4_bg_num_gdb_meta(&sb, 63), 1); // 最后一个 (64-1)
        assert_eq!(ext4_bg_num_gdb_meta(&sb, 2), 0); // 中间的
        assert_eq!(ext4_bg_num_gdb_meta(&sb, 50), 0); // 中间的

        // 第二个元组
        assert_eq!(ext4_bg_num_gdb_meta(&sb, 64), 1); // 第一个
        assert_eq!(ext4_bg_num_gdb_meta(&sb, 65), 1); // 第二个
        assert_eq!(ext4_bg_num_gdb_meta(&sb, 127), 1); // 最后一个
        assert_eq!(ext4_bg_num_gdb_meta(&sb, 100), 0); // 中间的
    }

    #[test]
    fn test_ext4_num_base_meta_clusters() {
        let mut sb = create_test_superblock();

        // 测试1：没有启用稀疏超级块
        sb.features_read_only = to_le32(0);
        let clusters_0_full = ext4_num_base_meta_clusters(&sb, 0);
        let clusters_1_full = ext4_num_base_meta_clusters(&sb, 1);
        let clusters_2_full = ext4_num_base_meta_clusters(&sb, 2);

        // 在完全模式下，所有块组的元数据簇数应该相同（都有超级块）
        assert_eq!(clusters_0_full, clusters_1_full);
        assert_eq!(clusters_0_full, clusters_2_full);

        // 启用稀疏超级块特性
        sb.features_read_only = to_le32(EXT4_FRO_COM_SPARSE_SUPER);

        let clusters_0_sparse = ext4_num_base_meta_clusters(&sb, 0);
        let clusters_2_sparse = ext4_num_base_meta_clusters(&sb, 2);
        let clusters_3_sparse = ext4_num_base_meta_clusters(&sb, 3);

        // 块组0总是有超级块
        // 块组2（偶数，不是稀疏）没有超级块
        // 块组3（3的幂）有超级块

        // 块组2应该没有超级块，元数据应该为0
        assert_eq!(clusters_2_sparse, 0, "块组2在稀疏模式下不应该有超级块");

        // 块组0和块组3应该有元数据（可能相同也可能不同，取决于GDT分布）
        // 但块组0和3的元数据应该 >= 块组2
        assert!(clusters_0_sparse >= clusters_2_sparse);
        assert!(clusters_3_sparse >= clusters_2_sparse);
    }
}
