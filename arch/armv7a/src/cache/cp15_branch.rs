use core::{mem::size_of, ops::RangeInclusive};

use crate::{asm::*, registers::*};

pub(super) unsafe fn enable() {
    SCTLR.modify(SCTLR::Z::SET);
    isb()
}

pub(super) unsafe fn disable() {
    SCTLR.modify(SCTLR::Z::CLEAR);

    isb()
}

pub(super) unsafe fn invalidate_all() {
    BPIALL.set(0);
    dsb();
    isb()
}

pub(super) unsafe fn invalidate_range(virt_range: RangeInclusive<u32>) {
    CSSELR.write(CSSELR::Level::Level1 + CSSELR::InD::Instruction);
    isb();

    let ccsidr = CCSIDR.extract();
    CSSELR.write(CSSELR::Level::Level1 + CSSELR::InD::DataOrUnified);

    let linewidth_bytes = (1 << (ccsidr.read(CCSIDR::LineSize) + 2)) * size_of::<u32>() as u32;

    for vaddr in virt_range.step_by(linewidth_bytes as usize) {
        let line_addr = vaddr & !(linewidth_bytes - 1);
        BPIMVA.set(line_addr);
    }

    dsb();
    isb()
}
