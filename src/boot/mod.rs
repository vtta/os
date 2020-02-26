global_asm!(include_str!("entry64.asm"));

#[no_mangle]
extern "C" fn rust_main() -> ! {
    println!("+++ booting kernel +++");
    extern "C" {
        fn _start();
        fn bootstacktop();
    }
    println!("_start vaddr 0x{:x}", _start as usize);
    println!("bootstacktop vaddr 0x{:x}", bootstacktop as usize);

    crate::trap::init();
    unsafe {
        asm!("ebreak"::::"volatile");
    }

    panic!("I'm fucked!");
}
