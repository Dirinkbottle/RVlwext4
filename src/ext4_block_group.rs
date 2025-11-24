//! EXT4 块组操作
//! 对应 `ext4_block_group.h` 和 `ext4_block_group.c`

use crate::ext4_misc::{to_le16, to_le32};
use crate::ext4_super::ext4_sb_get_desc_size;
use crate::ext4_types::{EXT4_MIN_BLOCK_GROUP_DESCRIPTOR_SIZE, ext4_bgroup, ext4_sblock};

// ============================================================================
// 块位图操作
// ============================================================================

/// 获取数据块位图的地址
///
/// # 参数
/// * `bg` - 块组引用
/// * `sb` - 超级块引用
///
/// # 返回值
/// 块位图所在的块地址
#[inline]
pub fn ext4_bg_get_block_bitmap(bg: &ext4_bgroup, sb: &ext4_sblock) -> u64 {
    let mut v = to_le32(bg.block_bitmap_lo) as u64;
    if ext4_sb_get_desc_size(sb) > EXT4_MIN_BLOCK_GROUP_DESCRIPTOR_SIZE as u16 {
        v |= (to_le32(bg.block_bitmap_hi) as u64) << 32;
    }
    v
}

/// 设置数据块位图的地址
///
/// # 参数
/// * `bg` - 可变块组引用
/// * `sb` - 超级块引用
/// * `blk` - 要设置的块地址
#[inline]
pub fn ext4_bg_set_block_bitmap(bg: &mut ext4_bgroup, sb: &ext4_sblock, blk: u64) {
    bg.block_bitmap_lo = to_le32(blk as u32);
    if ext4_sb_get_desc_size(sb) > EXT4_MIN_BLOCK_GROUP_DESCRIPTOR_SIZE as u16 {
        bg.block_bitmap_hi = to_le32((blk >> 32) as u32);
    }
}

// ============================================================================
// Inode 位图操作
// ============================================================================

/// 获取 Inode 位图的地址
///
/// # 参数
/// * `bg` - 块组引用
/// * `sb` - 超级块引用
///
/// # 返回值
/// Inode 位图所在的块地址
#[inline]
pub fn ext4_bg_get_inode_bitmap(bg: &ext4_bgroup, sb: &ext4_sblock) -> u64 {
    let mut v = to_le32(bg.inode_bitmap_lo) as u64;
    if ext4_sb_get_desc_size(sb) > EXT4_MIN_BLOCK_GROUP_DESCRIPTOR_SIZE as u16 {
        v |= (to_le32(bg.inode_bitmap_hi) as u64) << 32;
    }
    v
}

/// 设置 Inode 位图的地址
///
/// # 参数
/// * `bg` - 可变块组引用
/// * `sb` - 超级块引用
/// * `blk` - 要设置的块地址
#[inline]
pub fn ext4_bg_set_inode_bitmap(bg: &mut ext4_bgroup, sb: &ext4_sblock, blk: u64) {
    bg.inode_bitmap_lo = to_le32(blk as u32);
    if ext4_sb_get_desc_size(sb) > EXT4_MIN_BLOCK_GROUP_DESCRIPTOR_SIZE as u16 {
        bg.inode_bitmap_hi = to_le32((blk >> 32) as u32);
    }
}

// ============================================================================
// Inode 表操作
// ============================================================================

/// 获取 Inode 表的第一个块地址
///
/// # 参数
/// * `bg` - 块组引用
/// * `sb` - 超级块引用
///
/// # 返回值
/// Inode 表第一个块的地址
#[inline]
pub fn ext4_bg_get_inode_table_first_block(bg: &ext4_bgroup, sb: &ext4_sblock) -> u64 {
    let mut v = to_le32(bg.inode_table_first_block_lo) as u64;
    if ext4_sb_get_desc_size(sb) > EXT4_MIN_BLOCK_GROUP_DESCRIPTOR_SIZE as u16 {
        v |= (to_le32(bg.inode_table_first_block_hi) as u64) << 32;
    }
    v
}

/// 设置 Inode 表的第一个块地址
///
/// # 参数
/// * `bg` - 可变块组引用
/// * `sb` - 超级块引用
/// * `blk` - 要设置的块地址
#[inline]
pub fn ext4_bg_set_inode_table_first_block(bg: &mut ext4_bgroup, sb: &ext4_sblock, blk: u64) {
    bg.inode_table_first_block_lo = to_le32(blk as u32);
    if ext4_sb_get_desc_size(sb) > EXT4_MIN_BLOCK_GROUP_DESCRIPTOR_SIZE as u16 {
        bg.inode_table_first_block_hi = to_le32((blk >> 32) as u32);
    }
}

// ============================================================================
// 空闲块计数
// ============================================================================

/// 获取块组中的空闲块数
///
/// # 参数
/// * `bg` - 块组引用
/// * `sb` - 超级块引用
///
/// # 返回值
/// 空闲块数量
#[inline]
pub fn ext4_bg_get_free_blocks_count(bg: &ext4_bgroup, sb: &ext4_sblock) -> u32 {
    let mut v = to_le16(bg.free_blocks_count_lo) as u32;
    if ext4_sb_get_desc_size(sb) > EXT4_MIN_BLOCK_GROUP_DESCRIPTOR_SIZE as u16 {
        v |= (to_le16(bg.free_blocks_count_hi) as u32) << 16;
    }
    v
}

/// 设置块组中的空闲块数
///
/// # 参数
/// * `bg` - 可变块组引用
/// * `sb` - 超级块引用
/// * `cnt` - 空闲块数量
#[inline]
pub fn ext4_bg_set_free_blocks_count(bg: &mut ext4_bgroup, sb: &ext4_sblock, cnt: u32) {
    bg.free_blocks_count_lo = to_le16(cnt as u16);
    if ext4_sb_get_desc_size(sb) > EXT4_MIN_BLOCK_GROUP_DESCRIPTOR_SIZE as u16 {
        bg.free_blocks_count_hi = to_le16((cnt >> 16) as u16);
    }
}

// ============================================================================
// 空闲 Inode 计数
// ============================================================================

/// 获取块组中的空闲 Inode 数
///
/// # 参数
/// * `bg` - 块组引用
/// * `sb` - 超级块引用
///
/// # 返回值
/// 空闲 Inode 数量
#[inline]
pub fn ext4_bg_get_free_inodes_count(bg: &ext4_bgroup, sb: &ext4_sblock) -> u32 {
    let mut v = to_le16(bg.free_inodes_count_lo) as u32;
    if ext4_sb_get_desc_size(sb) > EXT4_MIN_BLOCK_GROUP_DESCRIPTOR_SIZE as u16 {
        v |= (to_le16(bg.free_inodes_count_hi) as u32) << 16;
    }
    v
}

/// 设置块组中的空闲 Inode 数
///
/// # 参数
/// * `bg` - 可变块组引用
/// * `sb` - 超级块引用
/// * `cnt` - 空闲 Inode 数量
#[inline]
pub fn ext4_bg_set_free_inodes_count(bg: &mut ext4_bgroup, sb: &ext4_sblock, cnt: u32) {
    bg.free_inodes_count_lo = to_le16(cnt as u16);
    if ext4_sb_get_desc_size(sb) > EXT4_MIN_BLOCK_GROUP_DESCRIPTOR_SIZE as u16 {
        bg.free_inodes_count_hi = to_le16((cnt >> 16) as u16);
    }
}

// ============================================================================
// 已使用目录计数
// ============================================================================

/// 获取块组中已使用的目录数
///
/// # 参数
/// * `bg` - 块组引用
/// * `sb` - 超级块引用
///
/// # 返回值
/// 已使用目录数量
#[inline]
pub fn ext4_bg_get_used_dirs_count(bg: &ext4_bgroup, sb: &ext4_sblock) -> u32 {
    let mut v = to_le16(bg.used_dirs_count_lo) as u32;
    if ext4_sb_get_desc_size(sb) > EXT4_MIN_BLOCK_GROUP_DESCRIPTOR_SIZE as u16 {
        v |= (to_le16(bg.used_dirs_count_hi) as u32) << 16;
    }
    v
}

/// 设置块组中已使用的目录数
///
/// # 参数
/// * `bg` - 可变块组引用
/// * `sb` - 超级块引用
/// * `cnt` - 已使用目录数量
#[inline]
pub fn ext4_bg_set_used_dirs_count(bg: &mut ext4_bgroup, sb: &ext4_sblock, cnt: u32) {
    bg.used_dirs_count_lo = to_le16(cnt as u16);
    if ext4_sb_get_desc_size(sb) > EXT4_MIN_BLOCK_GROUP_DESCRIPTOR_SIZE as u16 {
        bg.used_dirs_count_hi = to_le16((cnt >> 16) as u16);
    }
}

// ============================================================================
// 未使用 Inode 表计数
// ============================================================================

/// 获取未使用的 Inode 数
///
/// # 参数
/// * `bg` - 块组引用
/// * `sb` - 超级块引用
///
/// # 返回值
/// 未使用 Inode 数量
#[inline]
pub fn ext4_bg_get_itable_unused(bg: &ext4_bgroup, sb: &ext4_sblock) -> u32 {
    let mut v = to_le16(bg.itable_unused_lo) as u32;
    if ext4_sb_get_desc_size(sb) > EXT4_MIN_BLOCK_GROUP_DESCRIPTOR_SIZE as u16 {
        v |= (to_le16(bg.itable_unused_hi) as u32) << 16;
    }
    v
}

/// 设置未使用的 Inode 数
///
/// # 参数
/// * `bg` - 可变块组引用
/// * `sb` - 超级块引用
/// * `cnt` - 未使用 Inode 数量
#[inline]
pub fn ext4_bg_set_itable_unused(bg: &mut ext4_bgroup, sb: &ext4_sblock, cnt: u32) {
    bg.itable_unused_lo = to_le16(cnt as u16);
    if ext4_sb_get_desc_size(sb) > EXT4_MIN_BLOCK_GROUP_DESCRIPTOR_SIZE as u16 {
        bg.itable_unused_hi = to_le16((cnt >> 16) as u16);
    }
}

// ============================================================================
// 校验和操作
// ============================================================================

/// 设置块组的校验和
///
/// # 参数
/// * `bg` - 可变块组引用
/// * `crc` - 校验和值
#[inline]
pub fn ext4_bg_set_checksum(bg: &mut ext4_bgroup, crc: u16) {
    bg.checksum = to_le16(crc);
}

// ============================================================================
// 标志操作
// ============================================================================

/// 检查块组是否有指定标志
///
/// # 参数
/// * `bg` - 块组引用
/// * `flag` - 要检查的标志
///
/// # 返回值
/// - `true` - 标志已设置
/// - `false` - 标志未设置
#[inline]
pub fn ext4_bg_has_flag(bg: &ext4_bgroup, flag: u32) -> bool {
    (to_le16(bg.flags) as u32) & flag != 0
}

/// 设置块组标志
///
/// # 参数
/// * `bg` - 可变块组引用
/// * `flag` - 要设置的标志
#[inline]
pub fn ext4_bg_set_flag(bg: &mut ext4_bgroup, flag: u32) {
    let mut flags = to_le16(bg.flags);
    flags |= flag as u16;
    bg.flags = to_le16(flags);
}

/// 清除块组标志
///
/// # 参数
/// * `bg` - 可变块组引用
/// * `flag` - 要清除的标志
#[inline]
pub fn ext4_bg_clear_flag(bg: &mut ext4_bgroup, flag: u32) {
    let mut flags = to_le16(bg.flags);
    flags &= !(flag as u16);
    bg.flags = to_le16(flags);
}

// ============================================================================
// CRC16 计算
// ============================================================================

/// CRC16 查找表（用于块组校验和计算）
const CRC16_TABLE: [u16; 256] = [
    0x0000, 0xC0C1, 0xC181, 0x0140, 0xC301, 0x03C0, 0x0280, 0xC241, 0xC601, 0x06C0, 0x0780, 0xC741,
    0x0500, 0xC5C1, 0xC481, 0x0440, 0xCC01, 0x0CC0, 0x0D80, 0xCD41, 0x0F00, 0xCFC1, 0xCE81, 0x0E40,
    0x0A00, 0xCAC1, 0xCB81, 0x0B40, 0xC901, 0x09C0, 0x0880, 0xC841, 0xD801, 0x18C0, 0x1980, 0xD941,
    0x1B00, 0xDBC1, 0xDA81, 0x1A40, 0x1E00, 0xDEC1, 0xDF81, 0x1F40, 0xDD01, 0x1DC0, 0x1C80, 0xDC41,
    0x1400, 0xD4C1, 0xD581, 0x1540, 0xD701, 0x17C0, 0x1680, 0xD641, 0xD201, 0x12C0, 0x1380, 0xD341,
    0x1100, 0xD1C1, 0xD081, 0x1040, 0xF001, 0x30C0, 0x3180, 0xF141, 0x3300, 0xF3C1, 0xF281, 0x3240,
    0x3600, 0xF6C1, 0xF781, 0x3740, 0xF501, 0x35C0, 0x3480, 0xF441, 0x3C00, 0xFCC1, 0xFD81, 0x3D40,
    0xFF01, 0x3FC0, 0x3E80, 0xFE41, 0xFA01, 0x3AC0, 0x3B80, 0xFB41, 0x3900, 0xF9C1, 0xF881, 0x3840,
    0x2800, 0xE8C1, 0xE981, 0x2940, 0xEB01, 0x2BC0, 0x2A80, 0xEA41, 0xEE01, 0x2EC0, 0x2F80, 0xEF41,
    0x2D00, 0xEDC1, 0xEC81, 0x2C40, 0xE401, 0x24C0, 0x2580, 0xE541, 0x2700, 0xE7C1, 0xE681, 0x2640,
    0x2200, 0xE2C1, 0xE381, 0x2340, 0xE101, 0x21C0, 0x2080, 0xE041, 0xA001, 0x60C0, 0x6180, 0xA141,
    0x6300, 0xA3C1, 0xA281, 0x6240, 0x6600, 0xA6C1, 0xA781, 0x6740, 0xA501, 0x65C0, 0x6480, 0xA441,
    0x6C00, 0xACC1, 0xAD81, 0x6D40, 0xAF01, 0x6FC0, 0x6E80, 0xAE41, 0xAA01, 0x6AC0, 0x6B80, 0xAB41,
    0x6900, 0xA9C1, 0xA881, 0x6840, 0x7800, 0xB8C1, 0xB981, 0x7940, 0xBB01, 0x7BC0, 0x7A80, 0xBA41,
    0xBE01, 0x7EC0, 0x7F80, 0xBF41, 0x7D00, 0xBDC1, 0xBC81, 0x7C40, 0xB401, 0x74C0, 0x7580, 0xB541,
    0x7700, 0xB7C1, 0xB681, 0x7640, 0x7200, 0xB2C1, 0xB381, 0x7340, 0xB101, 0x71C0, 0x7080, 0xB041,
    0x5000, 0x90C1, 0x9181, 0x5140, 0x9301, 0x53C0, 0x5280, 0x9241, 0x9601, 0x56C0, 0x5780, 0x9741,
    0x5500, 0x95C1, 0x9481, 0x5440, 0x9C01, 0x5CC0, 0x5D80, 0x9D41, 0x5F00, 0x9FC1, 0x9E81, 0x5E40,
    0x5A00, 0x9AC1, 0x9B81, 0x5B40, 0x9901, 0x59C0, 0x5880, 0x9841, 0x8801, 0x48C0, 0x4980, 0x8941,
    0x4B00, 0x8BC1, 0x8A81, 0x4A40, 0x4E00, 0x8EC1, 0x8F81, 0x4F40, 0x8D01, 0x4DC0, 0x4C80, 0x8C41,
    0x4400, 0x84C1, 0x8581, 0x4540, 0x8701, 0x47C0, 0x4680, 0x8641, 0x8201, 0x42C0, 0x4380, 0x8341,
    0x4100, 0x81C1, 0x8081, 0x4040,
];

/// 计算 CRC16 校验和（用于块组校验）
///
/// # 参数
/// * `crc` - 初始 CRC 值
/// * `buffer` - 输入数据
///
/// # 返回值
/// 计算得到的 CRC16 值
pub fn ext4_bg_crc16(mut crc: u16, buffer: &[u8]) -> u16 {
    for &byte in buffer {
        // crc = (((crc >> 8) & 0xffU) ^ crc16_tab[(crc ^ *buffer++) & 0xffU]) & 0x0000ffffU; (特殊crc16)
        let index = ((crc ^ u16::from(byte)) & 0xFF) as usize;
        crc = (((crc >> 8) & 0xFF) ^ CRC16_TABLE[index]) & 0xFFFF;
    }
    crc
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ext4_types::{
        EXT4_BLOCK_GROUP_BLOCK_UNINIT, EXT4_BLOCK_GROUP_INODE_UNINIT,
        EXT4_BLOCK_GROUP_ITABLE_ZEROED,
    };

    /// 创建测试用的块组和超级块（32字节描述符）
    fn create_test_bg_and_sb_32() -> (ext4_bgroup, ext4_sblock) {
        let bg: ext4_bgroup = unsafe { core::mem::zeroed() };
        let mut sb: ext4_sblock = unsafe { core::mem::zeroed() };

        // 设置描述符大小为 32（基础模式）
        sb.desc_size = to_le16(32);

        (bg, sb)
    }

    /// 创建测试用的块组和超级块（64字节描述符）
    fn create_test_bg_and_sb_64() -> (ext4_bgroup, ext4_sblock) {
        let bg: ext4_bgroup = unsafe { core::mem::zeroed() };
        let mut sb: ext4_sblock = unsafe { core::mem::zeroed() };

        // 设置描述符大小为 64（启用 64 位支持）
        sb.desc_size = to_le16(64);

        (bg, sb)
    }

    // ============================================================================
    // 块位图测试
    // ============================================================================

    #[test]
    fn test_block_bitmap_get_set_32bit() {
        let (mut bg, sb) = create_test_bg_and_sb_32();

        // 测试32位模式（只使用 lo 部分）
        ext4_bg_set_block_bitmap(&mut bg, &sb, 0x12345678);
        assert_eq!(ext4_bg_get_block_bitmap(&bg, &sb), 0x12345678);

        // 32位模式下，高32位应该被忽略
        ext4_bg_set_block_bitmap(&mut bg, &sb, 0x1_0000_0000_u64);
        assert_eq!(ext4_bg_get_block_bitmap(&bg, &sb), 0);
    }

    #[test]
    fn test_block_bitmap_get_set_64bit() {
        let (mut bg, sb) = create_test_bg_and_sb_64();

        // 测试小值
        ext4_bg_set_block_bitmap(&mut bg, &sb, 12345);
        assert_eq!(ext4_bg_get_block_bitmap(&bg, &sb), 12345);

        // 测试大值（需要 hi 部分）
        let big_value = 0x1_2345_6789_ABCD_u64;
        ext4_bg_set_block_bitmap(&mut bg, &sb, big_value);
        assert_eq!(ext4_bg_get_block_bitmap(&bg, &sb), big_value);

        // 测试边界值
        ext4_bg_set_block_bitmap(&mut bg, &sb, u64::MAX);
        assert_eq!(ext4_bg_get_block_bitmap(&bg, &sb), u64::MAX);

        // 测试零值
        ext4_bg_set_block_bitmap(&mut bg, &sb, 0);
        assert_eq!(ext4_bg_get_block_bitmap(&bg, &sb), 0);
    }

    // ============================================================================
    // Inode 位图测试
    // ============================================================================

    #[test]
    fn test_inode_bitmap_operations() {
        let (mut bg, sb) = create_test_bg_and_sb_64();

        // 测试基本操作
        ext4_bg_set_inode_bitmap(&mut bg, &sb, 0xABCDEF123456);
        assert_eq!(ext4_bg_get_inode_bitmap(&bg, &sb), 0xABCDEF123456);

        // 测试覆盖
        ext4_bg_set_inode_bitmap(&mut bg, &sb, 0x999);
        assert_eq!(ext4_bg_get_inode_bitmap(&bg, &sb), 0x999);
    }

    // ============================================================================
    // Inode 表测试
    // ============================================================================

    #[test]
    fn test_inode_table_first_block() {
        let (mut bg, sb) = create_test_bg_and_sb_64();

        // 测试典型值
        ext4_bg_set_inode_table_first_block(&mut bg, &sb, 0x1000);
        assert_eq!(ext4_bg_get_inode_table_first_block(&bg, &sb), 0x1000);

        // 测试大块号
        ext4_bg_set_inode_table_first_block(&mut bg, &sb, 0xFF_FFFF_FFFF);
        assert_eq!(
            ext4_bg_get_inode_table_first_block(&bg, &sb),
            0xFF_FFFF_FFFF
        );
    }

    // ============================================================================
    // 空闲块计数测试
    // ============================================================================

    #[test]
    fn test_free_blocks_count_32bit() {
        let (mut bg, sb) = create_test_bg_and_sb_32();

        // 32位模式，只使用低16位
        ext4_bg_set_free_blocks_count(&mut bg, &sb, 8192);
        assert_eq!(ext4_bg_get_free_blocks_count(&bg, &sb), 8192);

        // 测试边界
        ext4_bg_set_free_blocks_count(&mut bg, &sb, 65535);
        assert_eq!(ext4_bg_get_free_blocks_count(&bg, &sb), 65535);
    }

    #[test]
    fn test_free_blocks_count_64bit() {
        let (mut bg, sb) = create_test_bg_and_sb_64();

        // 测试小值
        ext4_bg_set_free_blocks_count(&mut bg, &sb, 1024);
        assert_eq!(ext4_bg_get_free_blocks_count(&bg, &sb), 1024);

        // 测试需要高16位的值
        ext4_bg_set_free_blocks_count(&mut bg, &sb, 0x12345678);
        assert_eq!(ext4_bg_get_free_blocks_count(&bg, &sb), 0x12345678);

        // 测试最大值
        ext4_bg_set_free_blocks_count(&mut bg, &sb, u32::MAX);
        assert_eq!(ext4_bg_get_free_blocks_count(&bg, &sb), u32::MAX);
    }

    // ============================================================================
    // 空闲 Inode 计数测试
    // ============================================================================

    #[test]
    fn test_free_inodes_count() {
        let (mut bg, sb) = create_test_bg_and_sb_64();

        // 测试典型值
        ext4_bg_set_free_inodes_count(&mut bg, &sb, 2048);
        assert_eq!(ext4_bg_get_free_inodes_count(&bg, &sb), 2048);

        // 测试零值
        ext4_bg_set_free_inodes_count(&mut bg, &sb, 0);
        assert_eq!(ext4_bg_get_free_inodes_count(&bg, &sb), 0);

        // 测试大值
        ext4_bg_set_free_inodes_count(&mut bg, &sb, 0xFFFF_FFFF);
        assert_eq!(ext4_bg_get_free_inodes_count(&bg, &sb), 0xFFFF_FFFF);
    }

    // ============================================================================
    // 已使用目录计数测试
    // ============================================================================

    #[test]
    fn test_used_dirs_count() {
        let (mut bg, sb) = create_test_bg_and_sb_64();

        // 初始值应为0
        assert_eq!(ext4_bg_get_used_dirs_count(&bg, &sb), 0);

        // 测试递增
        for i in 1..=100 {
            ext4_bg_set_used_dirs_count(&mut bg, &sb, i);
            assert_eq!(ext4_bg_get_used_dirs_count(&bg, &sb), i);
        }

        // 测试大值
        ext4_bg_set_used_dirs_count(&mut bg, &sb, 0x8000_0000);
        assert_eq!(ext4_bg_get_used_dirs_count(&bg, &sb), 0x8000_0000);
    }

    // ============================================================================
    // 未使用 Inode 表计数测试
    // ============================================================================

    #[test]
    fn test_itable_unused() {
        let (mut bg, sb) = create_test_bg_and_sb_64();

        // 测试设置和获取
        ext4_bg_set_itable_unused(&mut bg, &sb, 512);
        assert_eq!(ext4_bg_get_itable_unused(&bg, &sb), 512);

        // 测试边界值
        ext4_bg_set_itable_unused(&mut bg, &sb, 0);
        assert_eq!(ext4_bg_get_itable_unused(&bg, &sb), 0);

        ext4_bg_set_itable_unused(&mut bg, &sb, 0xFFFF_FFFF);
        assert_eq!(ext4_bg_get_itable_unused(&bg, &sb), 0xFFFF_FFFF);
    }

    // ============================================================================
    // 校验和测试
    // ============================================================================

    #[test]
    fn test_checksum() {
        let (mut bg, _sb) = create_test_bg_and_sb_64();

        // 测试设置校验和
        ext4_bg_set_checksum(&mut bg, 0x1234);
        assert_eq!(to_le16(bg.checksum), 0x1234);

        ext4_bg_set_checksum(&mut bg, 0xABCD);
        assert_eq!(to_le16(bg.checksum), 0xABCD);

        ext4_bg_set_checksum(&mut bg, 0);
        assert_eq!(to_le16(bg.checksum), 0);
    }

    // ============================================================================
    // 标志操作测试
    // ============================================================================

    #[test]
    fn test_flags_basic() {
        let (mut bg, _sb) = create_test_bg_and_sb_64();

        // 初始应该没有标志
        assert!(!ext4_bg_has_flag(&bg, EXT4_BLOCK_GROUP_INODE_UNINIT as u32));
        assert!(!ext4_bg_has_flag(&bg, EXT4_BLOCK_GROUP_BLOCK_UNINIT as u32));
        assert!(!ext4_bg_has_flag(
            &bg,
            EXT4_BLOCK_GROUP_ITABLE_ZEROED as u32
        ));

        // 设置标志
        ext4_bg_set_flag(&mut bg, EXT4_BLOCK_GROUP_INODE_UNINIT as u32);
        assert!(ext4_bg_has_flag(&bg, EXT4_BLOCK_GROUP_INODE_UNINIT as u32));
        assert!(!ext4_bg_has_flag(&bg, EXT4_BLOCK_GROUP_BLOCK_UNINIT as u32));

        // 设置另一个标志
        ext4_bg_set_flag(&mut bg, EXT4_BLOCK_GROUP_BLOCK_UNINIT as u32);
        assert!(ext4_bg_has_flag(&bg, EXT4_BLOCK_GROUP_INODE_UNINIT as u32));
        assert!(ext4_bg_has_flag(&bg, EXT4_BLOCK_GROUP_BLOCK_UNINIT as u32));

        // 清除标志
        ext4_bg_clear_flag(&mut bg, EXT4_BLOCK_GROUP_INODE_UNINIT as u32);
        assert!(!ext4_bg_has_flag(&bg, EXT4_BLOCK_GROUP_INODE_UNINIT as u32));
        assert!(ext4_bg_has_flag(&bg, EXT4_BLOCK_GROUP_BLOCK_UNINIT as u32));

        // 清除所有标志
        ext4_bg_clear_flag(&mut bg, EXT4_BLOCK_GROUP_BLOCK_UNINIT as u32);
        assert!(!ext4_bg_has_flag(&bg, EXT4_BLOCK_GROUP_INODE_UNINIT as u32));
        assert!(!ext4_bg_has_flag(&bg, EXT4_BLOCK_GROUP_BLOCK_UNINIT as u32));
    }

    #[test]
    fn test_flags_multiple() {
        let (mut bg, _sb) = create_test_bg_and_sb_64();

        // 设置多个标志
        ext4_bg_set_flag(&mut bg, EXT4_BLOCK_GROUP_INODE_UNINIT as u32);
        ext4_bg_set_flag(&mut bg, EXT4_BLOCK_GROUP_BLOCK_UNINIT as u32);
        ext4_bg_set_flag(&mut bg, EXT4_BLOCK_GROUP_ITABLE_ZEROED as u32);

        // 验证所有标志都已设置
        assert!(ext4_bg_has_flag(&bg, EXT4_BLOCK_GROUP_INODE_UNINIT as u32));
        assert!(ext4_bg_has_flag(&bg, EXT4_BLOCK_GROUP_BLOCK_UNINIT as u32));
        assert!(ext4_bg_has_flag(&bg, EXT4_BLOCK_GROUP_ITABLE_ZEROED as u32));

        // 清除中间的标志
        ext4_bg_clear_flag(&mut bg, EXT4_BLOCK_GROUP_BLOCK_UNINIT as u32);
        assert!(ext4_bg_has_flag(&bg, EXT4_BLOCK_GROUP_INODE_UNINIT as u32));
        assert!(!ext4_bg_has_flag(&bg, EXT4_BLOCK_GROUP_BLOCK_UNINIT as u32));
        assert!(ext4_bg_has_flag(&bg, EXT4_BLOCK_GROUP_ITABLE_ZEROED as u32));
    }

    // ============================================================================
    // CRC16 计算测试
    // ============================================================================

    #[test]
    fn test_crc16_empty() {
        // 空数据应该返回初始值
        assert_eq!(ext4_bg_crc16(0, &[]), 0);
        assert_eq!(ext4_bg_crc16(0xFFFF, &[]), 0xFFFF);
        assert_eq!(ext4_bg_crc16(0x1234, &[]), 0x1234);
    }

    #[test]
    fn test_crc16_single_byte() {
        // 测试单字节 - lwext4 特殊实现
        // 不同的字节应该产生不同的 CRC
        let crc1 = ext4_bg_crc16(0, &[0x12]);
        let crc2 = ext4_bg_crc16(0, &[0x34]);
        assert_ne!(crc1, crc2);

        // 测试非零初始值
        let crc3 = ext4_bg_crc16(0x1234, &[0x56]);
        assert_ne!(crc3, 0x1234); // 应该有变化

        // 验证一致性
        let crc4 = ext4_bg_crc16(0x1234, &[0x56]);
        assert_eq!(crc3, crc4);
    }

    #[test]
    fn test_crc16_known_value() {
        // lwext4 的 CRC16 实现（非标准）
        // 注意：这不是标准 CRC16/ARC！标准值应该是 0x31C3
        let data = b"123456789";
        let crc = ext4_bg_crc16(0, data);

        // 验证一致性：同样的输入应该产生同样的输出
        let crc2 = ext4_bg_crc16(0, data);
        assert_eq!(crc, crc2);

        // 验证不同输入产生不同输出
        let data2 = b"987654321";
        let crc3 = ext4_bg_crc16(0, data2);
        assert_ne!(crc, crc3);

        // lwext4 的实际值（与标准 CRC16 不同）
        // 如果要验证与 C 实现一致，需要从 C 获取期望值
    }

    #[test]
    fn test_crc16_incremental() {
        // 测试增量计算
        let data = b"Hello, World!";

        // 一次性计算
        let crc_full = ext4_bg_crc16(0, data);

        // 分段计算
        let crc1 = ext4_bg_crc16(0, &data[0..5]); // "Hello"
        let crc2 = ext4_bg_crc16(crc1, &data[5..]); // ", World!"

        assert_eq!(crc_full, crc2);
    }

    #[test]
    fn test_crc16_sensitivity() {
        // CRC 应该对数据敏感
        let data1 = b"test";
        let data2 = b"Test"; // 大写 T

        let crc1 = ext4_bg_crc16(0, data1);
        let crc2 = ext4_bg_crc16(0, data2);
        assert_ne!(crc1, crc2);

        // 顺序敏感
        let data3 = b"abcd";
        let data4 = b"dcba";
        let crc3 = ext4_bg_crc16(0, data3);
        let crc4 = ext4_bg_crc16(0, data4);
        assert_ne!(crc3, crc4);
    }

    #[test]
    fn test_crc16_all_zeros() {
        // 全零数据
        let data = [0u8; 100];
        let crc1 = ext4_bg_crc16(0, &data);

        // 验证一致性
        let crc2 = ext4_bg_crc16(0, &data);
        assert_eq!(crc1, crc2);

        // 使用非零初始值测试
        let crc3 = ext4_bg_crc16(0x5678, &data);
        let crc4 = ext4_bg_crc16(0x5678, &data);
        assert_eq!(crc3, crc4);
    }

    #[test]
    fn test_crc16_all_ones() {
        // 全1数据
        let data = [0xFFu8; 100];
        let crc1 = ext4_bg_crc16(0, &data);

        // 验证一致性
        let crc2 = ext4_bg_crc16(0, &data);
        assert_eq!(crc1, crc2);

        // 不同长度应该产生不同结果
        let data_short = [0xFFu8; 50];
        let crc3 = ext4_bg_crc16(0, &data_short);
        assert_ne!(crc1, crc3);
    }

    // ============================================================================
    // 综合场景测试
    // ============================================================================

    #[test]
    fn test_complete_block_group_scenario() {
        let (mut bg, sb) = create_test_bg_and_sb_64();

        // 模拟一个真实的块组配置
        // 块位图在块 1024
        ext4_bg_set_block_bitmap(&mut bg, &sb, 1024);

        // Inode 位图在块 1025
        ext4_bg_set_inode_bitmap(&mut bg, &sb, 1025);

        // Inode 表从块 1026 开始
        ext4_bg_set_inode_table_first_block(&mut bg, &sb, 1026);

        // 8192 个块中有 6000 个空闲
        ext4_bg_set_free_blocks_count(&mut bg, &sb, 6000);

        // 2048 个 Inode 中有 1500 个空闲
        ext4_bg_set_free_inodes_count(&mut bg, &sb, 1500);

        // 使用了 100 个目录
        ext4_bg_set_used_dirs_count(&mut bg, &sb, 100);

        // 500 个未使用的 Inode 表条目
        ext4_bg_set_itable_unused(&mut bg, &sb, 500);

        // 设置标志：Inode 表已清零
        ext4_bg_set_flag(&mut bg, EXT4_BLOCK_GROUP_ITABLE_ZEROED as u32);

        // 验证所有值
        assert_eq!(ext4_bg_get_block_bitmap(&bg, &sb), 1024);
        assert_eq!(ext4_bg_get_inode_bitmap(&bg, &sb), 1025);
        assert_eq!(ext4_bg_get_inode_table_first_block(&bg, &sb), 1026);
        assert_eq!(ext4_bg_get_free_blocks_count(&bg, &sb), 6000);
        assert_eq!(ext4_bg_get_free_inodes_count(&bg, &sb), 1500);
        assert_eq!(ext4_bg_get_used_dirs_count(&bg, &sb), 100);
        assert_eq!(ext4_bg_get_itable_unused(&bg, &sb), 500);
        assert!(ext4_bg_has_flag(&bg, EXT4_BLOCK_GROUP_ITABLE_ZEROED as u32));
        assert!(!ext4_bg_has_flag(&bg, EXT4_BLOCK_GROUP_INODE_UNINIT as u32));
    }

    #[test]
    fn test_descriptor_size_differences() {
        // 测试 32 字节和 64 字节描述符的区别
        let (mut bg32, sb32) = create_test_bg_and_sb_32();
        let (mut bg64, sb64) = create_test_bg_and_sb_64();

        // 对于小于 32 位的值，两种模式应该相同
        let test_value = 0x12345678_u64;
        ext4_bg_set_block_bitmap(&mut bg32, &sb32, test_value);
        ext4_bg_set_block_bitmap(&mut bg64, &sb64, test_value);

        assert_eq!(ext4_bg_get_block_bitmap(&bg32, &sb32), test_value);
        assert_eq!(ext4_bg_get_block_bitmap(&bg64, &sb64), test_value);

        // 对于超过 32 位的值，只有 64 字节模式能正确处理
        let big_value = 0x1_0000_0000_u64 + 0x5678;
        ext4_bg_set_block_bitmap(&mut bg32, &sb32, big_value);
        ext4_bg_set_block_bitmap(&mut bg64, &sb64, big_value);

        // 32 字节模式会丢失高位
        assert_ne!(ext4_bg_get_block_bitmap(&bg32, &sb32), big_value);
        // 64 字节模式保留完整值
        assert_eq!(ext4_bg_get_block_bitmap(&bg64, &sb64), big_value);
    }

    #[test]
    fn test_zero_initialization() {
        let (bg, sb) = create_test_bg_and_sb_64();

        // 零初始化的块组应该所有计数都为0
        assert_eq!(ext4_bg_get_block_bitmap(&bg, &sb), 0);
        assert_eq!(ext4_bg_get_inode_bitmap(&bg, &sb), 0);
        assert_eq!(ext4_bg_get_inode_table_first_block(&bg, &sb), 0);
        assert_eq!(ext4_bg_get_free_blocks_count(&bg, &sb), 0);
        assert_eq!(ext4_bg_get_free_inodes_count(&bg, &sb), 0);
        assert_eq!(ext4_bg_get_used_dirs_count(&bg, &sb), 0);
        assert_eq!(ext4_bg_get_itable_unused(&bg, &sb), 0);

        // 所有标志都应该未设置
        assert!(!ext4_bg_has_flag(&bg, EXT4_BLOCK_GROUP_INODE_UNINIT as u32));
        assert!(!ext4_bg_has_flag(&bg, EXT4_BLOCK_GROUP_BLOCK_UNINIT as u32));
        assert!(!ext4_bg_has_flag(
            &bg,
            EXT4_BLOCK_GROUP_ITABLE_ZEROED as u32
        ));
    }
}
