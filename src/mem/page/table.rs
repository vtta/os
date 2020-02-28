use crate::config::PHYSICAL_MEMORY_OFFSET;
use crate::mem::addr::{PhysAddr, VirtAddr};
use crate::mem::frame::Frame;
use crate::mem::page::base::PageTableBase;
use crate::mem::page::entry::{PageTableEntry, EF};
use crate::mem::page::map::Map;
use crate::mem::page::sv39::Sv39PageTable;
use crate::mem::page::Page;
use riscv::register::satp::{self, Satp};

pub(crate) struct PageTable<'a> {
    sv39: Sv39PageTable<'a>,
    root_frame: Frame,
}

pub(crate) struct PageEntry<'a> {
    pub(crate) pte: &'a mut PageTableEntry,
    page: Page,
}

impl<'a> PageTable<'a> {
    pub fn bare() -> Self {
        let frame = crate::mem::frame::alloc().expect("frame allocation failed");
        let table: &mut PageTableBase = unsafe { frame.as_kernel_mut(PHYSICAL_MEMORY_OFFSET) };
        table.clear();
        Self {
            sv39: Sv39PageTable::new(table, PHYSICAL_MEMORY_OFFSET),
            root_frame: frame,
        }
    }

    pub fn map(&mut self, va: VirtAddr, pa: PhysAddr) {
        let flags = EF::VALID | EF::READABLE | EF::WRITABLE;
        self.sv39
            .map(va.into(), pa.into(), flags)
            .map(|f| f.flush())
            .ok()
            .unwrap_or_else(|| panic!("map failed {:#x} {:#x}", va.as_usize(), pa.as_usize()));
    }
    pub fn unmap(&mut self, va: VirtAddr) {
        let page = Page(va);
        self.sv39
            .unmap(page)
            .map(|(frame, flush)| {
                crate::mem::frame::dealloc(frame);
                flush.flush();
            })
            .ok()
            .expect("unmap failed");
    }
    pub fn entry(&mut self, va: VirtAddr) -> Option<PageEntry> {
        let page = Page(va);
        self.sv39
            .entry(page)
            .map(|pte| PageEntry { pte, page })
            .ok()
    }
    pub fn activate(&self) {
        let active = Self::active().bits();
        let own = self.satp();
        if active != own {
            println!(
                "+++ switching page table from {:#x} to {:#x} +++",
                active, own
            );
            unsafe {
                asm!("csrw satp, $0" :: "r"(own) :: "volatile");
            }
            self.flush();
        }
    }
    pub fn satp(&self) -> usize {
        8 << 60 | self.root_frame.number()
    }
    pub fn flush(&self) {
        unsafe {
            riscv::asm::sfence_vma_all();
        }
    }
    pub fn active() -> Satp {
        satp::read()
    }
}

impl<'a> PageEntry<'a> {
    pub fn flush(&self) {
        unsafe {
            riscv::asm::sfence_vma(0, self.page.start_address().as_usize());
        }
    }
    pub fn valid(&self) -> bool {
        self.pte.flags().contains(EF::VALID)
    }
    pub fn readable(&self) -> bool {
        self.pte.flags().contains(EF::READABLE)
    }
    pub fn writable(&self) -> bool {
        self.pte.flags().contains(EF::WRITABLE)
    }
    pub fn executable(&self) -> bool {
        self.pte.flags().contains(EF::EXECUTABLE)
    }
    pub fn user(&self) -> bool {
        self.pte.flags().contains(EF::USER)
    }
    pub fn global(&self) -> bool {
        self.pte.flags().contains(EF::GLOBAL)
    }
    pub fn accessed(&self) -> bool {
        self.pte.flags().contains(EF::ACCESSED)
    }
    pub fn dirty(&self) -> bool {
        self.pte.flags().contains(EF::DIRTY)
    }
}
