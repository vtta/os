use riscv::register::{scause::Scause, sscratch, sstatus, sstatus::Sstatus, stvec};

global_asm!(include_str!("trap.asm"));

#[repr(C)]
pub struct Frame {
    /// General registers
    pub x: [usize; 32],
    /// Supervisor status register
    pub sstatus: Sstatus,
    /// Supervisor exception program counter
    pub sepc: usize,
    /// Supervisor trap value
    pub stval: usize,
    /// Supervisor cause register: record the cause of exception/interrupt/trap
    pub scause: Scause,
}

pub fn init() {
    println!("+++ setting up trap handler +++");
    unsafe {
        extern "C" {
            fn __alltraps();
        }
        // keep our assumption that: (see trap/trap.asm)
        //  if interrupted from S-Mode, sscratch is always 0
        sscratch::write(0);
        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);
        sstatus::set_sie();
    }
}

#[no_mangle]
extern "C" fn rust_trap(tf: &mut Frame) {
    println!("+++ entered trap handler +++");
    // points to the next instruction
    // C extension compact some instructions
    tf.sepc += 2;
}
