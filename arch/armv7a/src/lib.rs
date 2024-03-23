#![no_std]
#![feature(naked_functions)]
#![feature(stmt_expr_attributes)]
#![feature(pointer_is_aligned)]
#![feature(strict_provenance)]
#![allow(clippy::missing_safety_doc)]

mod start;
mod vectors;

pub mod asm;
pub mod cache;
pub mod lock;
pub mod mmu;
pub mod registers;

pub use start::*;
