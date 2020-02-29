// #![deny(missing_docs)]
#![no_std]
#![feature(asm)]
#![feature(const_fn)]
#![feature(global_asm)]
#![feature(naked_functions)]
#![feature(alloc_error_handler)]

//! An OS in rust!

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate alloc;

#[macro_use]
pub mod io;

mod boot;
mod config;
mod lang_item;
mod mem;
mod sbi;
mod thread;
mod trap;
