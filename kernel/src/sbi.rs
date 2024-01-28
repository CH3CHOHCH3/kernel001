#![allow(dead_code)]

use core::arch::asm;

const SBI_SET_TIMER: i32 = 0;
const SBI_CONSOLE_PUTCHAR: i32 = 1;
const SBI_CONSOLE_GETCHAR: i32 = 2;
const SBI_CLEAR_IPI: i32 = 3;
const SBI_SEND_IPI: i32 = 4;
const SBI_REMOTE_FENCE_I: i32 = 5;
const SBI_REMOTE_SFENCE_VMA: i32 = 6;
const SBI_REMOTE_SFENCE_VMA_ASID: i32 = 7;

const SBI_SYSTEM_RESET: i32 = 0x53525354;
const SBI_SHUTDOWN: i32 = 0;

pub struct Sbiret {
    pub err: i64,
    pub val: i64
}

#[inline(always)]
fn sbi_call(eid: i32, fid: i32, arg0: usize, arg1: usize, arg2: usize) -> Sbiret {
    let mut val: i64;
    let mut err: i64;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") arg0 => err,
            inlateout("x11") arg1 => val,
            in("x12") arg2,
            in("x16") fid,
            in("x17") eid,
        );
    }
    Sbiret{err, val}
}

pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, 0, c, 0, 0);
}

pub fn shutdown() -> ! {
    sbi_call(SBI_SYSTEM_RESET, SBI_SHUTDOWN, 0, 0, 0);
    panic!("failed to shutdown.");
}