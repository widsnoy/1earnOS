#![no_std]
#![no_main]

#[macro_use]
mod console;

mod lang_items;
mod sbi;
mod logging;
mod sync;
mod batch;

use core::arch::global_asm;
use log::*;

//use sbi::console_putchar;
global_asm!(include_str!("entry.asm"));

#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    clear_bss();
    logging::init();
    space_info();
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

pub fn space_info() {
    unsafe extern "C" {
        fn stext();
        fn etext();

        fn srodata();
        fn erodata();

        fn sdata();
        fn edata();

        fn boot_stack_lower_bound();
        fn boot_stack_top();

        fn sbss();
        fn ebss();
    }

    println!("[kernel] Hello, world!");
    trace!(
        "[kernel] .text [{:#x}, {:#x})",
        stext as usize,
        etext as usize
    );
    debug!(
        "[kernel] .rodata [{:#x}, {:#x})",
        srodata as usize, erodata as usize
    );
    info!(
        "[kernel] .data [{:#x}, {:#x})",
        sdata as usize, edata as usize
    );
    warn!(
        "[kernel] boot_stack top_bottom={:#x}, lower_bound={:#x}",
        boot_stack_top as usize, boot_stack_lower_bound as usize
    );
    error!("[kernel] .bss [{:#x}, {:#x})", sbss as usize, ebss as usize);

    // CI autotest success: sbi::shutdown(false)
    // CI autotest failed : sbi::shutdown(true)
    sbi::shutdown(false)
}
