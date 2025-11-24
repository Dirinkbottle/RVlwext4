// 块设备

// 没必要packed压缩
#[repr(C)]
pub struct ext4_blockdev {
    /// Offset in bdif. For multi partition mode.
    part_offset: u64,

    /// Part size in bdif. For multi partition mode.
    part_size: u64,

    /// Block cache.
    // struct ext4_bcache *bc;

    /// Block size (bytes) logical
    lg_bsize: u32,

    /// Block count: logical
    lg_bcnt: u64,

    /// Cache write back mode reference counter
    cache_write_back: u32,
    // struct ext4_fs *fs;

    // void *journal;
}

impl ext4_blockdev {
    pub fn open(&self) {}
    pub fn bread(&self, read_buffer: &mut [u8], block_id: u64, read_count: u32) {}
}
