use crate::config::PAGE_ENTRIES;
use crate::mem::frame::Frame;
use crate::mem::page::entry::{PageTableEntry, EF};
use bitflags::_core::ops::{Deref, DerefMut};

pub struct PageTableBase {
    entries: [PageTableEntry; PAGE_ENTRIES],
}

impl PageTableBase {
    /// Clear all entries
    pub fn clear(&mut self) {
        for e in self.entries.iter_mut() {
            e.set_unused();
        }
    }

    /// Parameter `frame` is the actual physical frame where the root page table resides,
    ///  it can be anywhere in the main memory.
    /// Denote `recursive index` by K, then virtual address of the root page table is
    ///  (K, K+1, 0) in Sv32, and (K, K, K+1, 0) in Sv39, and (K, K, K, K+1, 0) in Sv48.
    pub fn recursive(&mut self, frame: Frame, k: usize) {
        // RWX = 000 means pointing to next level, and next level is recursively pointing to self
        self[k].set(frame, EF::VALID);
        self[k + 1].set(frame, EF::VALID | EF::READABLE | EF::WRITABLE);
    }
}

impl Deref for PageTableBase {
    type Target = [PageTableEntry; PAGE_ENTRIES];

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}

impl DerefMut for PageTableBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entries
    }
}
