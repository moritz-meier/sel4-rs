use core::arch::asm;
use core::sync::atomic::{compiler_fence, Ordering};

#[inline(always)]
pub unsafe fn dmb() {
    compiler_fence(Ordering::SeqCst);
    asm!("dmb", options(nomem, nostack, preserves_flags));
    compiler_fence(Ordering::SeqCst);
}

#[inline(always)]
pub unsafe fn dsb() {
    compiler_fence(Ordering::SeqCst);
    asm!("dsb", options(nomem, nostack, preserves_flags));
    compiler_fence(Ordering::SeqCst);
}

#[inline(always)]
pub unsafe fn isb() {
    compiler_fence(Ordering::SeqCst);
    asm!("isb", options(nomem, nostack, preserves_flags));
    compiler_fence(Ordering::SeqCst);
}

#[inline(always)]
pub unsafe fn wfe() {
    compiler_fence(Ordering::SeqCst);
    asm!("wfe", options(nomem, nostack, preserves_flags));
    compiler_fence(Ordering::SeqCst);
}

#[inline(always)]
pub unsafe fn sev() {
    compiler_fence(Ordering::SeqCst);
    asm!("sev", options(nomem, nostack, preserves_flags));
    compiler_fence(Ordering::SeqCst);
}
