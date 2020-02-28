use crate::config::PAGE_SIZE;
use crate::mem::addr::VirtAddr;
use core::ops::Deref;

mod base;
pub(crate) mod entry;
mod map;
mod sv39;
pub(crate) mod table;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Page(VirtAddr);

impl From<VirtAddr> for Page {
    fn from(va: VirtAddr) -> Self {
        Self(va)
    }
}

impl Page {
    pub fn from_vpn(vpn: usize) -> Self {
        Self((vpn * PAGE_SIZE).into())
    }

    pub fn start_address(self) -> VirtAddr {
        (self.as_usize() & !(PAGE_SIZE - 1)).into()
    }
}

impl Deref for Page {
    type Target = VirtAddr;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub(crate) struct PageRange {
    begin: usize,
    end: usize,
}
impl PageRange {
    pub(crate) fn new(l: VirtAddr, r: VirtAddr) -> Self {
        Self {
            begin: l.as_usize() / PAGE_SIZE,
            end: (r.as_usize() - 1) / PAGE_SIZE + 1,
        }
    }
}
impl Iterator for PageRange {
    type Item = Page;

    fn next(&mut self) -> Option<Self::Item> {
        if self.begin < self.end {
            let page = Page::from_vpn(self.begin);
            self.begin += 1;
            Some(page)
        } else {
            None
        }
    }
}
