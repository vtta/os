use crate::config::*;

mod addr;
mod frame;
mod heap;
mod page;

pub fn init() {
    println!("+++ setting up physical memory +++");
    extern "C" {
        fn end();
    }
    let bppn = ((end as usize - KERNEL_BEGIN_VADDR + KERNEL_BEGIN_PADDR) / PAGE_SIZE + 1).into();
    let eppn = (PHYSICAL_MEMORY_END / PAGE_SIZE).into();
    frame::init(bppn, eppn);
    frame::test(bppn, eppn);
    heap::init();
    heap::test();
}
