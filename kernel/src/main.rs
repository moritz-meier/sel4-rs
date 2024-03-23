#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(ptr_sub_ptr)]
//#![feature(error_in_core)]

use core::{arch::asm, panic::PanicInfo};

use armv7a::entry;

#[entry]
unsafe fn main() -> ! {
    #[cfg(feature = "smp")]
    release_secondary_cores();

    loop {
        asm!("nop")
    }
}

#[cfg(feature = "smp")]
#[secondary_entry]
unsafe fn non_boot_main(_cpu_id: usize) -> ! {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
