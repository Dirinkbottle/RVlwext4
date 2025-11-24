// EXT4_BITMAP module

#[inline(always)]
pub fn ext4_bmap_bit_set(bmap: &mut [u8], bit: u32) {
    let byte_offset = (bit >> 3) as usize;
    let bit_mask = 1 << (bit & 0x7) as u8;
    if byte_offset < bmap.len() {
        bmap[byte_offset] |= bit_mask;
    }
}

#[inline(always)]
pub fn ext4_bmap_bit_clr(bmap: &mut [u8], bit: u32) {
    let byte_offset = (bit >> 3) as usize;
    let bit_mask = !(1 << (bit & 0x7) as u8);
    if byte_offset < bmap.len() {
        bmap[byte_offset] &= bit_mask;
    }
}

#[inline(always)]
pub fn ext4_bmap_is_bit_set(bmap: &[u8], bit: u32) -> bool {
    let byte_offset = (bit >> 3) as usize;
    let bit_mask = 1 << (bit & 0x7) as u8;

    (bmap[byte_offset] & bit_mask) != 0
}

#[inline(always)]
pub fn ext4_bmap_is_bit_clr(bmap: &[u8], bit: u32) -> bool {
    let byte_offset = (bit >> 3) as usize;
    let bit_mask = 1 << (bit & 0x7) as u8;

    (bmap[byte_offset] & bit_mask) == 0
}

pub fn ext4_bmap_bits_free(bmap: &mut [u8], mut sbit: u32, mut bcnt: u32) {
    // 非对齐部分
    while bcnt > 0 && (sbit & 0x7) != 0 {
        ext4_bmap_bit_clr(bmap, sbit);
        sbit += 1;
        bcnt -= 1;
    }
    // 对齐部分
    while bcnt >= 8 {
        let byte_offset = (sbit >> 3) as usize;
        bmap[byte_offset] = 0;
        sbit += 8;
        bcnt -= 8;
    }

    // 处理剩余位
    for i in 0..bcnt {
        ext4_bmap_bit_clr(bmap, sbit + i);
    }
}

/// [sbit, ebit)
pub fn ext4_bmap_bit_find_clr(bmap: &[u8], sbit: u32, ebit: u32) -> Option<u32> {
    (sbit..ebit).find(|&bit| ext4_bmap_is_bit_clr(bmap, bit))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bmap_bit_set() {
        let mut bmap = [0u8; 4]; // 32 bits

        // 设置第 0 位
        ext4_bmap_bit_set(&mut bmap, 0);
        assert_eq!(bmap[0], 0b0000_0001);

        // 设置第 7 位（第一个字节的最后一位）
        ext4_bmap_bit_set(&mut bmap, 7);
        assert_eq!(bmap[0], 0b1000_0001);

        // 设置第 8 位（第二个字节的第一位）
        ext4_bmap_bit_set(&mut bmap, 8);
        assert_eq!(bmap[1], 0b0000_0001);

        // 设置第 15 位
        ext4_bmap_bit_set(&mut bmap, 15);
        assert_eq!(bmap[1], 0b1000_0001);

        // 设置第 31 位（最后一位）
        ext4_bmap_bit_set(&mut bmap, 31);
        assert_eq!(bmap[3], 0b1000_0000);
    }

    #[test]
    fn test_bmap_bit_clr() {
        let mut bmap = [0xFFu8; 4]; // 所有位都是 1

        // 清除第 0 位
        ext4_bmap_bit_clr(&mut bmap, 0);
        assert_eq!(bmap[0], 0b1111_1110);

        // 清除第 7 位
        ext4_bmap_bit_clr(&mut bmap, 7);
        assert_eq!(bmap[0], 0b0111_1110);

        // 清除第 8 位
        ext4_bmap_bit_clr(&mut bmap, 8);
        assert_eq!(bmap[1], 0b1111_1110);

        // 清除第 31 位
        ext4_bmap_bit_clr(&mut bmap, 31);
        assert_eq!(bmap[3], 0b0111_1111);
    }

    #[test]
    fn test_bmap_is_bit_set() {
        let mut bmap = [0u8; 4];

        // 初始状态：所有位都是 0
        assert!(!ext4_bmap_is_bit_set(&bmap, 0));
        assert!(!ext4_bmap_is_bit_set(&bmap, 15));
        assert!(!ext4_bmap_is_bit_set(&bmap, 31));

        // 设置一些位
        ext4_bmap_bit_set(&mut bmap, 5);
        ext4_bmap_bit_set(&mut bmap, 10);
        ext4_bmap_bit_set(&mut bmap, 25);

        // 验证设置的位
        assert!(ext4_bmap_is_bit_set(&bmap, 5));
        assert!(ext4_bmap_is_bit_set(&bmap, 10));
        assert!(ext4_bmap_is_bit_set(&bmap, 25));

        // 验证未设置的位
        assert!(!ext4_bmap_is_bit_set(&bmap, 0));
        assert!(!ext4_bmap_is_bit_set(&bmap, 4));
        assert!(!ext4_bmap_is_bit_set(&bmap, 6));
        assert!(!ext4_bmap_is_bit_set(&bmap, 31));
    }

    #[test]
    fn test_bmap_is_bit_clr() {
        let mut bmap = [0xFFu8; 4];

        // 初始状态：所有位都是 1
        assert!(!ext4_bmap_is_bit_clr(&bmap, 0));
        assert!(!ext4_bmap_is_bit_clr(&bmap, 15));
        assert!(!ext4_bmap_is_bit_clr(&bmap, 31));

        // 清除一些位
        ext4_bmap_bit_clr(&mut bmap, 5);
        ext4_bmap_bit_clr(&mut bmap, 10);
        ext4_bmap_bit_clr(&mut bmap, 25);

        // 验证清除的位
        assert!(ext4_bmap_is_bit_clr(&bmap, 5));
        assert!(ext4_bmap_is_bit_clr(&bmap, 10));
        assert!(ext4_bmap_is_bit_clr(&bmap, 25));

        // 验证未清除的位
        assert!(!ext4_bmap_is_bit_clr(&bmap, 0));
        assert!(!ext4_bmap_is_bit_clr(&bmap, 4));
        assert!(!ext4_bmap_is_bit_clr(&bmap, 6));
        assert!(!ext4_bmap_is_bit_clr(&bmap, 31));
    }

    #[test]
    fn test_bmap_bits_free_aligned() {
        let mut bmap = [0xFFu8; 8]; // 64 bits，全部设置

        // 从字节边界开始释放 16 位（2 字节）
        ext4_bmap_bits_free(&mut bmap, 8, 16);

        assert_eq!(bmap[0], 0xFF); // 未受影响
        assert_eq!(bmap[1], 0x00); // 第 8-15 位清除
        assert_eq!(bmap[2], 0x00); // 第 16-23 位清除
        assert_eq!(bmap[3], 0xFF); // 未受影响
    }

    #[test]
    fn test_bmap_bits_free_unaligned() {
        let mut bmap = [0xFFu8; 8];

        // 从非字节边界开始释放 10 位
        ext4_bmap_bits_free(&mut bmap, 5, 10);

        // 第 0 字节：前 5 位保持，后 3 位清除
        assert_eq!(bmap[0], 0b0001_1111);
        // 第 1 字节：前 7 位清除，最后 1 位保持
        assert_eq!(bmap[1], 0b1000_0000);
        // 其他字节未受影响
        assert_eq!(bmap[2], 0xFF);
    }

    #[test]
    fn test_bmap_bits_free_single_byte() {
        let mut bmap = [0xFFu8; 4];

        // 释放单个字节内的几位
        ext4_bmap_bits_free(&mut bmap, 2, 4);

        // 位 2, 3, 4, 5 被清除
        // 二进制：11000011 = 0xC3
        assert_eq!(bmap[0], 0b1100_0011);
        assert_eq!(bmap[1], 0xFF);
    }

    #[test]
    fn test_bmap_bits_free_full_bytes() {
        let mut bmap = [0xFFu8; 16];

        // 释放整整 4 个字节（32 位）
        ext4_bmap_bits_free(&mut bmap, 16, 32);

        assert_eq!(bmap[0], 0xFF);
        assert_eq!(bmap[1], 0xFF);
        assert_eq!(bmap[2], 0x00);
        assert_eq!(bmap[3], 0x00);
        assert_eq!(bmap[4], 0x00);
        assert_eq!(bmap[5], 0x00);
        assert_eq!(bmap[6], 0xFF);
    }

    #[test]
    fn test_bmap_bits_free_zero_count() {
        let mut bmap = [0xFFu8; 4];

        // 释放 0 位，应该不改变任何内容
        ext4_bmap_bits_free(&mut bmap, 10, 0);

        assert_eq!(bmap, [0xFF, 0xFF, 0xFF, 0xFF]);
    }

    #[test]
    fn test_bmap_bit_find_clr_empty() {
        let bmap = [0u8; 4]; // 所有位都清除

        // 应该找到第一个清除的位
        let result = ext4_bmap_bit_find_clr(&bmap, 0, 32);
        assert_eq!(result, Some(0));

        let result = ext4_bmap_bit_find_clr(&bmap, 5, 32);
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_bmap_bit_find_clr_full() {
        let bmap = [0xFFu8; 4]; // 所有位都设置

        // 应该找不到清除的位
        let result = ext4_bmap_bit_find_clr(&bmap, 0, 32);
        assert_eq!(result, None);
    }

    #[test]
    fn test_bmap_bit_find_clr_partial() {
        let mut bmap = [0xFFu8; 4];

        // 清除一些位
        ext4_bmap_bit_clr(&mut bmap, 10);
        ext4_bmap_bit_clr(&mut bmap, 20);
        ext4_bmap_bit_clr(&mut bmap, 25);

        // 从头开始查找
        let result = ext4_bmap_bit_find_clr(&bmap, 0, 32);
        assert_eq!(result, Some(10));

        // 跳过第一个，查找第二个
        let result = ext4_bmap_bit_find_clr(&bmap, 11, 32);
        assert_eq!(result, Some(20));

        // 查找第三个
        let result = ext4_bmap_bit_find_clr(&bmap, 21, 32);
        assert_eq!(result, Some(25));

        // 超过最后一个清除位
        let result = ext4_bmap_bit_find_clr(&bmap, 26, 32);
        assert_eq!(result, None);
    }

    #[test]
    fn test_bmap_bit_find_clr_range() {
        let mut bmap = [0xFFu8; 4];

        // 清除位 5, 10, 15
        ext4_bmap_bit_clr(&mut bmap, 5);
        ext4_bmap_bit_clr(&mut bmap, 10);
        ext4_bmap_bit_clr(&mut bmap, 15);

        // 只在 [0, 8) 范围内查找
        let result = ext4_bmap_bit_find_clr(&bmap, 0, 8);
        assert_eq!(result, Some(5));

        // 在 [6, 12) 范围内查找
        let result = ext4_bmap_bit_find_clr(&bmap, 6, 12);
        assert_eq!(result, Some(10));

        // 在 [11, 20) 范围内查找
        let result = ext4_bmap_bit_find_clr(&bmap, 11, 20);
        assert_eq!(result, Some(15));

        // 在 [0, 5) 范围内查找（不包含位 5）
        let result = ext4_bmap_bit_find_clr(&bmap, 0, 5);
        assert_eq!(result, None);
    }

    #[test]
    fn test_bmap_operations_integration() {
        let mut bmap = [0u8; 8]; // 64 bits

        // 1. 设置一些位
        ext4_bmap_bit_set(&mut bmap, 5);
        ext4_bmap_bit_set(&mut bmap, 10);
        ext4_bmap_bit_set(&mut bmap, 15);
        ext4_bmap_bit_set(&mut bmap, 20);

        // 2. 验证设置
        assert!(ext4_bmap_is_bit_set(&bmap, 5));
        assert!(ext4_bmap_is_bit_set(&bmap, 10));
        assert!(ext4_bmap_is_bit_set(&bmap, 15));
        assert!(ext4_bmap_is_bit_set(&bmap, 20));

        // 3. 查找第一个清除的位
        let first_clr = ext4_bmap_bit_find_clr(&bmap, 0, 64);
        assert_eq!(first_clr, Some(0));

        let next_clr = ext4_bmap_bit_find_clr(&bmap, 6, 64);
        assert_eq!(next_clr, Some(6));

        // 4. 批量释放
        ext4_bmap_bits_free(&mut bmap, 5, 16); // 释放位 5-20

        // 5. 验证释放
        assert!(ext4_bmap_is_bit_clr(&bmap, 5));
        assert!(ext4_bmap_is_bit_clr(&bmap, 10));
        assert!(ext4_bmap_is_bit_clr(&bmap, 15));
        assert!(ext4_bmap_is_bit_clr(&bmap, 20));
    }

    #[test]
    fn test_bmap_boundary_conditions() {
        let mut bmap = [0u8; 4];

        // 测试第一位
        ext4_bmap_bit_set(&mut bmap, 0);
        assert!(ext4_bmap_is_bit_set(&bmap, 0));

        // 测试最后一位
        ext4_bmap_bit_set(&mut bmap, 31);
        assert!(ext4_bmap_is_bit_set(&bmap, 31));

        // 测试字节边界
        ext4_bmap_bit_set(&mut bmap, 7);
        ext4_bmap_bit_set(&mut bmap, 8);
        assert!(ext4_bmap_is_bit_set(&bmap, 7));
        assert!(ext4_bmap_is_bit_set(&bmap, 8));
    }

    #[test]
    fn test_bmap_alternating_pattern() {
        let mut bmap = [0u8; 4];

        // 设置交替位: 0, 2, 4, 6, 8, ...
        for i in (0..32).step_by(2) {
            ext4_bmap_bit_set(&mut bmap, i);
        }

        // 验证模式
        for i in 0..32 {
            if i % 2 == 0 {
                assert!(ext4_bmap_is_bit_set(&bmap, i), "Bit {} should be set", i);
            } else {
                assert!(ext4_bmap_is_bit_clr(&bmap, i), "Bit {} should be clear", i);
            }
        }

        // 查找清除的位应该返回 1
        let result = ext4_bmap_bit_find_clr(&bmap, 0, 32);
        assert_eq!(result, Some(1));
    }
}
