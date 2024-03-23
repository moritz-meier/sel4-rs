use core::arch::asm;

pub(super) unsafe fn lock(lock: *mut u32) {
    asm!(
        "mov r2, #1",
        "1: ldrex r1, [{lock}]",
        "cmp r1, #0",
        "wfene",
        "strexeq r1, r2, [{lock}]",
        "cmpeq r1, #0",
        "bne 1b",
        "dmb",
        lock = in(reg) lock,
    )
}

pub(super) unsafe fn unlock(lock: *mut u32) {
    asm!(
        "mov r1, #0",
        "dmb",
        "str r1, [{lock}]",
        "dsb",
        "sev",
        lock = in(reg) lock,
    )
}
