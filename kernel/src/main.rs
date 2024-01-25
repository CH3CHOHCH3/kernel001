#![no_std]
#![no_main]

mod lang;

use core::arch::global_asm;
global_asm!(include_str!("entry.S"));

#[no_mangle]
pub fn start() -> ! {
    loop {}
}