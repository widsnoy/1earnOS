#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use core::arch::asm;

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("try sret");
    unsafe {
        asm!("sret");
    }
    0
}
