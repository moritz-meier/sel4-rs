#![no_std]
#![no_main]
#![feature(naked_functions)]

use core::{arch::asm, panic::PanicInfo};

use rootserver::libsel4_sys;
use rootserver::rootserver;

#[rootserver]
unsafe fn main() -> ! {
    libsel4_sys::seL4_DebugDumpScheduler();

    loop {
        asm!("nop")
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
