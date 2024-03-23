use crate::registers::*;

#[cfg(feature = "mpcore")]
pub struct SnoopControlUnit;

#[cfg(feature = "mpcore")]
impl SnoopControlUnit {
    #[cfg(feature = "cortex-a9")]
    pub unsafe fn enable() {
        let scu = get_scu_regs();

        // Errata
        // ARM Cortex-A9 processors Software Developers Errata Notice (r4)
        // (764369) Data or unified cache line maintenance by MVA fails on Inner Shareable memory

        // If Cortex-A9
        if MIDR.get() & 0xFF0FFFF0 == 0x410FC090 {
            // "Set bit[0] in the undocumented SCU Diagnostic Control register
            // located at offset 0x30 from the PERIPHBASE address.
            //
            // Setting this bit disables the migratory bit feature.
            // This forces a dirty cache line to be evicted to the lower
            // memory subsystem, which is both the Point of Coherency
            // and the Point of Unification, when it is being read
            // by another processor.
            //
            // Note that this bit can be written, but is always Read as Zero."
            scu.diagnostic_control
                .modify(DiagnosticControl::DisableMigratoryBitFeature::SET);
        }

        scu.control.modify(Control::Enable::SET);
    }

    #[cfg(feature = "cortex-a9")]
    pub unsafe fn disable() {
        let scu = get_scu_regs();
        scu.control.modify(Control::Enable::CLEAR);
    }

    pub unsafe fn enable_smp() {
        ACTLR.modify(ACTLR::SMP::SET);

        #[cfg(feature = "cortex-a9")]
        ACTLR.modify(ACTLR::FW::SET);
    }

    pub unsafe fn disable_smp() {
        ACTLR.modify(ACTLR::SMP::CLEAR);

        #[cfg(feature = "cortex-a9")]
        ACTLR.modify(ACTLR::FW::CLEAR);
    }
}

#[cfg(all(feature = "cortex-a9", feature = "mpcore"))]
unsafe fn get_scu_regs() -> &'static mut SCU_RegisterBlock {
    let base_addr = CBAR.get();
    let scu = base_addr as *mut SCU_RegisterBlock;

    unsafe { &mut *scu }
}

#[cfg(all(feature = "cortex-a9", feature = "mpcore"))]
register_structs! {
    #[allow(non_camel_case_types)]
    SCU_RegisterBlock {
        (0x00 => control: ReadWrite<u32, Control::Register>),
        (0x04 => _other1),
        (0x30 => diagnostic_control: ReadWrite<u32, DiagnosticControl::Register>),
        (0x34 => _other2),
        (0x58 => @END),
    }
}

#[cfg(all(feature = "cortex-a9", feature = "mpcore"))]
register_bitfields! {
    u32,
    Control [
        Enable OFFSET(0) NUMBITS(1) [],
    ],

    DiagnosticControl [
        DisableMigratoryBitFeature OFFSET(0) NUMBITS(1) [],
    ],
}
