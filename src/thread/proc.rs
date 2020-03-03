use crate::thread::sched::{TaskId, ThreadPool};
use crate::thread::Thread;
use crate::trap;
use alloc::boxed::Box;
use core::borrow::BorrowMut;
use spin::Mutex;

pub(crate) static mut CPU: Processor = Processor::new();

struct ProcessorInner {
    pool: Box<ThreadPool>,
    idle: Box<Thread>,
    cur: Option<(TaskId, Box<Thread>)>,
}
pub(crate) struct Processor {
    inner: Option<ProcessorInner>,
}
impl Processor {
    const fn new() -> Self {
        Self { inner: None }
    }
    pub(crate) fn init(&mut self, pool: Box<ThreadPool>, idle: Box<Thread>) {
        *self.inner.borrow_mut() = Some(ProcessorInner {
            pool,
            idle,
            cur: None,
        })
    }
    pub(crate) fn push(&mut self, thread: Box<Thread>) {
        self.inner().pool.push(thread);
    }
    fn inner(&mut self) -> &mut ProcessorInner {
        self.inner
            .borrow_mut()
            .as_mut()
            .expect("processor not initialized")
    }
    #[inline(never)]
    pub(crate) fn idle(&mut self) -> ! {
        loop {
            // came back to life, turn off interrupt
            trap::disable();
            let inner = self.inner();
            if let Some(t) = inner.pool.pick() {
                inner.cur = Some(t);
                let cur = inner
                    .cur
                    .as_mut()
                    .expect("I just put it in there, this should not happen!");
                println!(">>> switching to thread {}", cur.0);
                inner.idle.switch(&mut cur.1);
                println!("<<< switched back to idle thread");
                let (tid, thread) = self
                    .inner()
                    .cur
                    .take()
                    .expect("I just put it in there, this should not happen!");
                self.inner().pool.r#yield(tid, thread);
            } else {
                println!("[idle] sleeping");
                // sleep and wait for interrupt
                trap::enable_and_wait();
            }
        }
    }
    pub(crate) fn tick(&mut self) {
        let inner = self.inner();
        println!("testing time slice");
        if let Some((_, thread)) = &mut inner.cur {
            // time's up
            if inner.pool.tick() {
                println!("time is up");
                let sstatus = trap::disable();
                thread.switch(&mut inner.idle);
                trap::restore(sstatus);
            } else {
                println!("you got some more time");
            }
        }
    }
    pub(crate) fn exit(&mut self, code: usize) -> ! {
        trap::disable();
        let inner = self.inner();
        let (tid, thread) = inner.cur.as_mut().expect("thread to exist must be running");
        println!("thread {} exited, with code {}", tid, code);
        inner.pool.exit(*tid, code);
        thread.switch(&mut inner.idle);
        assert!(false, "this should not be reachable");
        loop {}
    }
    pub(crate) fn run(&mut self) {
        Thread::boot_thread().switch(&mut self.inner().idle);
    }
}
