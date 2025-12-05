
use core::panic::PanicInfo;

/// Panic 处理
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        crate::println!(
            "\x1b[31m[PANIC] {}:{}: {}\x1b[0m",
            location.file(),
            location.line(),
            info.message()
        );
    } else {
        crate::println!("\x1b[31m[PANIC] {}\x1b[0m", info.message());
    }
    
    // 关机
    const VIRT_TEST: *mut u32 = 0x100000 as *mut u32;
    unsafe {
        VIRT_TEST.write_volatile(0x3333); // fail code
    }
    
    loop {
        unsafe { core::arch::asm!("wfi") }
    }
}

/// abort
#[no_mangle]
extern "C" fn abort() -> ! {
    panic!("abort!");
}
