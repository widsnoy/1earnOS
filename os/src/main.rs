#![no_std]
#![no_main]

#[macro_use]
mod console;

mod lang_items;
mod sbi;

use core::arch::global_asm;

//use sbi::console_putchar;
global_asm!(include_str!("entry.asm"));

#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    clear_bss();
    let x:Option<u32> = Option::None;
    print!("{}", x.unwrap());
    loop{}
}
fn clear_bss() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe {
            (a as *mut u8).write_volatile(0);
        }
    })
}

