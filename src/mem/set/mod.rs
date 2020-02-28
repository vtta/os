use crate::config::*;
use crate::mem::addr::VirtAddr;
use crate::mem::page::table::PageTable;
use crate::mem::set::area::MemArea;
use crate::mem::set::attrib::MemAttrib;
use crate::mem::set::handler::MemHandler;
use alloc::boxed::Box;
use alloc::vec::Vec;

mod area;
pub(crate) mod attrib;
pub(crate) mod handler;

pub(crate) struct MemSet<'a> {
    areas: Vec<MemArea>,
    tbl: PageTable<'a>,
}
impl<'a> MemSet<'a> {
    pub(crate) fn new() -> Self {
        let tbl = PageTable::bare();
        let areas: Vec<MemArea> = Vec::new();
        let mut set = Self { tbl, areas };
        set.kmap();
        set
    }
    pub(crate) fn push(
        &mut self,
        begin: VirtAddr,
        end: VirtAddr,
        handler: impl MemHandler + 'static,
        attrib: MemAttrib,
    ) {
        println!(
            "pushing memory area [{:#x}, {:#x})",
            begin.as_usize(),
            end.as_usize()
        );
        assert!(begin <= end, "invalid memory region");
        assert!(
            !self.is_overlap(begin, end),
            &format!(
                "memory areas overlap [{:#x}, {:#x})",
                begin.as_usize(),
                end.as_usize()
            )
        );
        let area = MemArea::new(begin, end, Box::new(handler), attrib);
        area.map(&mut self.tbl);
        self.areas.push(area);
    }
    pub(crate) unsafe fn activate(&self) {
        self.tbl.activate()
    }
    pub(crate) fn is_overlap(&self, begin: VirtAddr, end: VirtAddr) -> bool {
        0 < self
            .areas
            .iter()
            .filter(|a| a.is_overlap(begin, end))
            .count()
    }
    fn kmap(&mut self) {
        extern "C" {
            fn stext();
            fn etext();
            fn srodata();
            fn erodata();
            fn sdata();
            fn edata();
            fn sbss();
            fn ebss();
            fn end();
        }
        // kernel text R-X
        self.push(
            (stext as usize).into(),
            (etext as usize).into(),
            handler::Linear::new(PHYSICAL_MEMORY_OFFSET),
            attrib::MemAttrib::new().readable(true).executable(true),
        );
        // kernel rodata R--
        self.push(
            (srodata as usize).into(),
            (erodata as usize).into(),
            handler::Linear::new(PHYSICAL_MEMORY_OFFSET),
            attrib::MemAttrib::new().readable(true),
        );
        // kernel data RW-
        self.push(
            (sdata as usize).into(),
            (edata as usize).into(),
            handler::Linear::new(PHYSICAL_MEMORY_OFFSET),
            attrib::MemAttrib::new().readable(true).writable(true),
        );
        // kernel bss RW-
        self.push(
            (sbss as usize).into(),
            (ebss as usize).into(),
            handler::Linear::new(PHYSICAL_MEMORY_OFFSET),
            attrib::MemAttrib::new().readable(true).writable(true),
        );
        // kernel remapped physical space RW-
        self.push(
            // align to PAGE_SIZE
            ((end as usize / PAGE_SIZE + 1) * PAGE_SIZE).into(),
            (PHYSICAL_MEMORY_END + PHYSICAL_MEMORY_OFFSET).into(),
            handler::Linear::new(PHYSICAL_MEMORY_OFFSET),
            attrib::MemAttrib::new().readable(true).writable(true),
        );
    }
}
