use crate::mem::addr::PhysAddr;
use crate::mem::frame::Frame;
use core::fmt;

#[derive(Copy, Clone)]
pub struct PageTableEntry(usize);

impl PageTableEntry {
    pub fn is_unused(self) -> bool {
        self.0 == 0
    }
    pub fn set_unused(&mut self) {
        self.0 = 0;
    }
    pub fn flags(self) -> PageTableFlags {
        PageTableFlags::from_bits_truncate(self.0)
    }
    pub fn ppn(self) -> usize {
        self.0 >> 10
    }
    pub fn addr(self) -> PhysAddr {
        PhysAddr::new(self.ppn() << 12)
    }
    pub fn frame(self) -> Frame {
        Frame::from(self.addr())
    }
    pub fn set(&mut self, frame: Frame, mut flags: PageTableFlags) {
        // U540 will raise page fault when accessing page with A=0 or D=0
        flags |= EF::ACCESSED | EF::DIRTY;
        self.0 = (frame.number() << 10) | flags.bits();
    }
    pub fn flags_mut(&mut self) -> &mut PageTableFlags {
        unsafe { &mut *(self as *mut _ as *mut PageTableFlags) }
    }
}

impl fmt::Debug for PageTableEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("PageTableEntry")
            .field("frame", &self.frame())
            .field("flags", &self.flags())
            .finish()
    }
}

bitflags! {
    /// Possible flags for a page table entry.
    pub struct PageTableFlags: usize {
        const VALID =       1;
        const READABLE =    1 << 1;
        const WRITABLE =    1 << 2;
        const EXECUTABLE =  1 << 3;
        const USER =        1 << 4;
        const GLOBAL =      1 << 5;
        const ACCESSED =    1 << 6;
        const DIRTY =       1 << 7;
        const RESERVED1 =   1 << 8;
        const RESERVED2 =   1 << 9;
    }
}

pub(crate) type EF = PageTableFlags;
