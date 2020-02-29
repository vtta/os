use crate::trap;

pub(crate) struct Context {
    /// context content has already been saved on stack
    sp: usize,
}
impl Context {
    #[naked]
    #[inline(never)]
    pub(crate) unsafe extern "C" fn switch(&mut self, target: &mut Context) {
        asm!(include_str!("switch.asm"):::: "volatile")
    }
}

#[repr(C)]
pub(crate) struct ContextContent {
    pub(crate) ra: usize,
    satp: usize,
    s: [usize; 12],
    tf: trap::Frame,
}
