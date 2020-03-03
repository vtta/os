use crate::config::*;
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
}

#[no_mangle]
extern "C" fn rust_trap(tf: &mut Frame) {
    println!("+++ entered trap handler +++");
    println!(
        "sstatus {:#x} sepc {:#x} scause {:#x} stval {:#x}",
        tf.sstatus,
        tf.sepc,
        tf.scause.bits(),
        tf.stval
    );
    match tf.scause.cause() {
        Trap::Exception(Exception::Breakpoint) => breakpoint(tf),
        Trap::Interrupt(Interrupt::SupervisorTimer) => stimer(),
        Trap::Exception(Exception::InstructionPageFault) => page_fault(tf),
        Trap::Exception(Exception::LoadPageFault) => page_fault(tf),
        Trap::Exception(Exception::StorePageFault) => page_fault(tf),
        _ => panic!("+++ unhandled trap +++"),
    }
    println!("returning from timer rust_trap");
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
        println!("+++ {} ticks +++", timer::TICKS);
        if timer::TICKS == 1000 {
            timer::TICKS = 0;
        }
    }
    timer::set(TIMEBASE);
    crate::thread::tick();
    println!("returning from timer interrupt");
}

fn page_fault(tf: &mut Frame) {
    println!(
        "{:?} va = {:#x} instruction = {:#x}",
        tf.scause.cause(),
        tf.stval,
        tf.sepc
    );
    panic!("page fault!");
}

#[inline(always)]
pub(crate) fn disable() -> usize {
    let mut sstatus: usize;
    unsafe { asm!("csrci sstatus, 1 << 1":"=r"(sstatus):::"volatile") }
    sstatus
}
#[inline(always)]
pub(crate) fn restore(flags: usize) {
    unsafe { asm!("csrs sstatus, $0"::"r"(flags)::"volatile") }
}
#[inline(always)]
fn enable() {
    restore(1 << 1);
}
#[inline(always)]
pub(crate) fn wait() {
    unsafe { asm!("wfi"::::"volatile") }
}
#[inline(always)]
pub(crate) fn enable_and_wait() {
    unsafe {
        asm!("csrsi sstatus, 1 << 1; wfi" :::: "volatile");
    }
}
