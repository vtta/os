#![no_std]
#![no_main]
#![feature(global_asm)]

use core::panic::PanicInfo;

global_asm!(include_str!("boot/entry64.asm"));

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    loop {}
}
