#![allow(dead_code)]

pub fn console_putchar(ch: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, 0, ch, 0, 0, 0, 0);
}

pub fn console_getchar() -> isize {
    sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0, 0, 0, 0).0
}

// https://github.com/riscv/riscv-sbi-doc/blob/master/riscv-sbi.adoc
// #legacy-sbi-extension-extension-ids-0x00-through-0x0f
// 0x09-0x0F RESERVED
const SBI_SET_TIMER: i32 = 0;
const SBI_CONSOLE_PUTCHAR: i32 = 1;
const SBI_CONSOLE_GETCHAR: i32 = 2;
const SBI_CLEAR_IPI: i32 = 3;
const SBI_SEND_IPI: i32 = 4;
const SBI_REMOTE_FENCE_I: i32 = 5;
const SBI_REMOTE_SFENCE_VMA: i32 = 6;
const SBI_REMOTE_SFENCE_VMA_ASID: i32 = 7;
const SBI_SHUTDOWN: i32 = 8;

#[inline(always)]
fn sbi_call(
    extension: i32,
    function: i32,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
) -> (isize, isize) {
    let (error, value): (isize, isize);
    unsafe {
        asm!("ecall"
            : "={x10}" (error), "={x11}" (value)
            : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x13}" (arg3), "{x14}" (arg4), "{x16}" (function), "{x17}" (extension)
            : "memory"
            : "volatile");
    }
    (error, value)
}
