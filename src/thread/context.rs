use crate::trap;
use bit_field::BitField;

#[repr(C)]
#[derive(Debug)]
pub(crate) struct Context {
    /// context content has already been saved on stack
    ///
    /// addr points to the beginning of context content on stack
    pub(crate) addr: usize,
}
impl Context {
    #[naked]
    #[inline(never)]
    pub(crate) unsafe extern "C" fn switch(&mut self, _target: &mut Context) {
        asm!(include_str!("switch.asm"):::: "volatile")
    }
    /// create content of the new kernel context and push it onto stack
    pub(crate) unsafe fn new_kthread(sepc: usize, sp: usize, satp: usize) -> Self {
        let content = ContextContent::new_kthread(sepc, sp, satp);
        // allocate space on stack
        let ptr = (sp as *mut ContextContent).sub(1);
        // put content there
        *ptr = content;
        // cast to a Context wrapper
        Self { addr: ptr as usize }
    }
}

#[repr(C)]
pub(crate) struct ContextContent {
    pub(crate) ra: usize,
    satp: usize,
    s: [usize; 12],
    pub(crate) tf: trap::Frame,
}

impl ContextContent {
    /// construct a new kernel thread
    ///
    /// what we need:
    /// program => sepc
    /// stack => sp
    /// page table => satp
    fn new_kthread(sepc: usize, sp: usize, satp: usize) -> Self {
        // although we use `__trapret` to set up the stack and registers
        // it doesn't mean that it has anything to do with interrupt
        extern "C" {
            fn __trapret();
        }
        let ra = __trapret as usize;
        let s = [0usize; 12];
        let mut sstatus = 0usize;
        unsafe {
            asm!("csrr $0, sstatus": "=r"(sstatus):::"volatile");
        }
        // mark SPP (previous privilege) bit as supervisor
        sstatus.set_bit(8, true);
        // SPIE (previous interrupt enable)
        sstatus.set_bit(5, true);
        // SIE bit
        sstatus.set_bit(2, false);
        // no drop or pointer related to trap frame
        // GOSH!!! That's a relief
        let mut tf: trap::Frame = unsafe { core::mem::zeroed() };
        tf.sstatus = sstatus;
        tf.x[2] = sp;
        tf.sepc = sepc;
        Self { ra, satp, s, tf }
    }
}
