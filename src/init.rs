global_asm!(include_str!("boot/entry64.asm"));

#[no_mangle]
extern "C" fn rust_main() -> ! {
    extern "C" {
        fn _start();
        fn bootstacktop();
    }
    println!("_start vaddr 0x{:x}", _start as usize);
    println!("bootstacktop vaddr 0x{:x}", bootstacktop as usize);
    panic!("I'm fucked!");
    loop {}
}
