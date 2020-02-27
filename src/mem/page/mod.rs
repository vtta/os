use crate::config::PAGE_SIZE;
use crate::mem::addr::VirtAddr;
use core::ops::Deref;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Page(VirtAddr);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VPN(usize);

impl From<VirtAddr> for Page {
    fn from(va: VirtAddr) -> Self {
        Self(va)
    }
}

impl Page {
    pub fn from_vpn(vpn: VPN) -> Self {
        Self((*vpn * PAGE_SIZE).into())
    }
}

impl Deref for Page {
    type Target = VirtAddr;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<usize> for VPN {
    fn from(vpn: usize) -> Self {
        Self(vpn)
    }
}

impl Deref for VPN {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
