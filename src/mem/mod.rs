use crate::config::*;
use crate::mem::set::MemSet;

mod addr;
mod frame;
mod heap;
pub(crate) mod page;
mod set;

pub fn init() {
    println!("+++ setting up physical memory +++");
    extern "C" {
        fn end();
        fn boot_stack();
        fn boot_stack_top();
    }
    let bppn = (end as usize - KERNEL_BEGIN_VADDR + KERNEL_BEGIN_PADDR) / PAGE_SIZE + 1;
    let eppn = PHYSICAL_MEMORY_END / PAGE_SIZE;
    heap::init();
    heap::test();
    frame::init(bppn, eppn);
    frame::test(bppn, eppn);
    let mut memset = MemSet::new();
    memset.push(
        (boot_stack as usize).into(),
        (boot_stack_top as usize).into(),
        set::handler::Linear::new(PHYSICAL_MEMORY_OFFSET),
        set::attrib::MemAttrib::new().readable(true).writable(true),
    );
    unsafe {
        memset.activate();
    }
}
