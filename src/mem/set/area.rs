use crate::mem::addr::VirtAddr;
use crate::mem::set::attrib::MemAttrib;
use crate::mem::set::handler::MemHandler;

use crate::mem::page::table::PageTable;
use crate::mem::page::PageRange;
use alloc::boxed::Box;

pub(crate) struct MemArea {
    begin: VirtAddr,
    end: VirtAddr,
    handler: Box<dyn MemHandler>,
    attrib: MemAttrib,
}
impl MemArea {
    pub(crate) fn new(
        begin: VirtAddr,
        end: VirtAddr,
        handler: Box<dyn MemHandler>,
        attrib: MemAttrib,
    ) -> Self {
        Self {
            begin,
            end,
            handler,
            attrib,
        }
    }
    pub(crate) fn map(&self, tbl: &mut PageTable) {
        for page in PageRange::new(self.begin, self.end) {
            self.handler.map(tbl, page.start_address(), self.attrib);
        }
    }
    pub(crate) fn unmap(&self, tbl: &mut PageTable) {
        for page in PageRange::new(self.begin, self.end) {
            self.handler.unmap(tbl, page.start_address());
        }
    }
    pub(crate) fn is_overlap(&self, begin: VirtAddr, end: VirtAddr) -> bool {
        !(end.page_number() <= self.begin.page_number()
            || self.end.page_number() <= begin.page_number())
    }
}
