use crate::config::PAGE_SIZE;
use crate::mem::addr::PhysAddr;
use alloc::{FrameAlloc, FRAME_ALLOCATOR};
use core::ops::Deref;

mod alloc;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame(PhysAddr);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PPN(usize);

pub fn init(l: PPN, r: PPN) {
    println!("+++ setting up physical memory +++");
    FRAME_ALLOCATOR.lock().init(l, r);
    mem_test(l, r);
}

pub fn alloc() -> Option<Frame> {
    FRAME_ALLOCATOR.lock().alloc().map(Frame::from_ppn)
}

pub fn dealloc(f: Frame) {
    FRAME_ALLOCATOR.lock().dealloc(f.page_number())
}

fn mem_test(l: PPN, r: PPN) {
    let l = *l;
    let r = *r;
    println!("free pages count {}", r - l);
    mem_test_full(l, r);
    println!("alloc {:x?}", alloc());
    let f = alloc();
    println!("alloc {:x?}", f);
    println!("alloc {:x?}", alloc());
    println!("dealloc {:x?}", f);
    dealloc(f.unwrap());
    println!("alloc {:x?}", alloc());
    println!("alloc {:x?}", alloc());
}

fn mem_test_full(l: usize, r: usize) {
    let mut cnt = 0usize;
    for _ in l..r {
        let f = alloc();
        assert_eq!(f.is_some(), true);
        cnt += 1;
        // println!("{}/{} {:x}", cnt, r - l, f.unwrap().as_usize());
    }
    assert_eq!(cnt, r - l);
    for i in l..r {
        dealloc(Frame::from_ppn(i.into()));
    }
}

impl From<PhysAddr> for Frame {
    fn from(pa: PhysAddr) -> Self {
        Self(pa)
    }
}

impl Frame {
    pub fn from_ppn(ppn: PPN) -> Self {
        Self((*ppn * PAGE_SIZE).into())
    }
}

impl Deref for Frame {
    type Target = PhysAddr;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<usize> for PPN {
    fn from(ppn: usize) -> Self {
        Self(ppn)
    }
}

impl Deref for PPN {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
