use core::{mem::size_of, ops::RangeInclusive};

use tock_registers::{
    fields::{FieldValue, TryFromValue},
    interfaces::{ReadWriteable, Readable, Writeable},
};

use crate::{asm::*, registers::*};

pub(super) unsafe fn enable() {
    SCTLR.modify(SCTLR::C::SET);
    unsafe { isb() }
}

pub(super) unsafe fn disable() {
    SCTLR.modify(SCTLR::C::CLEAR);
    unsafe { isb() }
}

pub(super) unsafe fn invalidate_all() {
    op_all(CacheOp::Invalidate)
}

pub(super) unsafe fn invalidate_range(virt_range: RangeInclusive<u32>) {
    op_mva(virt_range, CacheOp::Invalidate);
}

pub(super) unsafe fn clean_invalidate_all() {
    op_all(CacheOp::CleanInvalidate)
}

pub(super) unsafe fn clean_invalidate_range(virt_range: RangeInclusive<u32>) {
    op_mva(virt_range, CacheOp::CleanInvalidate);
}

// -------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq)]
enum CacheOp {
    Invalidate,
    CleanInvalidate,
}

const LEVELS: [CSSELR::Level::Value; 7] = [
    CSSELR::Level::Value::Level1,
    CSSELR::Level::Value::Level2,
    CSSELR::Level::Value::Level3,
    CSSELR::Level::Value::Level4,
    CSSELR::Level::Value::Level5,
    CSSELR::Level::Value::Level6,
    CSSELR::Level::Value::Level7,
];

unsafe fn op_all(op: CacheOp) {
    let clidr = CLIDR.extract();

    for level in LEVELS {
        let ctype = (clidr.get() >> (level as u32 * 3)) & CLIDR::Ctype1.mask;
        let ctype = CLIDR::Ctype1::Value::try_from_value(ctype);

        match ctype {
            Some(
                CLIDR::Ctype1::Value::Data
                | CLIDR::Ctype1::Value::SeperateInstructionAndData
                | CLIDR::Ctype1::Value::Unified,
            ) => dcache_op_level(level, op),
            _ => continue,
        }
    }
}

unsafe fn dcache_op_level(level: CSSELR::Level::Value, op: CacheOp) {
    CSSELR.write(FieldValue::from(level) + CSSELR::InD::DataOrUnified);
    isb();

    let ccsidr = CCSIDR.extract();
    CSSELR.write(CSSELR::Level::Level1 + CSSELR::InD::DataOrUnified);

    let num_sets = ccsidr.read(CCSIDR::NumSets) + 1;
    let num_ways = ccsidr.read(CCSIDR::Associativity) + 1;
    let linewidth_bytes = (1 << (ccsidr.read(CCSIDR::LineSize) + 2)) * size_of::<u32>() as u32;

    let way_shift = size_of::<u32>() as u32 - num_bits(num_ways);
    let set_shift = num_bits(linewidth_bytes);

    dsb();

    for way in 0..num_ways {
        for set in 0..num_sets {
            let way_set_level = way << way_shift | set << set_shift | (level as u32) << 1;

            match op {
                CacheOp::Invalidate => DCISW.set(way_set_level),
                CacheOp::CleanInvalidate => DCCISW.set(way_set_level),
            }
        }
    }

    dsb();
    isb()
}

unsafe fn op_mva(virt_range: RangeInclusive<u32>, op: CacheOp) {
    CSSELR.write(CSSELR::Level::Level1 + CSSELR::InD::DataOrUnified);
    isb();

    let ccsidr = CCSIDR.extract();
    let linewidth_bytes = (1 << (ccsidr.read(CCSIDR::LineSize) + 2)) * size_of::<u32>() as u32;

    let start = *virt_range.start();
    let end = *virt_range.end();

    dsb();

    for vaddr in virt_range.step_by(linewidth_bytes as usize) {
        let line_addr = vaddr & !(linewidth_bytes - 1);

        let mut op = op;
        if line_addr < start && op == CacheOp::Invalidate {
            op = CacheOp::CleanInvalidate;
        }

        let lineend_addr = line_addr.saturating_add(linewidth_bytes - 1);
        if end < lineend_addr && op == CacheOp::Invalidate {
            op = CacheOp::CleanInvalidate;
        }

        match op {
            CacheOp::Invalidate => DCIMVAC.set(line_addr),
            CacheOp::CleanInvalidate => DCCIMVAC.set(line_addr),
        }
    }

    dsb()
}

#[inline(always)]
fn num_bits(val: u32) -> u32 {
    val.next_power_of_two().ilog2()
}
