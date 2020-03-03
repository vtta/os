use crate::config::*;
use crate::thread::Thread;
use alloc::boxed::Box;
use alloc::vec::Vec;

pub(crate) type TaskId = usize;
pub(crate) type ExitCode = usize;
#[derive(Debug, Copy, Clone)]
enum Status {
    /// there is no thread
    Uninitialized,
    /// waiting for resources
    Ready,
    /// running
    Running(TaskId),
    /// blocked
    Sleeping,
    /// finished
    Exited(ExitCode),
}

pub(crate) trait Scheduler {
    /// add a task for scheduling
    fn push(&mut self, _: TaskId);
    // pick one for running
    fn pick(&mut self) -> Option<TaskId>;
    // give up resources
    fn r#yield(&mut self, tid: TaskId);
    // timer interrupt will trigger this
    // true means time is up
    fn tick(&mut self) -> bool;
    // a thread has finished
    fn exit(&mut self, _: TaskId);
}

struct ThreadInfo {
    status: Status,
    thread: Option<Box<Thread>>,
}
impl Default for ThreadInfo {
    fn default() -> Self {
        Self {
            status: Status::Uninitialized,
            thread: None,
        }
    }
}
pub(crate) struct ThreadPool {
    threads: Vec<ThreadInfo>,
    scheduler: Box<dyn Scheduler + Send>,
}
impl ThreadPool {
    /// create a thread pool with given capacity and scheduler
    pub(crate) fn new(cap: usize, scheduler: Box<dyn Scheduler + Send>) -> Self {
        let mut threads = Vec::new();
        threads.resize_with(cap, Default::default);
        Self { threads, scheduler }
    }
    /// allocate a task id
    fn alloc(&self) -> Option<TaskId> {
        self.threads
            .iter()
            .enumerate()
            .find(|(_, tinfo)| match tinfo.status {
                Status::Uninitialized => true,
                Status::Exited(_) => true,
                _ => false,
            })
            .map(|(tid, _)| tid)
    }
    /// add a runnable thread, None -> Ready
    pub(crate) fn push(&mut self, t: Box<Thread>) {
        let tid = self.alloc().expect("cannot alloc TaskId");
        self.threads[tid].thread = Some(t);
        self.threads[tid].status = Status::Ready;
        self.scheduler.push(tid);
    }
    /// pick one to run, Ready -> Running
    pub(crate) fn pick(&mut self) -> Option<(TaskId, Box<Thread>)> {
        self.scheduler.pick().map(|tid| {
            let tinfo = &mut self.threads[tid];
            tinfo.status = Status::Running(tid);
            // when to put back???
            (tid, tinfo.thread.take().expect("no such thread exists"))
        })
    }
    /// give up the resources, Running -> Ready
    pub(crate) fn r#yield(&mut self, tid: TaskId, thread: Box<Thread>) {
        self.threads[tid].thread = Some(thread);
        if let Status::Running(_) = self.threads[tid].status {
            self.threads[tid].status = Status::Ready;
            self.scheduler.r#yield(tid);
        }
    }
    pub(crate) fn tick(&mut self) -> bool {
        self.scheduler.tick()
    }
    pub(crate) fn exit(&mut self, tid: TaskId, code: usize) {
        self.threads[tid].status = Status::Exited(code);
        self.scheduler.exit(tid);
    }
}

#[derive(Copy, Clone)]
struct RRNode {
    tid: TaskId,
    time: usize,
}
impl RRNode {
    fn new(tid: TaskId, time: usize) -> Self {
        Self { tid, time }
    }
}
/// round robin scheduler, first come first serve
pub(crate) struct RRScheduler {
    queue: alloc::collections::VecDeque<RRNode>,
}
impl Scheduler for RRScheduler {
    // add a new thread for scheduling
    fn push(&mut self, tid: TaskId) {
        self.queue.push_back(RRNode::new(tid, TICKS_PER_TIME_SLICE))
    }
    // pick the head node to schedule if not empty
    fn pick(&mut self) -> Option<TaskId> {
        self.queue.front().cloned().map(|node| node.tid)
    }
    // give up resources
    fn r#yield(&mut self, tid: TaskId) {
        if let Some((idx, &node)) = self
            .queue
            .iter()
            .enumerate()
            .find(|(_id, node)| node.tid == tid)
        {
            self.queue.remove(idx);
            self.queue.push_back(node);
        }
    }
    fn tick(&mut self) -> bool {
        if let Some(mut front) = self.queue.pop_front() {
            front.time -= 1;
            if front.time == 0 {
                true
            } else {
                self.queue.push_front(front);
                false
            }
        } else {
            false
        }
    }
    fn exit(&mut self, tid: TaskId) {
        if let Some((idx, _node)) = self
            .queue
            .iter()
            .enumerate()
            .find(|(_id, node)| node.tid == tid)
        {
            self.queue.remove(idx);
        }
    }
}
impl RRScheduler {
    pub(crate) fn new() -> Self {
        let queue = alloc::collections::VecDeque::new();
        Self { queue }
    }
}
