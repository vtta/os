use crate::mem::addr::VirtAddr;
use crate::mem::frame::{self, Frame};
use crate::mem::page::base::PageTableBase;
use crate::mem::page::entry::{PageTableEntry, PageTableFlags, EF};
use crate::mem::page::map::{Error, Flush, Map};
use crate::mem::page::Page;

/// Three level page table with `Map` trait implemented.
#[cfg(riscv64)]
pub struct Sv39PageTable<'a> {
    root_table: &'a mut PageTableBase,
    // VA = PA + linear_offset
    linear_offset: usize,
}

#[cfg(riscv64)]
impl<'a> Sv39PageTable<'a> {
    pub fn new(root_table: &'a mut PageTableBase, linear_offset: usize) -> Self {
        Self {
            root_table,
            linear_offset,
        }
    }

    fn walk_or_insert(&mut self, vaddr: VirtAddr) -> Result<&mut PageTableBase, Error> {
        let p3_table = &mut self.root_table;
        let p2_table = if p3_table[vaddr.p3_index()].is_unused() {
            let frame = frame::alloc().ok_or(Error::FrameAllocationFailed)?;
            p3_table[vaddr.p3_index()].set(frame, EF::VALID);
            let p2_table: &mut PageTableBase = unsafe { frame.as_kernel_mut(self.linear_offset) };
            p2_table.clear();
            p2_table
        } else {
            unsafe {
                p3_table[vaddr.p3_index()]
                    .frame()
                    .as_kernel_mut(self.linear_offset)
            }
        };
        let p1_table = if p2_table[vaddr.p2_index()].is_unused() {
            let frame = frame::alloc().ok_or(Error::FrameAllocationFailed)?;
            p2_table[vaddr.p2_index()].set(frame, EF::VALID);
            let p1_table: &mut PageTableBase = unsafe { frame.as_kernel_mut(self.linear_offset) };
            p1_table.clear();
            p1_table
        } else {
            unsafe {
                p2_table[vaddr.p2_index()]
                    .frame()
                    .as_kernel_mut(self.linear_offset)
            }
        };
        Ok(p1_table)
    }

    fn walk(&mut self, vaddr: VirtAddr) -> Result<&mut PageTableBase, Error> {
        let p3_table = &mut self.root_table;
        let entry = &mut p3_table[vaddr.p3_index()];
        if !entry.is_unused() {
            return Err(Error::PageNotMapped);
        }

        let p2_table: &mut PageTableBase =
            unsafe { entry.frame().as_kernel_mut(self.linear_offset) };
        let entry = &mut p2_table[vaddr.p2_index()];
        if !entry.is_unused() {
            return Err(Error::PageNotMapped);
        }

        let p1_table: &mut PageTableBase =
            unsafe { entry.frame().as_kernel_mut(self.linear_offset) };
        Ok(p1_table)
    }
}

#[cfg(riscv64)]
impl<'a> Map for Sv39PageTable<'a> {
    /// Setup the mapping between `page` and `frame`
    fn map(&mut self, page: Page, frame: Frame, flags: PageTableFlags) -> Result<Flush, Error> {
        let p1_table = self.walk_or_insert(page.start_address())?;
        let entry = &mut p1_table[page.p1_index()];
        if entry.is_unused() {
            entry.set(frame, flags);
            Ok(Flush::new(page))
        } else {
            Err(Error::PageAlreadyMapped)
        }
    }

    /// Delete the mapping between `page` and corresponding frame
    ///
    /// Error if the mapping doesn't exist
    fn unmap(&mut self, page: Page) -> Result<(Frame, Flush), Error> {
        let entry = self.entry(page)?;
        let frame = entry.frame();
        entry.set_unused();
        Ok((frame, Flush::new(page)))
    }

    /// Walk down the page table and get the PTE for given page
    ///
    /// Error if the mapping doesn't exist
    fn entry(&mut self, page: Page) -> Result<&mut PageTableEntry, Error> {
        let p1_table = self.walk(page.start_address())?;
        Ok(&mut p1_table[page.p1_index()])
    }
}
