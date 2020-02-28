use crate::config::PAGE_SIZE;
use crate::mem::addr::PhysAddr;
use alloc::FRAME_ALLOCATOR;
use core::ops::Deref;

pub(crate) mod alloc;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame(PhysAddr);

pub(crate) fn init(l: usize, r: usize) {
    FRAME_ALLOCATOR.lock().init(l, r);
}

pub(crate) fn alloc() -> Option<Frame> {
    FRAME_ALLOCATOR.lock().alloc().map(Frame::from_ppn)
}

pub(crate) fn dealloc(f: Frame) {
    FRAME_ALLOCATOR.lock().dealloc(f.page_number())
}

#[allow(clippy::many_single_char_names)]
pub(crate) fn test(l: usize, r: usize) {
    println!("free pages count {}", r - l);
    mem_test_full(l, r);
    let a = alloc();
    assert!(a.is_some());
    let b = alloc();
    assert!(b.is_some());
    assert_ne!(a, b);
    let c = alloc();
    assert!(c.is_some());
    assert_ne!(b, c);
    let d = alloc();
    assert!(d.is_some());
    assert_ne!(c, d);
    dealloc(c.unwrap());
    let e = alloc();
    assert!(e.is_some());
    assert_eq!(c, e);
    let f = alloc();
    assert!(f.is_some());
    assert_ne!(e, f);
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
        dealloc(Frame::from_ppn(i));
    }
}

impl From<PhysAddr> for Frame {
    fn from(pa: PhysAddr) -> Self {
        Self(pa)
    }
}

impl Frame {
    pub fn from_ppn(ppn: usize) -> Self {
        Self((ppn * PAGE_SIZE).into())
    }

    pub fn number(self) -> usize {
        self.page_number()
    }

    pub fn start_address(self) -> PhysAddr {
        (self.as_usize() & !(PAGE_SIZE - 1)).into()
    }
}

impl Deref for Frame {
    type Target = PhysAddr;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
