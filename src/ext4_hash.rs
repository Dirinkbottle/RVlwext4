//! EXT4 directory indexing hash functions
//! Corresponds to ext4_hash.c/h in the C implementation
//!
//! This module implements various hash algorithms used by EXT2/3/4 filesystems
//! for directory indexing (HTree).

use crate::ext4_errno::*;

/// Hash algorithm: Legacy
pub const EXT2_HTREE_LEGACY: i32 = 0;
/// Hash algorithm: Half MD4
pub const EXT2_HTREE_HALF_MD4: i32 = 1;
/// Hash algorithm: TEA (Tiny Encryption Algorithm)
pub const EXT2_HTREE_TEA: i32 = 2;
/// Hash algorithm: Legacy with unsigned char
pub const EXT2_HTREE_LEGACY_UNSIGNED: i32 = 3;
/// Hash algorithm: Half MD4 with unsigned char
pub const EXT2_HTREE_HALF_MD4_UNSIGNED: i32 = 4;
/// Hash algorithm: TEA with unsigned char
pub const EXT2_HTREE_TEA_UNSIGNED: i32 = 5;

/// HTree end-of-file marker
pub const EXT2_HTREE_EOF: u32 = 0x7FFFFFFF;

/// Hash information structure
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Ext4HashInfo {
    pub hash: u32,
    pub minor_hash: u32,
    pub hash_version: u32,
    pub seed: Option<[u32; 4]>,
}

// MD4 functions
#[inline(always)]
const fn f(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (!x & z)
}

#[inline(always)]
const fn g(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (x & z) | (y & z)
}

#[inline(always)]
const fn h(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}

#[inline(always)]
const fn rotate_left(x: u32, n: u32) -> u32 {
    (x << n) | (x >> (32 - n))
}

/// Half MD4 transformation
fn ext2_half_md4(hash: &mut [u32; 4], data: &[u32; 8]) {
    let mut a = hash[0];
    let mut b = hash[1];
    let mut c = hash[2];
    let mut d = hash[3];

    // Round 1
    macro_rules! ff {
        ($a:expr, $b:expr, $c:expr, $d:expr, $x:expr, $s:expr) => {
            $a = $a.wrapping_add(f($b, $c, $d)).wrapping_add($x);
            $a = rotate_left($a, $s);
        };
    }

    ff!(a, b, c, d, data[0], 3);
    ff!(d, a, b, c, data[1], 7);
    ff!(c, d, a, b, data[2], 11);
    ff!(b, c, d, a, data[3], 19);
    ff!(a, b, c, d, data[4], 3);
    ff!(d, a, b, c, data[5], 7);
    ff!(c, d, a, b, data[6], 11);
    ff!(b, c, d, a, data[7], 19);

    // Round 2
    macro_rules! gg {
        ($a:expr, $b:expr, $c:expr, $d:expr, $x:expr, $s:expr) => {
            $a = $a
                .wrapping_add(g($b, $c, $d))
                .wrapping_add($x)
                .wrapping_add(0x5A827999);
            $a = rotate_left($a, $s);
        };
    }

    gg!(a, b, c, d, data[1], 3);
    gg!(d, a, b, c, data[3], 5);
    gg!(c, d, a, b, data[5], 9);
    gg!(b, c, d, a, data[7], 13);
    gg!(a, b, c, d, data[0], 3);
    gg!(d, a, b, c, data[2], 5);
    gg!(c, d, a, b, data[4], 9);
    gg!(b, c, d, a, data[6], 13);

    // Round 3
    macro_rules! hh {
        ($a:expr, $b:expr, $c:expr, $d:expr, $x:expr, $s:expr) => {
            $a = $a
                .wrapping_add(h($b, $c, $d))
                .wrapping_add($x)
                .wrapping_add(0x6ED9EBA1);
            $a = rotate_left($a, $s);
        };
    }

    hh!(a, b, c, d, data[3], 3);
    hh!(d, a, b, c, data[7], 9);
    hh!(c, d, a, b, data[2], 11);
    hh!(b, c, d, a, data[6], 15);
    hh!(a, b, c, d, data[1], 3);
    hh!(d, a, b, c, data[5], 9);
    hh!(c, d, a, b, data[0], 11);
    hh!(b, c, d, a, data[4], 15);

    hash[0] = hash[0].wrapping_add(a);
    hash[1] = hash[1].wrapping_add(b);
    hash[2] = hash[2].wrapping_add(c);
    hash[3] = hash[3].wrapping_add(d);
}

/// TEA (Tiny Encryption Algorithm) transformation
fn ext2_tea(hash: &mut [u32; 4], data: &[u32; 8]) {
    const TEA_DELTA: u32 = 0x9E3779B9;

    let mut x = hash[0];
    let mut y = hash[1];

    for i in 1..=16 {
        let sum = (i as u32).wrapping_mul(TEA_DELTA);
        x = x.wrapping_add(
            ((y << 4).wrapping_add(data[0]))
                ^ (y.wrapping_add(sum))
                ^ ((y >> 5).wrapping_add(data[1])),
        );
        y = y.wrapping_add(
            ((x << 4).wrapping_add(data[2]))
                ^ (x.wrapping_add(sum))
                ^ ((x >> 5).wrapping_add(data[3])),
        );
    }

    hash[0] = hash[0].wrapping_add(x);
    hash[1] = hash[1].wrapping_add(y);
}

/// Legacy hash algorithm
fn ext2_legacy_hash(name: &[u8], unsigned_char: bool) -> u32 {
    let mut h1: u32 = 0x12A3FE2D;
    let mut h2: u32 = 0x37ABE8F9;
    const MULTI: u32 = 0x6D22F5;

    for &byte in name {
        let val = if unsigned_char {
            byte as u32
        } else {
            (byte as i8) as i32 as u32
        };

        let h0 = h2.wrapping_add(h1 ^ val.wrapping_mul(MULTI));
        let h0 = if h0 & 0x80000000 != 0 {
            h0.wrapping_sub(0x7FFFFFFF)
        } else {
            h0
        };

        h2 = h1;
        h1 = h0;
    }

    h1 << 1
}

/// Prepare hash buffer from source string
fn ext2_prep_hashbuf(src: &[u8], dst: &mut [u32; 8], unsigned_char: bool) {
    let slen = src.len() as u32;
    let padding = slen | (slen << 8) | (slen << 16) | (slen << 24);
    let dlen = dst.len() * 4; // 8 * 4 = 32 bytes

    let len = slen.min(dlen as u32) as usize;
    let mut buf_val = padding;
    let mut dst_idx = 0;

    for i in 0..len {
        let buf_byte = if unsigned_char {
            src[i] as u32
        } else {
            (src[i] as i8) as i32 as u32
        };

        if i % 4 == 0 {
            buf_val = padding;
        }

        buf_val = (buf_val << 8) | buf_byte;

        if i % 4 == 3 {
            if dst_idx < dst.len() {
                dst[dst_idx] = buf_val;
                dst_idx += 1;
            }
            buf_val = padding;
        }
    }

    // 处理剩余数据
    if len % 4 != 0 && dst_idx < dst.len() {
        // 需要补充填充
        let remaining = 4 - (len % 4);
        for _ in 0..remaining {
            buf_val <<= 8;
            buf_val |= (padding >> 24) & 0xFF;
        }
        dst[dst_idx] = buf_val;
        dst_idx += 1;
    }

    // 填充剩余的 dst
    while dst_idx < dst.len() {
        dst[dst_idx] = padding;
        dst_idx += 1;
    }
}

/// Calculate HTree hash for directory entry name
///
/// # Arguments
/// * `name` - Entry name bytes
/// * `hash_seed` - Optional hash seed from superblock (4 u32 values)
/// * `hash_version` - Hash algorithm version
/// * `hash_minor` - Optional output for minor hash
///
/// # Returns
/// * `Ok(hash_major)` - Major hash value on success
/// * `Err(errno)` - Error code on failure
pub fn ext2_htree_hash(
    name: &[u8],
    hash_seed: Option<&[u32; 4]>,
    hash_version: i32,
    hash_minor: Option<&mut u32>,
) -> Result<u32, i32> {
    let len = name.len();

    // Validate inputs
    if !(1..=255).contains(&len) {
        if let Some(minor) = hash_minor {
            *minor = 0;
        }
        return Err(ENOTSUP);
    }

    // Initialize hash state with MD4 IV
    let mut hash = [0x67452301u32, 0xEFCDAB89, 0x98BADCFE, 0x10325476];

    // Apply seed if provided
    if let Some(seed) = hash_seed {
        hash.copy_from_slice(seed);
    }

    let (major, minor) = match hash_version {
        EXT2_HTREE_TEA_UNSIGNED | EXT2_HTREE_TEA => {
            let unsigned_char = hash_version == EXT2_HTREE_TEA_UNSIGNED;
            let mut offset = 0;

            while offset < len {
                let mut data = [0u32; 8];
                let chunk = &name[offset..];
                ext2_prep_hashbuf(chunk, &mut data, unsigned_char);
                ext2_tea(&mut hash, &data);
                offset += 16;
            }

            (hash[0], hash[1])
        }

        EXT2_HTREE_LEGACY_UNSIGNED | EXT2_HTREE_LEGACY => {
            let unsigned_char = hash_version == EXT2_HTREE_LEGACY_UNSIGNED;
            let major = ext2_legacy_hash(name, unsigned_char);
            (major, 0)
        }

        EXT2_HTREE_HALF_MD4_UNSIGNED | EXT2_HTREE_HALF_MD4 => {
            let unsigned_char = hash_version == EXT2_HTREE_HALF_MD4_UNSIGNED;
            let mut offset = 0;

            while offset < len {
                let mut data = [0u32; 8];
                let chunk = &name[offset..];
                ext2_prep_hashbuf(chunk, &mut data, unsigned_char);
                ext2_half_md4(&mut hash, &data);
                offset += 32;
            }

            (hash[1], hash[2])
        }

        _ => {
            if let Some(minor) = hash_minor {
                *minor = 0;
            }
            return Err(ENOTSUP);
        }
    };

    // Clear lowest bit and handle EOF case
    let mut major = major & !1;
    if major == (EXT2_HTREE_EOF << 1) {
        major = (EXT2_HTREE_EOF - 1) << 1;
    }

    if let Some(minor_out) = hash_minor {
        *minor_out = minor;
    }

    Ok(major)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_constants() {
        assert_eq!(EXT2_HTREE_LEGACY, 0);
        assert_eq!(EXT2_HTREE_HALF_MD4, 1);
        assert_eq!(EXT2_HTREE_TEA, 2);
        assert_eq!(EXT2_HTREE_LEGACY_UNSIGNED, 3);
        assert_eq!(EXT2_HTREE_HALF_MD4_UNSIGNED, 4);
        assert_eq!(EXT2_HTREE_TEA_UNSIGNED, 5);
        assert_eq!(EXT2_HTREE_EOF, 0x7FFFFFFF);
    }

    #[test]
    fn test_invalid_name_length() {
        // 空名称
        let result = ext2_htree_hash(b"", None, EXT2_HTREE_TEA, None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ENOTSUP);

        // 名称过长 (> 255)
        let long_name = [b'a'; 256];
        let result = ext2_htree_hash(&long_name, None, EXT2_HTREE_TEA, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_hash_version() {
        let name = b"test";
        let result = ext2_htree_hash(name, None, 999, None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ENOTSUP);
    }

    #[test]
    fn test_hash_consistency() {
        let test_names = [
            b"test" as &[u8],
            b"hello",
            b"world",
            b"a",
            b"test_file_name",
        ];

        let versions = [
            EXT2_HTREE_LEGACY,
            EXT2_HTREE_HALF_MD4,
            EXT2_HTREE_TEA,
            EXT2_HTREE_LEGACY_UNSIGNED,
            EXT2_HTREE_HALF_MD4_UNSIGNED,
            EXT2_HTREE_TEA_UNSIGNED,
        ];

        for name in &test_names {
            for &version in &versions {
                let hash1 = ext2_htree_hash(name, None, version, None).unwrap();
                let hash2 = ext2_htree_hash(name, None, version, None).unwrap();
                assert_eq!(
                    hash1,
                    hash2,
                    "Hash should be consistent for name {:?} with version {}",
                    core::str::from_utf8(name),
                    version
                );
            }
        }
    }

    #[test]
    fn test_hash_different_names() {
        let names = [b"test" as &[u8], b"test2", b"different"];

        for &version in &[EXT2_HTREE_TEA, EXT2_HTREE_HALF_MD4] {
            let hash1 = ext2_htree_hash(names[0], None, version, None).unwrap();
            let hash2 = ext2_htree_hash(names[1], None, version, None).unwrap();
            let hash3 = ext2_htree_hash(names[2], None, version, None).unwrap();

            assert_ne!(hash1, hash2);
            assert_ne!(hash1, hash3);
            assert_ne!(hash2, hash3);
        }
    }

    #[test]
    fn test_hash_with_seed() {
        let name = b"test_file";
        let seed1 = [0x12345678u32, 0xABCDEF00, 0x11111111, 0x22222222];
        let seed2 = [0x87654321u32, 0x00FEDCBA, 0x33333333, 0x44444444];

        let hash1 = ext2_htree_hash(name, Some(&seed1), EXT2_HTREE_TEA, None).unwrap();
        let hash2 = ext2_htree_hash(name, Some(&seed2), EXT2_HTREE_TEA, None).unwrap();
        let hash3 = ext2_htree_hash(name, None, EXT2_HTREE_TEA, None).unwrap();

        // 不同的 seed 应该产生不同的 hash
        assert_ne!(hash1, hash2);
        assert_ne!(hash1, hash3);
        assert_ne!(hash2, hash3);
    }

    #[test]
    fn test_hash_minor_output() {
        let name = b"test_file";
        let mut minor = 0u32;

        let major = ext2_htree_hash(name, None, EXT2_HTREE_TEA, Some(&mut minor)).unwrap();

        // 确保major和minor都有值
        assert_ne!(major, 0);
        // minor 可能为 0，但应该被设置
    }

    #[test]
    fn test_hash_lowest_bit_cleared() {
        let name = b"test";

        for &version in &[EXT2_HTREE_TEA, EXT2_HTREE_HALF_MD4, EXT2_HTREE_LEGACY] {
            let hash = ext2_htree_hash(name, None, version, None).unwrap();
            // 最低位应该被清除
            assert_eq!(hash & 1, 0, "Lowest bit should be cleared");
        }
    }

    #[test]
    fn test_signed_vs_unsigned() {
        let name = b"\x80\xFF\x7F\x01"; // 包含高位字节

        // 签名和无符号版本应该产生不同的结果
        let hash_signed = ext2_htree_hash(name, None, EXT2_HTREE_TEA, None).unwrap();
        let hash_unsigned = ext2_htree_hash(name, None, EXT2_HTREE_TEA_UNSIGNED, None).unwrap();

        assert_ne!(hash_signed, hash_unsigned);
    }

    #[test]
    fn test_long_name() {
        // 测试接近最大长度的名称
        let name = [b'a'; 255];

        for &version in &[EXT2_HTREE_TEA, EXT2_HTREE_HALF_MD4, EXT2_HTREE_LEGACY] {
            let result = ext2_htree_hash(&name, None, version, None);
            assert!(result.is_ok(), "Should handle max length name");
        }
    }

    #[test]
    fn test_all_hash_algorithms() {
        let name = b"test_all_algorithms";

        let versions = [
            ("LEGACY", EXT2_HTREE_LEGACY),
            ("HALF_MD4", EXT2_HTREE_HALF_MD4),
            ("TEA", EXT2_HTREE_TEA),
            ("LEGACY_UNSIGNED", EXT2_HTREE_LEGACY_UNSIGNED),
            ("HALF_MD4_UNSIGNED", EXT2_HTREE_HALF_MD4_UNSIGNED),
            ("TEA_UNSIGNED", EXT2_HTREE_TEA_UNSIGNED),
        ];

        for (desc, version) in &versions {
            let result = ext2_htree_hash(name, None, *version, None);
            assert!(result.is_ok(), "Algorithm {} should work", desc);
        }
    }
}
