use crate::config::PAGE_SIZE;
use bit_field::BitField;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysAddr(usize);

impl From<usize> for PhysAddr {
    fn from(paddr: usize) -> Self {
        #[cfg(riscv64)]
        assert!(paddr.get_bits(32..64) == 0, "pa 32..64 not zero?");
        Self(paddr)
    }
}

#[cfg(riscv32)]
impl PhysAddr {
    pub fn p2_index(self) -> usize {
        self.0.get_bits(22..32)
    }
    pub fn p1_index(self) -> usize {
        return self.0.get_bits(12..22);
    }
    pub fn page_number(self) -> usize {
        self.0.get_bits(12..32)
    }
}

#[cfg(riscv64)]
impl PhysAddr {
    pub fn p4_index(self) -> usize {
        self.0.get_bits(39..48)
    }
    pub fn p3_index(self) -> usize {
        self.0.get_bits(30..39)
    }
    pub fn p2_index(self) -> usize {
        self.0.get_bits(21..30)
    }
    pub fn p1_index(self) -> usize {
        self.0.get_bits(12..21)
    }
    pub fn page_number(self) -> usize {
        self.0.get_bits(12..64)
    }
}

impl PhysAddr {
    pub fn new(addr: usize) -> Self {
        addr.into()
    }
    pub fn as_usize(self) -> usize {
        self.0
    }
    pub fn page_aligned(self) -> Self {
        Self((self.as_usize() / PAGE_SIZE) * PAGE_SIZE)
    }
    pub fn page_offset(self) -> usize {
        self.0.get_bits(0..12)
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub(crate) unsafe fn as_mut<'a, 'b, T>(&'a self) -> &'b mut T {
        &mut *(self.0 as *mut T)
    }
    /// convert to a virtual address in the kernel space
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub(crate) unsafe fn as_kernel_mut<'a, 'b, T>(&'a self, linear_offset: usize) -> &'b mut T {
        &mut *((self.0 + linear_offset) as *mut T)
    }
}
