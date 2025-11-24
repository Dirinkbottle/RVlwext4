// EXT4_MISC module

/// 向上取整除法
/// 例如: div_round_up(10, 3) = 4
#[inline(always)]
pub const fn div_round_up(x: usize, y: usize) -> usize {
    x.div_ceil(y)
}

/// 将 x 对齐到 y 的倍数
/// 例如: align(10, 8) = 16
#[inline(always)]
pub const fn align(x: usize, y: usize) -> usize {
    y * div_round_up(x, y)
}

// 也可以用宏的形式（更接近 C）
#[macro_export]
macro_rules! EXT4_DIV_ROUND_UP {
    ($x:expr, $y:expr) => {
        (($x) + ($y) - 1) / ($y)
    };
}

#[macro_export]
macro_rules! EXT4_ALIGN {
    ($x:expr, $y:expr) => {
        ($y) * $crate::EXT4_DIV_ROUND_UP!($x, $y)
    };
}

/// 64位字节序反转
#[inline(always)]
pub const fn reorder64(n: u64) -> u64 {
    ((n & 0xff) << 56)
        | ((n & 0xff00) << 40)
        | ((n & 0xff0000) << 24)
        | ((n & 0xff000000) << 8)
        | ((n & 0xff00000000) >> 8)
        | ((n & 0xff0000000000) >> 24)
        | ((n & 0xff000000000000) >> 40)
        | ((n & 0xff00000000000000) >> 56)
}

/// 32位字节序反转
#[inline(always)]
pub const fn reorder32(n: u32) -> u32 {
    ((n & 0xff) << 24) | ((n & 0xff00) << 8) | ((n & 0xff0000) >> 8) | ((n & 0xff000000) >> 24)
}

/// 16位字节序反转
#[inline(always)]
pub const fn reorder16(n: u16) -> u16 {
    ((n & 0xff) << 8) | ((n & 0xff00) >> 8)
}

#[cfg(target_endian = "big")]
mod endian {
    use super::*;

    #[inline(always)]
    pub const fn to_le64(n: u64) -> u64 {
        reorder64(n)
    }
    #[inline(always)]
    pub const fn to_le32(n: u32) -> u32 {
        reorder32(n)
    }
    #[inline(always)]
    pub const fn to_le16(n: u16) -> u16 {
        reorder16(n)
    }

    #[inline(always)]
    pub const fn to_be64(n: u64) -> u64 {
        n
    }
    #[inline(always)]
    pub const fn to_be32(n: u32) -> u32 {
        n
    }
    #[inline(always)]
    pub const fn to_be16(n: u16) -> u16 {
        n
    }
}

#[cfg(target_endian = "little")]
mod endian {
    use super::*;

    #[inline(always)]
    pub const fn to_le64(n: u64) -> u64 {
        n
    }
    #[inline(always)]
    pub const fn to_le32(n: u32) -> u32 {
        n
    }
    #[inline(always)]
    pub const fn to_le16(n: u16) -> u16 {
        n
    }

    #[inline(always)]
    pub const fn to_be64(n: u64) -> u64 {
        reorder64(n)
    }
    #[inline(always)]
    pub const fn to_be32(n: u32) -> u32 {
        reorder32(n)
    }
    #[inline(always)]
    pub const fn to_be16(n: u16) -> u16 {
        reorder16(n)
    }
}

#[inline(always)]
pub fn to_le16(n: u16) -> u16 {
    n.to_le()
}
#[inline(always)]
pub fn to_le32(n: u32) -> u32 {
    n.to_le()
}
#[inline(always)]
pub fn to_le64(n: u64) -> u64 {
    n.to_le()
}

#[inline(always)]
pub fn to_be16(n: u16) -> u16 {
    n.to_be()
}
#[inline(always)]
pub fn to_be32(n: u32) -> u32 {
    n.to_be()
}
#[inline(always)]
pub fn to_be64(n: u64) -> u64 {
    n.to_be()
}

/// 从 EXT4 结构体读取 32 位字段（小端）
#[macro_export]
macro_rules! ext4_get32 {
    ($s:expr, $f:ident) => {
        $crate::ext4_misc::to_le32($s.$f)
    };
}

/// 从 EXT4 结构体读取 16 位字段（小端）
#[macro_export]
macro_rules! ext4_get16 {
    ($s:expr, $f:ident) => {
        $crate::ext4_misc::to_le16($s.$f)
    };
}

/// 从 EXT4 结构体读取 8 位字段
#[macro_export]
macro_rules! ext4_get8 {
    ($s:expr, $f:ident) => {
        $s.$f
    };
}

/// 向 EXT4 结构体写入 32 位字段（小端）
#[macro_export]
macro_rules! ext4_set32 {
    ($s:expr, $f:ident, $v:expr) => {
        $s.$f = $crate::ext4_misc::to_le32($v)
    };
}

/// 向 EXT4 结构体写入 16 位字段（小端）
#[macro_export]
macro_rules! ext4_set16 {
    ($s:expr, $f:ident, $v:expr) => {
        $s.$f = $crate::ext4_misc::to_le16($v)
    };
}

/// 向 EXT4 结构体写入 8 位字段
#[macro_export]
macro_rules! ext4_set8 {
    ($s:expr, $f:ident, $v:expr) => {
        $s.$f = $v
    };
}

/// 从 JBD2 结构体读取 32 位字段（大端）
#[macro_export]
macro_rules! jbd_get32 {
    ($s:expr, $f:ident) => {
        $crate::misc::to_be32($s.$f)
    };
}

/// 从 JBD2 结构体读取 16 位字段（大端）
#[macro_export]
macro_rules! jbd_get16 {
    ($s:expr, $f:ident) => {
        $crate::misc::to_be16($s.$f)
    };
}

/// 从 JBD2 结构体读取 8 位字段
#[macro_export]
macro_rules! jbd_get8 {
    ($s:expr, $f:ident) => {
        $s.$f
    };
}

/// 向 JBD2 结构体写入 32 位字段（大端）
#[macro_export]
macro_rules! jbd_set32 {
    ($s:expr, $f:ident, $v:expr) => {
        $s.$f = $crate::misc::to_be32($v)
    };
}

/// 向 JBD2 结构体写入 16 位字段（大端）
#[macro_export]
macro_rules! jbd_set16 {
    ($s:expr, $f:ident, $v:expr) => {
        $s.$f = $crate::misc::to_be16($v)
    };
}

/// 向 JBD2 结构体写入 8 位字段
#[macro_export]
macro_rules! jbd_set8 {
    ($s:expr, $f:ident, $v:expr) => {
        $s.$f = $v
    };
}

/// 获取结构体字段偏移量 获取到field的偏移量，不包含field
#[macro_export]
macro_rules! offsetof {
    ($type:ty, $field:ident) => {{
        let dummy = core::mem::MaybeUninit::<$type>::uninit();
        let dummy_ptr = dummy.as_ptr();
        let field_ptr = unsafe { core::ptr::addr_of!((*dummy_ptr).$field) };
        (field_ptr as usize) - (dummy_ptr as usize)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_div_round_up() {
        assert_eq!(div_round_up(10, 3), 4);
        assert_eq!(div_round_up(9, 3), 3);
        assert_eq!(div_round_up(1, 1), 1);
    }

    #[test]
    fn test_align() {
        assert_eq!(align(10, 8), 16);
        assert_eq!(align(16, 8), 16);
        assert_eq!(align(1, 8), 8);
    }

    #[test]
    fn test_reorder() {
        assert_eq!(reorder16(0x1234), 0x3412);
        assert_eq!(reorder32(0x12345678), 0x78563412);
        assert_eq!(reorder64(0x123456789ABCDEF0), 0xF0DEBC9A78563412);
    }

    #[test]
    fn test_endian() {
        #[cfg(target_endian = "little")]
        {
            assert_eq!(to_le32(0x12345678), 0x12345678);
            assert_eq!(to_be32(0x12345678), 0x78563412);
        }

        #[cfg(target_endian = "big")]
        {
            assert_eq!(to_le32(0x12345678), 0x78563412);
            assert_eq!(to_be32(0x12345678), 0x12345678);
        }
    }

    #[test]
    fn test_macros() {
        assert_eq!(EXT4_DIV_ROUND_UP!(10, 3), 4);
        assert_eq!(EXT4_ALIGN!(10, 8), 16);
    }

    #[test]
    fn test_struct_access() {
        #[repr(C)]
        struct TestStruct {
            field32: u32,
            field16: u16,
            field8: u8,
        }

        let mut s = TestStruct {
            field32: 0x12345678,
            field16: 0x1234,
            field8: 0x12,
        };

        // 测试读取
        let val32 = ext4_get32!(s, field32);
        let val16 = ext4_get16!(s, field16);

        #[cfg(target_endian = "little")]
        {
            assert_eq!(val32, 0x12345678);
            assert_eq!(val16, 0x1234);
        }

        // 测试写入
        ext4_set32!(s, field32, 0xAABBCCDD);
        ext4_set16!(s, field16, 0xAABB);
        ext4_set8!(s, field8, 0xAA);

        assert_eq!(ext4_get8!(s, field8), 0xAA);
    }

    #[test]
    fn test_offsetof() {
        #[repr(C)]
        struct TestStruct {
            a: u32,
            b: u16,
            c: u8,
        }

        assert_eq!(offsetof!(TestStruct, a), 0);
        assert_eq!(offsetof!(TestStruct, b), 4);
        assert_eq!(offsetof!(TestStruct, c), 6);
    }
}
