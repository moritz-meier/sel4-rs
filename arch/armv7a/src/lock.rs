use core::{arch::asm, ptr::addr_of_mut};

use critical_section::{set_impl, RawRestoreState};
use tock_registers::interfaces::Readable;

mod spinlock;

use crate::registers::*;

struct CriticalSection;
set_impl!(CriticalSection);

#[cfg(not(feature = "multicore"))]
unsafe impl critical_section::Impl for CriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        let restore_cpsr = CPSR.get();
        unsafe { asm!("cpsid if") };
        restore_cpsr
    }

    unsafe fn release(restore_cpsr: RawRestoreState) {
        if !CPSR::I.is_set(restore_cpsr) {
            unsafe { asm!("cpsie i") }
        }

        if !CPSR::F.is_set(restore_cpsr) {
            unsafe { asm!("cpsie f") }
        }
    }
}

#[cfg(feature = "multicore")]
static mut GLOBAL_LOCK: u32 = 0;

#[cfg(feature = "multicore")]
unsafe impl critical_section::Impl for CriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        let restore_cpsr = CPSR.get();
        spinlock::lock(addr_of_mut!(GLOBAL_LOCK));
        unsafe { asm!("cpsid if") };
        restore_cpsr
    }

    unsafe fn release(restore_cpsr: RawRestoreState) {
        if !CPSR::I.is_set(restore_cpsr) {
            unsafe { asm!("cpsie i") }
        }

        if !CPSR::F.is_set(restore_cpsr) {
            unsafe { asm!("cpsie f") }
        }

        spinlock::unlock(addr_of_mut!(GLOBAL_LOCK));
    }
}
