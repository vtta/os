// #![deny(missing_docs)]
#![no_std]
#![feature(asm)]
#![feature(global_asm)]
#![feature(const_fn)]

//! An OS in rust!

#[macro_use]
pub mod io;

mod boot;
mod config;
mod lang_item;
mod mem;
mod sbi;
mod trap;
