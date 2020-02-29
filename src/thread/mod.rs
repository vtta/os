use crate::config::{KERNEL_STACK_SIZE, PAGE_SIZE};

mod context;

pub(crate) struct Thread {
    context: context::Context,
    kstack: KStack,
}
impl Thread {
    fn switch(&mut self, target: &mut Thread) {
        unsafe {
            self.context.switch(&mut target.context);
        }
    }
}

pub(crate) struct KStack {
    bottom: usize,
}
impl KStack {
    fn new() -> Self {
        let bottom = unsafe {
            alloc::alloc::alloc(
                alloc::alloc::Layout::from_size_align(KERNEL_STACK_SIZE, PAGE_SIZE)
                    .expect("kernel stack layout illegal"),
            ) as usize
        };
        Self { bottom }
    }
}
impl Drop for KStack {
    fn drop(&mut self) {
        unsafe {
            alloc::alloc::dealloc(
                self.bottom as _,
                alloc::alloc::Layout::from_size_align(KERNEL_STACK_SIZE, PAGE_SIZE)
                    .expect("kernel stack layout illegal"),
            )
        }
    }
}
