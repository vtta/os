use crate::config::PAGE_SIZE;
use crate::mem::page::VPN;
use bit_field::BitField;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtAddr(usize);

impl From<usize> for VirtAddr {
    fn from(vaddr: usize) -> Self {
        #[cfg(riscv64)]
        {
            if vaddr.get_bit(47) {
                assert!(
                    vaddr.get_bits(48..64) == 0xFFFF,
                    "high bits should be sign extended"
                );
            } else {
                assert!(
                    vaddr.get_bits(48..64) == 0x0000,
                    "high bits should be sign extended"
                );
            }
        }

        // #[cfg(riscv32)]
        Self(vaddr)
    }
}

#[cfg(riscv32)]
impl VirtAddr {
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
impl VirtAddr {
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
    pub fn page_number(self) -> VPN {
        self.0.get_bits(12..64).into()
    }
}

impl VirtAddr {
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
}
