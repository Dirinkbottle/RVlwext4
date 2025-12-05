//! VirtIO 块设备驱动
//! 适配到 RVlwext4 的 BlockDevice trait

use alloc::vec::Vec;
use core::ptr::NonNull;
use spin::Mutex;
use virtio_drivers::{BufferDirection, Hal};
use virtio_drivers::device::blk::VirtIOBlk;
use virtio_drivers::transport::mmio::{MmioTransport, VirtIOHeader};
use RVlwext4::{BLOCK_SIZE, BLOCK_SIZE_U32, BlockDevError, BlockDevResult, BlockDevice};

/// VirtIO MMIO 基地址范围 (QEMU virt 机器)
const VIRTIO_BASE: usize = 0x1000_1000;
const VIRTIO_SIZE: usize = 0x1000;
const VIRTIO_COUNT: usize = 8;

/// 全局块设备
static BLOCK_DEVICE: Mutex<Option<VirtIOBlockWrapper>> = Mutex::new(None);

/// VirtIO HAL 实现
pub struct HalImpl;

unsafe impl Hal for HalImpl {
    fn dma_alloc(pages: usize, _direction: BufferDirection) -> (usize, NonNull<u8>) {
        use alloc::alloc::{alloc_zeroed, Layout};
        let layout = Layout::from_size_align(pages * 4096, 4096).unwrap();
        let ptr = unsafe { alloc_zeroed(layout) };
        let paddr = ptr as usize;
        (paddr, NonNull::new(ptr).unwrap())
    }

    unsafe fn dma_dealloc(paddr: usize, vaddr: NonNull<u8>, pages: usize) -> i32 {
        use alloc::alloc::{dealloc, Layout};
        let layout = Layout::from_size_align(pages * 4096, 4096).unwrap();
        dealloc(vaddr.as_ptr(), layout);
        0
    }

    unsafe fn mmio_phys_to_virt(paddr: usize, _size: usize) -> NonNull<u8> {
        NonNull::new(paddr as *mut u8).unwrap()
    }

    unsafe fn share(buffer: NonNull<[u8]>, _direction: BufferDirection) -> usize {
        buffer.as_ptr() as *mut u8 as usize
    }

    unsafe fn unshare(_paddr: usize, _buffer: NonNull<[u8]>, _direction: BufferDirection) {
        // 在简单的实现中，不需要做任何事
    }
}

/// VirtIO 块设备包装器
pub struct VirtIOBlockWrapper {
    inner: Mutex<VirtIOBlk<HalImpl, MmioTransport>>,
    capacity: u64,
}

impl VirtIOBlockWrapper {
    pub fn new() -> Result<Self, &'static str> {
        unsafe {
            log::debug!("扫描 VirtIO MMIO 设备...");
            
            // 扫描所有可能的 VirtIO MMIO 地址
            for i in 0..VIRTIO_COUNT {
                let addr = VIRTIO_BASE + i * VIRTIO_SIZE;
                
                // 检查 magic 值
                let magic = (addr as *const u32).read_volatile();
                
                if magic != 0x74726976 {
                    continue; // 不是 VirtIO 设备，跳过
                }
                
                log::debug!("发现 VirtIO 设备在 0x{:x}", addr);
                
                // 读取设备 ID (偏移 0x08)
                let device_id = ((addr + 0x08) as *const u32).read_volatile();
                log::debug!("  设备ID: {}", device_id);
                
                // 设备 ID 2 是块设备
                if device_id != 2 {
                    log::debug!("  不是块设备，跳过");
                    continue;
                }
                
                log::info!("找到 VirtIO 块设备在 0x{:x}", addr);
                
                let header = NonNull::new(addr as *mut VirtIOHeader)
                    .ok_or("VirtIO header is null")?;
                
                let transport = MmioTransport::new(header)
                    .map_err(|e| {
                        log::error!("Transport 初始化失败: {:?}", e);
                        "VirtIO transport 初始化失败"
                    })?;
                
                log::debug!("Transport 初始化成功");
                
                let blk = VirtIOBlk::<HalImpl, MmioTransport>::new(transport)
                    .map_err(|e| {
                        log::error!("VirtIOBlk 初始化失败: {:?}", e);
                        "VirtIO 块设备初始化失败"
                    })?;
                
                let capacity = blk.capacity();
                log::info!("VirtIO 块设备容量: {} 扇区", capacity);
                
                return Ok(Self {
                    inner: Mutex::new(blk),
                    capacity,
                });
            }
            
            Err("未找到 VirtIO 块设备")
        }
    }
}

impl BlockDevice for VirtIOBlockWrapper {
    fn read(&self, buffer: &mut [u8], block_id: u32, count: u32) -> BlockDevResult<()> {
        // VirtIO 使用 512 字节扇区，RVlwext4 使用可配置 BLOCK_SIZE（需为 1024/2048/4096）
        const SECTOR_SIZE: usize = 512;
        let sectors_per_block = (BLOCK_SIZE_U32 / SECTOR_SIZE as u32) as u64;

        let start_sector = block_id as u64 * sectors_per_block;
        let total_sectors = count as u64 * sectors_per_block;

        let required = count as usize * BLOCK_SIZE;
        if buffer.len() < required {
            return Err(BlockDevError::BufferTooSmall {
                provided: buffer.len(),
                required,
            });
        }

        // 直接使用 buffer 前 required 字节作为扇区缓冲区
        // 要求 required 始终是 512 的整数倍（由 BLOCK_SIZE 保证）
        self.inner
            .lock()
            .read_blocks(start_sector as usize, &mut buffer[..required])
            .map_err(|_| BlockDevError::ReadError)?;

        Ok(())
    }

    fn write(&mut self, buffer: &[u8], block_id: u32, count: u32) -> BlockDevResult<()> {
        const SECTOR_SIZE: usize = 512;
        let sectors_per_block = (BLOCK_SIZE_U32 / SECTOR_SIZE as u32) as u64;

        let start_sector = block_id as u64 * sectors_per_block;
        let total_sectors = count as u64 * sectors_per_block;

        let required = count as usize * BLOCK_SIZE;
        if buffer.len() < required {
            return Err(BlockDevError::BufferTooSmall {
                provided: buffer.len(),
                required,
            });
        }

        self.inner
            .lock()
            .write_blocks(start_sector as usize, &buffer[..required])
            .map_err(|_| BlockDevError::WriteError)?;

        Ok(())
    }

    fn open(&mut self) -> BlockDevResult<()> {
        Ok(())
    }

    fn close(&mut self) -> BlockDevResult<()> {
        Ok(())
    }

    fn total_blocks(&self) -> u64 {
        // 容量是扇区数，转换为块数
        let sectors_per_block = (BLOCK_SIZE_U32 / 512) as u64;
        self.capacity / sectors_per_block
    }

    fn block_size(&self) -> u32 {
        BLOCK_SIZE_U32
    }

    fn is_open(&self) -> bool {
        true
    }

    fn is_readonly(&self) -> bool {
        false
    }
}

/// 初始化 VirtIO 块设备
pub fn init_virtio_blk() -> Result<(), &'static str> {
    let device = VirtIOBlockWrapper::new()?;
    log::info!("VirtIO 块设备容量: {} 块 ({} MB)", 
               device.total_blocks(),
               device.total_blocks() * BLOCK_SIZE as u64 / (1024 * 1024));
    
    *BLOCK_DEVICE.lock() = Some(device);
    Ok(())
}

/// 获取块设备引用（用于测试）
pub fn get_block_device() -> Option<VirtIOBlockDeviceHandle> {
    let guard = BLOCK_DEVICE.lock();
    if guard.is_some() {
        Some(VirtIOBlockDeviceHandle)
    } else {
        None
    }
}

/// 使用全局块设备的可变引用执行操作
pub fn with_block_device_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut VirtIOBlockWrapper) -> R,
{
    let mut guard = BLOCK_DEVICE.lock();
    let device = guard.as_mut().expect("Block device not initialized");
    f(device)
}

/// 块设备句柄（用于安全访问全局设备）
pub struct VirtIOBlockDeviceHandle;

impl VirtIOBlockDeviceHandle {
    pub fn with_device<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&VirtIOBlockWrapper) -> R,
    {
        let guard = BLOCK_DEVICE.lock();
        guard.as_ref().map(f)
    }
    
    pub fn with_device_mut<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut VirtIOBlockWrapper) -> R,
    {
        let mut guard = BLOCK_DEVICE.lock();
        guard.as_mut().map(f)
    }
}
