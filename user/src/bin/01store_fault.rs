#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("非法内存写入");
    unsafe {
        core::ptr::null_mut::<u8>().write_volatile(0);
    }
    0
}
