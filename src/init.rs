global_asm!(include_str!("boot/entry64.asm"));

#[no_mangle]
extern "C" fn boot_main() -> ! {
    println!("+++ booting kernel +++");
    extern "C" {
        fn _start();
        fn bootstacktop();
    }
    println!("_start vaddr 0x{:x}", _start as usize);
    println!("bootstacktop vaddr 0x{:x}", bootstacktop as usize);

    crate::interrupt::init();
    unsafe {
        asm!("ebreak"::::"volatile");
    }

    panic!("I'm fucked!");
}
