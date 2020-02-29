use crate::config::{KERNEL_STACK_SIZE, PAGE_SIZE};
use alloc::boxed::Box;
use core::cmp::min;

mod context;

#[derive(Debug)]
pub(crate) struct Thread {
    context: context::Context,
    kstack: KStack,
}
impl Thread {
    fn switch(&mut self, target: &mut Self) {
        unsafe {
            self.context.switch(&mut target.context);
        }
    }
    fn new(entry: usize) -> Box<Self> {
        Thread::with_args().create(entry)
    }
    fn with_args() -> ThreadArgs {
        ThreadArgs { n: 0, args: [0; 8] }
    }
    fn boot_thread() -> Box<Self> {
        // no need to worry about ra
        // because before the switch from boot thread
        // ra has automatically been set to next instruction following switch
        // DANGER!!! zero inited a pointer
        Box::new(unsafe { core::mem::zeroed() })
    }
    fn add_args(&mut self, args: &[usize]) {
        let context = &mut self.context;
        let content = unsafe { &mut *(context.addr as *mut context::ContextContent) };
        let len = min(8, args.len());
        content.tf.x[10..(10 + len)].clone_from_slice(&args[..len]);
    }
}

struct ThreadArgs {
    n: usize,
    args: [usize; 8],
}
impl ThreadArgs {
    fn arg(mut self, arg: usize) -> Self {
        self.args[self.n] = arg;
        self.n += 1;
        self
    }
    fn create(mut self, entry: usize) -> Box<Thread> {
        let kstack = KStack::new();
        let context = unsafe {
            context::Context::new_kthread(entry, kstack.top(), riscv::register::satp::read().bits())
        };
        let content = unsafe { &mut *(context.addr as *mut context::ContextContent) };
        content.tf.x[10..18].clone_from_slice(&self.args[..]);
        Box::new(Thread { context, kstack })
    }
}

#[derive(Debug)]
pub(crate) struct KStack {
    bottom: usize,
}
impl KStack {
    fn new() -> Self {
        let bottom = unsafe {
            // be careful with hand-written RAII
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
        // println!("dropping thread {:?}", self);
        // we shouldn't drop zero inited KStack for the boot thread
        if self.bottom != 0 {
            unsafe {
                alloc::alloc::dealloc(
                    self.bottom as _,
                    alloc::alloc::Layout::from_size_align(KERNEL_STACK_SIZE, KERNEL_STACK_SIZE)
                        .expect("kernel stack layout illegal"),
                )
            }
        }
    }
}

pub(crate) fn init() {
    let mut boot = Thread::boot_thread();
    let mut new = Thread::new(thread_switch_test as usize);
    new.add_args(&[&*boot as *const _ as usize, &*new as *const _ as usize]);
    boot.switch(&mut new);
    println!("switched back from thread_switch_test!");
}

extern "C" fn thread_switch_test(boot: &mut Thread, myself: &mut Thread) {
    println!("I'm leaving soon, but I still want to say: Hello world!");
    myself.switch(boot);
}
