#![no_std]
#![feature(naked_functions)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(not(any(feature = "stm32mp1", feature = "zynq7000")))]
compile_error!("No platform selected. [stm32mp1, zynq7000]");

#[cfg(all(feature = "stm32mp1", feature = "zynq7000"))]
compile_error!("At most one platform can be selected. [stm32mp1, zynq7000]");

#[cfg(target_arch = "arm")]
mod aarch32;
