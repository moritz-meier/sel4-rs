use core::ops::RangeInclusive;

mod cp15_branch;
mod cp15_dcache;
mod cp15_icache;
mod scu;

pub use scu::*;

pub struct DCache;

impl DCache {
    pub unsafe fn enable() {
        cp15_dcache::enable()
    }

    pub unsafe fn disable() {
        cp15_dcache::disable()
    }

    pub unsafe fn invalidate_all() {
        cp15_dcache::invalidate_all()
    }

    pub unsafe fn invalidate_range(virt_range: RangeInclusive<u32>) {
        cp15_dcache::invalidate_range(virt_range)
    }

    pub unsafe fn clean_invalidate_all() {
        cp15_dcache::clean_invalidate_all()
    }

    pub unsafe fn clean_invalidate_range(virt_range: RangeInclusive<u32>) {
        cp15_dcache::clean_invalidate_range(virt_range)
    }
}

pub struct ICache;

impl ICache {
    pub unsafe fn enable() {
        cp15_icache::enable()
    }

    pub unsafe fn disable() {
        cp15_icache::disable()
    }

    pub unsafe fn invalidate_all() {
        cp15_icache::invalidate_all()
    }

    pub unsafe fn invalidate_range(virt_range: RangeInclusive<u32>) {
        cp15_icache::invalidate_range(virt_range)
    }
}

pub struct BranchPredictor;

impl BranchPredictor {
    pub unsafe fn enable() {
        cp15_branch::enable()
    }

    pub unsafe fn disable() {
        cp15_branch::disable()
    }

    pub unsafe fn invalidate_all() {
        cp15_branch::invalidate_all()
    }

    pub unsafe fn invalidate_range(virt_range: RangeInclusive<u32>) {
        cp15_branch::invalidate_range(virt_range)
    }
}
