#![no_std]
#![feature(naked_functions)]
#![allow(clippy::missing_safety_doc)]

use core::arch::asm;

pub use macros::rootserver;

pub use sel4_lib_sys as libsel4_sys;

// todo: use ToolchainConfig or gen_config.json, not platform
// #[cfg(any(feature = "zynq7000"))]
mod arm32;

pub trait Rootserver {
    unsafe extern "C" fn entry() -> !;
}

#[naked]
#[rustfmt::skip]
pub unsafe extern "C" fn start<RootserverImpl: Rootserver>() -> ! {
    asm! {
        "ldr sp, =__stack_end",
        "b {entry}",
        entry = sym RootserverImpl::entry,
        options(noreturn)
    }
}
