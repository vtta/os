use core::mem::size_of;

pub const MAX_PHYSICAL_MEMORY: usize = 1024 * 1024 * 128;
pub const PHYSICAL_MEMORY_BEGIN: usize = 0x8000_0000;
pub const PHYSICAL_MEMORY_END: usize = PHYSICAL_MEMORY_BEGIN + MAX_PHYSICAL_MEMORY;

pub const KERNEL_BEGIN_PADDR: usize = 0x8020_0000;
pub const KERNEL_BEGIN_VADDR: usize = 0xffff_ffff_c020_0000;
pub const KERNEL_HEAP_SIZE: usize = 0x80_0000;
pub const KERNEL_STACK_SIZE: usize = 0x8_0000;
pub const PHYSICAL_MEMORY_OFFSET: usize = KERNEL_BEGIN_VADDR - KERNEL_BEGIN_PADDR;

pub const PAGE_BITS: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_BITS;
pub const PAGE_ENTRIES: usize = PAGE_SIZE / size_of::<usize>();

pub const MAX_PHYSICAL_PAGES: usize = MAX_PHYSICAL_MEMORY / PAGE_SIZE;

// roughly 1 percent of CPU clock
pub const TIMEBASE: u64 = 100_000;
pub const TICKS_PER_TIME_SLICE: usize = 10;
