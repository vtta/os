use crate::config::PAGE_SIZE;
use crate::mem::addr::VirtAddr;
use core::ops::Deref;

mod entry;
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
}

impl Deref for Page {
    type Target = VirtAddr;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
