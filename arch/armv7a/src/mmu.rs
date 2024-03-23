use crate::{asm::isb, registers::*};

mod attributes;
mod bitfields;
mod level1;
mod level2;

pub use attributes::*;
pub use level1::*;
pub use level2::*;

pub struct MMU;

impl MMU {
    pub unsafe fn enable() {
        SCTLR.modify(SCTLR::M::Enable);
        isb()
    }

    pub unsafe fn disable() {
        SCTLR.modify(SCTLR::M::Disable);
        isb()
    }

    pub unsafe fn setup(pd: &PageDirectory) {
        let base_addr = pd.base_ptr() as u32;

        #[cfg(not(feature = "lpa_ext"))]
        TTBCR.write(TTBCR::N.val(0) + TTBCR::PD0::Enable + TTBCR::PD1::Enable);

        #[cfg(feature = "lpa_ext")]
        TTBCR.write(TTBCR::N.val(0) + TTBCR::PD0::Enable + TTBCR::PD1::Enable + TTBCR::EAE::Bit32);

        #[cfg(not(feature = "mpcore"))]
        TTBR0.write(
            TTBR0::C::InnerCacheable
                + TTBR0::S::NonShareable
                + TTBR0::RGN::OuterWriteBackWriteAllocate
                + TTBR0::NOS::CLEAR // Ignored by S == NonShareable
                + TTBR0::BASE.val(base_addr >> 14),
        );

        #[cfg(feature = "mpcore")]
        TTBR0.write(
            TTBR0::IRGN1::CLEAR
                + TTBR0::S::NonShareable
                + TTBR0::RGN::OuterWriteBackWriteAllocate
                + TTBR0::NOS::CLEAR // Ignored by S == NonShareable
                + TTBR0::IRGN0::SET
                + TTBR0::BASE.val(base_addr >> 14),
        );

        DACR.write(DACR::D0::Client);
        SCTLR.modify(SCTLR::TRE::Disable + SCTLR::AFE::Enable);

        isb()
    }
}
