use crate::config::*;
use crate::sbi::set_timer;
use riscv::register::{sie, time};

pub static mut TICKS: u64 = 0;

pub fn init() {
    println!("+++ setting up timer +++");
    unsafe {
        TICKS = 0;
        sie::set_stimer();
    }
    // only after setting the timer for the first time
    // the machine timer interrupt bit would be set (mie[7])
    set(TIMEBASE);
}

pub fn set(delta: u64) {
    set_timer(get() + delta);
}

fn get() -> u64 {
    time::read64()
}
