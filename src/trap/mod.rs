use riscv::register::{
    scause::{Exception, Interrupt, Scause, Trap},
    sscratch, sstatus, stvec,
};

global_asm!(include_str!("trap.asm"));

pub mod timer;

#[repr(C)]
pub struct Frame {
    /// General registers
    pub x: [usize; 32],
    /// Supervisor status register
    pub sstatus: usize,
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
        // sie::clear_stimer();
        extern "C" {
            fn __alltraps();
        }
        // keep our assumption that: (see trap/trap.asm)
        //  if interrupted from S-Mode, sscratch is always 0
        sscratch::write(0);
        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);
        sstatus::set_sie();
    }
    timer::init();
}

#[no_mangle]
extern "C" fn rust_trap(tf: &mut Frame) {
    // println!("+++ entered trap handler +++");
    match tf.scause.cause() {
        Trap::Exception(Exception::Breakpoint) => breakpoint(tf),
        Trap::Interrupt(Interrupt::SupervisorTimer) => stimer(),
        _ => panic!("+++ unhandled trap +++"),
    }
}

fn breakpoint(tf: &mut Frame) {
    println!("a breakpoint set at {:#x}", tf.sepc);
    // points to the next instruction
    // C extension compact some instructions
    tf.sepc += 2;
}

fn stimer() {
    unsafe {
        timer::TICKS += 1;
        if timer::TICKS == 1000 {
            timer::TICKS = 0;
            println!("+++ 1000 ticks +++")
        }
    }
    timer::set(timer::TIMEBASE);
}
