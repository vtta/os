use crate::config::{KERNEL_STACK_SIZE, PAGE_SIZE};
use alloc::boxed::Box;

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
    fn new_kthread(entry: usize) -> Box<Thread> {
        Thread::with_args(entry).create()
    }
    fn with_args(entry: usize) -> ThreadArgs {
        ThreadArgs {
            entry,
            n: 0,
            args: [0; 8],
        }
    }
}

struct ThreadArgs {
    entry: usize,
    n: usize,
    args: [usize; 8],
}
impl ThreadArgs {
    fn arg(mut self, arg: usize) -> Self {
        self.args[self.n] = arg;
        self.n += 1;
        self
    }
    fn create(mut self) -> Box<Thread> {
        unsafe {
            let kstack = KStack::new();
            let context = context::Context::new_kthread(
                self.entry,
                kstack.top(),
                riscv::register::satp::read().bits(),
            );
            let content = &mut *(context.addr as *mut context::ContextContent);
            content.tf.x[10..18].clone_from_slice(&self.args[..]);
            Box::new(Thread { context, kstack })
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
                alloc::alloc::Layout::from_size_align(KERNEL_STACK_SIZE, KERNEL_STACK_SIZE)
                    .expect("kernel stack layout illegal"),
            ) as usize
        };
        Self { bottom }
    }
    fn top(&self) -> usize {
        self.bottom + KERNEL_STACK_SIZE
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
