use core::{mem::size_of, ops::RangeInclusive};

use tock_registers::interfaces::{ReadWriteable, Readable, Writeable};

use crate::{asm::*, registers::*};

pub(super) unsafe fn enable() {
    SCTLR.modify(SCTLR::I::SET);
    unsafe { isb() }
}

pub(super) unsafe fn disable() {
    SCTLR.modify(SCTLR::I::CLEAR);
    unsafe { isb() }
}

pub(super) unsafe fn invalidate_all() {
    ICIALLU.set(0);
    unsafe {
        dsb();
        isb()
    }
}

pub(super) unsafe fn invalidate_range(virt_range: RangeInclusive<u32>) {
    CSSELR.write(CSSELR::Level::Level1 + CSSELR::InD::Instruction);
    unsafe { isb() }

    let ccsidr = CCSIDR.extract();
    CSSELR.write(CSSELR::Level::Level1 + CSSELR::InD::DataOrUnified);

    let linewidth_bytes = (1 << (ccsidr.read(CCSIDR::LineSize) + 2)) * size_of::<u32>() as u32;

    for vaddr in virt_range.step_by(linewidth_bytes as usize) {
        let line_addr = vaddr & !(linewidth_bytes - 1);
        ICIMVAU.set(line_addr);
    }

    unsafe {
        dsb();
        isb()
    }
}
