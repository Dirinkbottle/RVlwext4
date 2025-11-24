// EXT4_DEBUG mode

use alloc::string::*;

const DEBUG_BALLOC: u32 = 1 << 0;
const DEBUG_BCACHE: u32 = 1 << 1;
const DEBUG_BITMAP: u32 = 1 << 2;
const DEBUG_BLOCK_GROUP: u32 = 1 << 3;
const DEBUG_BLOCKDEV: u32 = 1 << 4;
const DEBUG_DIR_IDX: u32 = 1 << 5;
const DEBUG_DIR: u32 = 1 << 6;
const DEBUG_EXTENT: u32 = 1 << 7;
const DEBUG_FS: u32 = 1 << 8;
const DEBUG_HASH: u32 = 1 << 9;
const DEBUG_IALLOC: u32 = 1 << 10;
const DEBUG_INODE: u32 = 1 << 11;
const DEBUG_SUPER: u32 = 1 << 12;
const DEBUG_XATTR: u32 = 1 << 13;
const DEBUG_MKFS: u32 = 1 << 14;
const DEBUG_EXT4: u32 = 1 << 15;
const DEBUG_JBD: u32 = 1 << 16;
const DEBUG_MBR: u32 = 1 << 17;
const DEBUG_NOPREFIX: u32 = 1 << 31;
const DEBUG_ALL: u32 = 0xFFFFFFFF;

static mut debug_mask: u32 = 0;

#[inline(always)]
pub fn ext4_dmask_id2str(m: u32) -> String {
    match m {
        DEBUG_BALLOC => "ext4_balloc: ".to_string(),
        DEBUG_BCACHE => "ext4_bcache: ".to_string(),
        DEBUG_BITMAP => "ext4_bitmap: ".to_string(),
        DEBUG_BLOCK_GROUP => "ext4_block_group: ".to_string(),
        DEBUG_BLOCKDEV => "ext4_blockdev: ".to_string(),
        DEBUG_DIR_IDX => "ext4_dir_idx: ".to_string(),
        DEBUG_DIR => "ext4_dir: ".to_string(),
        DEBUG_EXTENT => "ext4_extent: ".to_string(),
        DEBUG_FS => "ext4_fs: ".to_string(),
        DEBUG_HASH => "ext4_hash: ".to_string(),
        DEBUG_IALLOC => "ext4_ialloc: ".to_string(),
        DEBUG_INODE => "ext4_inode: ".to_string(),
        DEBUG_SUPER => "ext4_super: ".to_string(),
        DEBUG_XATTR => "ext4_xattr: ".to_string(),
        DEBUG_MKFS => "ext4_mkfs: ".to_string(),
        DEBUG_EXT4 => "ext4_jbd: ".to_string(),
        DEBUG_JBD => "ext4_mbr: ".to_string(),
        DEBUG_MBR => "ext4: ".to_string(),
        _ => " ".to_string(),
    }
}

pub fn ext4_dmask_set(m: u32) {
    unsafe {
        debug_mask |= m;
    }
}

pub fn ext4_dmask_clr(m: u32) {
    unsafe {
        debug_mask &= m;
    }
}

pub fn ext4_dmask_get() -> u32 {
    unsafe { debug_mask }
}

#[macro_export]
macro_rules! ext4_dbg {
    ($mask:expr, $($arg:tt)*) => {
        {
            #[cfg(feature = "debug_printf")]
            {
                if ($mask & $crate::ext4_dmask_get()) != 0 {
                    // 如果不是 DEBUG_NOPREFIX，打印前缀
                    if ($mask & $crate::DEBUG_NOPREFIX) == 0 {
                        debug!("{}", $crate::ext4_dmask_id2str($mask));
                        debug!("l: {}   ", line!());
                    }
                    // 打印用户消息
                    debug!($($arg)*);
                    // 在 no_std 环境中，根据 log 配置决定是否需要 flush
                    // 标准环境可以用 std::io::stdout().flush()
                }
            }
            #[cfg(not(feature = "debug_printf"))]
            {
                // 空操作，避免未使用变量警告
                let _ = ($mask, format_args!($($arg)*));
            }
        }
    };
}

#[macro_export]
macro_rules! ext4_assert {
    ($cond:expr) => {{
        #[cfg(feature = "debug_assert")]
        {
            #[cfg(feature = "own_assert")]
            {
                if !($cond) {
                    debug!("assertion failed:\nfile: {}\nline: {}", file!(), line!());
                    loop {} // 死循环
                }
            }
            #[cfg(not(feature = "own_assert"))]
            {
                assert!($cond); // 使用标准 assert
            }
        }
        #[cfg(not(feature = "debug_assert"))]
        {
            // 空操作，但保留表达式求值以避免副作用差异
            let _ = $cond;
        }
    }};
}
