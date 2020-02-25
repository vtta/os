use core::fmt::{self, Write};

use crate::sbi;

struct Stdout;

pub fn putchar(ch: char) {
    sbi::console_putchar(ch as usize);
}

pub fn puts(s: &str) {
    for ch in s.chars() {
        putchar(ch);
    }
}

impl fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        puts(s);
        Ok(())
    }
}

#[allow(unused_must_use)]
pub fn _print(args: fmt::Arguments) {
    Stdout.write_fmt(args);
}

/// fake print macro with the same flavour in std
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::io::_print(format_args!($($arg)*));
    });
}

/// fake println macro with the same flavour in std
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
