use crate::config::*;
use core::mem::size_of;
use spin::Mutex;

pub static FRAME_ALLOCATOR: Mutex<SegmentTreeAllocator> = Mutex::new(SegmentTreeAllocator::new());

// range update/query
pub struct SegmentTreeAllocator {
    // value indicates a page/range is fully occupied
    occupied: BitSet,
    cap: usize,
    len: usize,
}

const BITSET_UNIT_LEN: usize = 8 * size_of::<usize>();
const BITSET_BITS: usize = (PHYSICAL_MEMORY_END + PAGE_SIZE - 1) / PAGE_SIZE * 2;
const BITSET_UNITS: usize = (BITSET_BITS + BITSET_UNIT_LEN - 1) / BITSET_UNIT_LEN;

struct BitSet {
    // current size: 68KiB
    buf: [usize; BITSET_UNITS],
}

impl BitSet {
    const fn new() -> Self {
        Self {
            buf: [0; BITSET_UNITS],
        }
    }
}

impl BitSet {
    fn get(&self, idx: usize) -> bool {
        // assert!(idx < self.cap());
        self.buf[idx / BITSET_UNIT_LEN] & 1usize << (idx % BITSET_UNIT_LEN) > 0
    }
    fn set(&mut self, idx: usize) {
        // assert!(idx < self.cap());
        self.buf[idx / BITSET_UNIT_LEN] |= 1usize << (idx % BITSET_UNIT_LEN)
    }
    fn reset(&mut self, idx: usize) {
        // assert!(idx < self.cap());
        self.buf[idx / BITSET_UNIT_LEN] &= !(1usize << (idx % BITSET_UNIT_LEN))
    }
    const fn cap(&self) -> usize {
        BITSET_BITS
    }
}

/// loosely borrowed from: https://codeforces.com/blog/entry/18051
impl SegmentTreeAllocator {
    const fn new() -> Self {
        let bs = BitSet::new();
        Self {
            len: 0,
            cap: bs.cap() / 2,
            occupied: bs,
        }
    }

    pub(crate) fn init(&mut self, l: usize, r: usize) {
        for i in 0..l {
            self.occupied.set(i + self.cap);
        }
        for i in r..self.cap {
            self.occupied.set(i + self.cap);
        }
        for i in (1..self.cap).rev() {
            if self.occupied.get(i * 2) && self.occupied.get(i * 2 + 1) {
                self.occupied.set(i);
            }
        }
    }

    /// we do not guarantee the first allocated frame has the smallest ppn
    pub(crate) fn alloc(&mut self) -> Option<usize> {
        if self.len >= self.cap {
            return None;
        }
        // occupied[1] must be false
        let mut x = 1;
        while x < self.cap {
            x *= 2;
            if self.occupied.get(x) {
                x += 1
            }
        }

        self.len += 1;
        self.occupied.set(x);

        let mut p = x;
        while p > 1 {
            if self.occupied.get(p) && self.occupied.get(p ^ 1) {
                self.occupied.set(p / 2);
            }
            p /= 2;
        }

        Some(x - self.cap)
    }

    pub(crate) fn dealloc(&mut self, ppn: usize) {
        if !self.occupied.get(ppn) {
            return;
        }
        let mut p = ppn + self.cap;
        while p > 0 {
            self.occupied.reset(p);
            p /= 2;
        }
        self.len -= 1;
    }
}
