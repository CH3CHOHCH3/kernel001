#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod sbi;
mod lang;
mod stdio;
mod config;
mod bootmem;

extern "C" {
    // defined in kernel.ld
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn edata();
    fn sbss();
    fn ebss();
}

use core::arch::global_asm;
global_asm!(include_str!("entry.S"));

#[no_mangle]
pub fn start() -> ! {
    clear_bss();
    print_section();
    bootmem::bootmem_init();
    panic!("shutdown the qemu now.");
}

fn print_section() {
    println!("{} is booting.", "kernel001");
    println!("etext: {:#01x}", etext as usize);
    println!("srodata: {:#01x}", srodata as usize);
    println!("erodata: {:#01x}", erodata as usize);
    println!("sdata: {:#01x}", sdata as usize);
    println!("edata: {:#01x}", edata as usize);
    println!("sbss: {:#01x}", sbss as usize);
    println!("ebss: {:#01x}", ebss as usize);
}

/**
 * Qemu with -kernel option will clear the .bss automatically.
 * However we use -device so we clear it manually.
 */
fn clear_bss() {
    (sbss as usize..ebss as usize).for_each(|p| {
        unsafe {
            (p as *mut u8).write_volatile(0)
        }
    });
}
