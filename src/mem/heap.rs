extern crate alloc;
use crate::config::KERNEL_HEAP_SIZE;
use alloc::boxed::Box;
use alloc::vec::Vec;
use buddy_system_allocator::LockedHeap;

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::new();

#[alloc_error_handler]
fn alloc_error_handler(_: core::alloc::Layout) -> ! {
    panic!("there is no enough space in the kernel heap");
}

pub(crate) fn init() {
    static mut HEAP: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];
    unsafe {
        HEAP_ALLOCATOR
            .lock()
            .init(HEAP.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }
}

pub(crate) fn test() {
    fn in_heap(pos: usize) -> bool {
        extern "C" {
            fn sbss();
            fn ebss();
        }
        let sbss = sbss as usize;
        let ebss = ebss as usize;
        sbss <= pos && pos < ebss
    }
    let ptr = Box::new(5);
    assert!(in_heap(ptr.as_ref() as *const _ as usize));
    assert_eq!(*ptr, 5);
    let mut vec = Vec::new();
    for i in 0..10000 {
        vec.push(i);
        assert!(in_heap(&vec[i] as *const _ as usize));
        assert_eq!(vec[i], i);
    }
    assert_eq!(vec.len(), 10000);
    for i in (0..10000).rev() {
        assert!(in_heap(&vec[i] as *const _ as usize));
        assert_eq!(vec[i], i);
        vec.pop();
    }
    assert_eq!(vec.len(), 0);
    let vec: Vec<u8> = Vec::with_capacity(KERNEL_HEAP_SIZE / 2);
    assert!(in_heap(vec.as_ptr() as usize));
    drop(vec);
    let vec: Vec<u8> = Vec::with_capacity(KERNEL_HEAP_SIZE / 2);
    assert!(in_heap(vec.as_ptr() as usize));
    let vec: Vec<u8> = Vec::with_capacity(KERNEL_HEAP_SIZE / 4);
    assert!(in_heap(vec.as_ptr() as usize));
    let vec: Vec<u8> = Vec::with_capacity(KERNEL_HEAP_SIZE / 8);
    assert!(in_heap(vec.as_ptr() as usize));
    let vec: Vec<u8> = Vec::with_capacity(KERNEL_HEAP_SIZE / 16);
    assert!(in_heap(vec.as_ptr() as usize));
}
