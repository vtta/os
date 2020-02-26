// #![deny(missing_docs)]
#![no_std]
#![feature(asm)]
#![feature(global_asm)]

//! An OS in rust!

#[macro_use]
pub mod io;

mod boot;
mod lang_item;
mod sbi;
mod trap;
