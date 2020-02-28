use crate::mem::addr::PhysAddr;
use crate::mem::addr::VirtAddr;
use crate::mem::frame::{self, Frame};
use crate::mem::page::entry::{PageTableEntry, PageTableFlags};
use crate::mem::page::Page;

pub(crate) trait Map {
    fn map(&mut self, page: Page, frame: Frame, flags: PageTableFlags) -> Result<Flush, Error>;

    /// Removes a mapping from the page table and returns the frame that used to be mapped.
    ///
    /// Note that no page tables or pages are deallocated, caller should handle deallocation.
    fn unmap(&mut self, page: Page) -> Result<(Frame, Flush), Error>;

    /// Get the reference of the specified `page` entry
    fn entry(&mut self, page: Page) -> Result<&mut PageTableEntry, Error>;

    /// Updates the flags of an existing mapping.
    fn update_flags(&mut self, page: Page, flags: PageTableFlags) -> Result<Flush, Error> {
        self.entry(page).map(|e| {
            *e.flags_mut() = flags;
            Flush::new(page)
        })
    }

    /// Return the frame that the specified page is mapped to.
    fn translate(&mut self, page: Page) -> Option<Frame> {
        self.entry(page)
            .ok()
            .and_then(|e| if e.is_unused() { None } else { Some(e.frame()) })
    }

    /// Maps the given frame to the virtual page with the same address.
    fn identity(&mut self, frame: Frame, flags: PageTableFlags) -> Result<Flush, Error> {
        let page = Page::from(VirtAddr::new(frame.start_address().as_usize()));
        self.map(page, frame, flags)
    }
}

pub(crate) enum Error {
    /// An additional frame was needed for the mapping process,
    /// but the frame allocator returned `None`.
    FrameAllocationFailed,
    /// The given page is already mapped to a physical frame.
    PageAlreadyMapped,

    /// An upper level page table entry has the `HUGE_PAGE` flag set, which means that the
    /// given page is part of an already mapped huge page or can't be freed individually.
    ParentEntryHugePage,

    /// The given page is not mapped to a physical frame.
    PageNotMapped,
    /// The page table entry for the given page points to an invalid physical address.
    InvalidFrameAddress(PhysAddr),
}

#[must_use = "Page Table changes must be flushed or ignored."]
pub(crate) struct Flush(Page);

impl Flush {
    /// Create a new flush promise
    pub fn new(page: Page) -> Self {
        Self(page)
    }

    /// Flush the page from the TLB to ensure that the newest mapping is used.
    pub fn flush(self) {
        unsafe {
            riscv::asm::sfence_vma(0, self.0.start_address().as_usize());
        }
    }

    /// Don't flush the TLB and silence the “must be used” warning.
    pub fn ignore(self) {}
}
