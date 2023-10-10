#![no_std]
#![no_main]
#![feature(naked_functions)]

use core::panic::PanicInfo;

use sel4_rs::{rootserver, sel4_sys, Rootserver};

rootserver!(System);
struct System;

impl Rootserver for System {
    fn rootserver() -> ! {
        unsafe { sel4_sys::seL4_DebugDumpScheduler() };

        unsafe { sel4_sys::seL4_DebugHalt() };

        loop {}
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
