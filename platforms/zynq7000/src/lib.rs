#![no_std]

pub mod platform;
pub mod uart;

mod static_ref;

pub use armv7a as arch;
pub use platform::*;
