use crate::mem::addr::VirtAddr;
use crate::mem::frame;
use crate::mem::page::table::PageTable;
use crate::mem::set::attrib::MemAttrib;

pub(crate) trait MemHandler {
    fn map(&self, tbl: &mut PageTable, va: VirtAddr, attr: MemAttrib);
    fn unmap(&self, tbl: &mut PageTable, va: VirtAddr);
}

pub(crate) struct Linear {
    offset: usize,
}
impl Linear {
    pub(crate) fn new(offset: usize) -> Self {
        Self { offset }
    }
}
impl MemHandler for Linear {
    /// cannot be used to map memory that are not original mapped before booting
    fn map(&self, tbl: &mut PageTable, va: VirtAddr, attr: MemAttrib) {
        tbl.map(va, (va.as_usize() - self.offset).into());
        attr.apply(
            tbl.entry(va)
                .unwrap_or_else(|| panic!("cannot find mapped entry {:#x}", va.as_usize())),
        )
    }

    fn unmap(&self, tbl: &mut PageTable, va: VirtAddr) {
        tbl.unmap(va)
    }
}

pub(crate) struct ByFrame;
impl ByFrame {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
impl MemHandler for ByFrame {
    fn map(&self, tbl: &mut PageTable, va: VirtAddr, attr: MemAttrib) {
        let frame = frame::alloc().expect("frame allocation failed");
        tbl.map(va, frame.start_address());
        attr.apply(tbl.entry(va).expect("cannot find mapped entry"));
    }

    fn unmap(&self, tbl: &mut PageTable, va: VirtAddr) {
        tbl.unmap(va)
    }
}
