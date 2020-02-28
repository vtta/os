use crate::mem::page::entry::EF;
use crate::mem::page::table::PageEntry;

#[derive(Copy, Clone, Debug)]
pub(crate) struct MemAttrib {
    readable: bool,
    writable: bool,
    executable: bool,
    user: bool,
}

impl MemAttrib {
    pub(crate) fn new() -> Self {
        Self {
            readable: false,
            writable: false,
            executable: false,
            user: false,
        }
    }
    pub(crate) fn readable(mut self, r: bool) -> Self {
        self.readable = r;
        self
    }
    pub(crate) fn writable(mut self, w: bool) -> Self {
        self.writable = w;
        self
    }
    pub(crate) fn executable(mut self, x: bool) -> Self {
        self.executable = x;
        self
    }
    pub(crate) fn user(mut self, u: bool) -> Self {
        self.user = u;
        self
    }
    pub(crate) fn apply(self, pe: PageEntry) {
        let flags = pe.pte.flags_mut();
        flags.set(EF::VALID, true);
        flags.set(EF::READABLE, self.readable);
        flags.set(EF::WRITABLE, self.writable);
        flags.set(EF::EXECUTABLE, self.executable);
    }
}
