use riscv::register::{sie, time};

use crate::sbi::set_timer;

pub static mut TICKS: u64 = 0;
// roughly 1 percent of CPU clock
pub const TIMEBASE: u64 = 100_000;

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
