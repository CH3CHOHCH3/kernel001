#![no_std]
#![no_main]

mod lang;

use core::arch::global_asm;
global_asm!(include_str!("entry.S"));

#[no_mangle]
pub fn start() -> ! {
    clear_bss();
    loop {}
}

/**
 * Qemu with -kernel option will clear the .bss automatically.
 * However we use -device so we clear it manually.
 */
fn clear_bss() {
    extern "C" {
        // sbss and ebss are defined in kernel.ld
        static sbss: usize;
        static ebss: usize;
    }
    unsafe {
        (sbss..ebss).for_each(|p| {
            (p as *mut u8).write_volatile(0)
        });
    }
}
