use crate::config::*;
global_asm!(include_str!("entry64.asm"));

#[no_mangle]
extern "C" fn rust_main() -> ! {
    println!("+++ booting kernel +++");
    extern "C" {
        fn end();
        fn _start();
        fn bootstacktop();
    }
    let text_size = end as usize - KERNEL_BEGIN_VADDR;
    println!(
        "free physical memory paddr [{:#x}, {:#x})",
        KERNEL_BEGIN_PADDR + text_size,
        crate::config::PHYSICAL_MEMORY_END
    );
    println!(
        "free physical memory ppn [{:#x}, {:#x})",
        (KERNEL_BEGIN_PADDR + text_size) / PAGE_SIZE + 1,
        PHYSICAL_MEMORY_END / PAGE_SIZE
    );
    println!("_start vaddr {:#x}", _start as usize);
    println!("bootstacktop vaddr {:#x}", bootstacktop as usize);

    crate::trap::init();
    crate::mem::init(
        ((KERNEL_BEGIN_PADDR + text_size) / PAGE_SIZE + 1).into(),
        (PHYSICAL_MEMORY_END / PAGE_SIZE).into(),
    );
    unsafe {
        asm!("ebreak"::::"volatile");
    }

    panic!("I'm fucked!");
}
