macro_rules! coproc_reg {
    ($(#[$attr:meta])* $vis:vis readonly $name:ident<T=$type:ty $(,R=$reg:ty)?> => $cp:ident $params:tt) => {
        paste! {
            $(#[$attr])*
            #[allow(non_camel_case_types)]
            $vis struct [<$name Accessor>];

            $(#[$attr])*
            $vis const $name: [<$name Accessor>] = [<$name Accessor>];

            coproc_impl! {
                $(#[$attr])*
                impl Readable<T = $type $(,R=$reg)?> for [<$name Accessor>] => $cp $params
            }
        }
    };

    ($(#[$attr:meta])* $vis:vis writeonly $name:ident<T=$type:ty $(,R=$reg:ty)?> => $cp:ident $params:tt) => {
        paste! {
            $(#[$attr])*
            #[allow(non_camel_case_types)]
            $vis struct [<$name Accessor>];

            $(#[$attr])*
            $vis const $name: [<$name Accessor>] = [<$name Accessor>];

            coproc_impl! {
                $(#[$attr])*
                impl Writeable<T = $type $(,R=$reg)?> for [<$name Accessor>] => $cp $params
            }
        }
    };

    ($(#[$attr:meta])* $vis:vis readwrite $name:ident<T=$type:ty $(,R=$reg:ty)?> => $cp:ident $params:tt) => {
        paste! {
            $(#[$attr])*
            #[allow(non_camel_case_types)]
            $vis struct [<$name Accessor>];

            $(#[$attr])*
            $vis const $name: [<$name Accessor>] = [<$name Accessor>];

            coproc_impl! {
                $(#[$attr])*
                impl Readable<T = $type $(,R=$reg)?> for [<$name Accessor>] => $cp $params
            }

            coproc_impl! {
                $(#[$attr])*
                impl Writeable<T = $type $(,R=$reg)?> for [<$name Accessor>] => $cp $params
            }
        }
    };
}

macro_rules! coproc_impl {
    ($(#[$attr:meta])* impl Readable<T=$type:ident $(,R=$reg:ty)?> for $accessor:ident => $cp:ident $params:tt) => {
        $(#[$attr])*
        impl tock_registers::interfaces::Readable for $accessor {
            type T = $type;
            type R = default_reg_type!($($reg)?);

            #[inline]
            fn get(&self) -> $type {
                unsafe { coproc_read!($type, $cp $params) }
            }
        }
    };

    ($(#[$attr:meta])* impl Writeable<T=$type:ident $(,R=$reg:ty)?> for $accessor:ident => $cp:ident $params:tt) => {
        $(#[$attr])*
        impl tock_registers::interfaces::Writeable for $accessor {
            type T = $type;
            type R = default_reg_type!($($reg)?);

            #[inline]
            fn set(&self, value: $type) {
                unsafe { coproc_write!($type, value, $cp $params) }
            }
        }
    };
}

macro_rules! default_reg_type {
    () => {
        ()
    };

    ($reg:ty) => {
        $reg
    };
}

macro_rules! coproc_read {
    (u32, cp15($opc1:literal, _, $crn:ident, $crm:ident, $opc2:literal)) => {
        coproc_read_u32!(p15, $opc1, _, $crn, $crm, $opc2)
    };

    (u64, cp15($opc:literal, _, $crm:ident)) => {
        coproc_read_u64!(p15, $opc, _, $crm)
    };
}

macro_rules! coproc_write {
    (u32, $val:ident, cp15($opc1:literal, _, $crn:ident, $crm:ident, $opc2:literal)) => {
        coproc_write_u32!(p15, $opc1, $val, $crn, $crm, $opc2)
    };

    (u64, $val:ident, cp15($opc:literal, _, $crm:ident)) => {
        coproc_write_u64!(p15, $opc, $val, $crm)
    };
}

#[macropol::macropol]
#[allow(unused_macros)]
macro_rules! coproc_read_u32 {
    ($cp:ident, $opc1:literal, _, $crn:ident, $crm:ident, $opc2:literal) => {
        {
            let value: u32;
            core::arch::asm!(
                "mrc $&cp, $&opc1, {}, $&crn, $&crm, $&opc2",
                lateout(reg) value,
            );
            value
        }
    };
}

#[macropol::macropol]
#[allow(unused_macros)]
macro_rules! coproc_write_u32 {
    ($cp:ident, $opc1:literal, $val:ident, $crn:ident, $crm:ident, $opc2:literal) => {
        {
            {
                let value = $val as u32;
                core::arch::asm!(
                    "mcr $&cp, $&opc1, {}, $&crn, $&crm, $&opc2",
                    in(reg) value,
                )
            }
        }
    };
}

#[macropol::macropol]
#[allow(unused_macros)]
macro_rules! coproc_read_u64 {
    ($cp:ident, $opc:literal, _, $crm:ident) => {
        {
            let low: u32;
            let high: u32;
            core::arch::asm!(
                "mrrc $&cp, $&opc, {low}, {high}, $&crm",
                low = lateout(reg) low,
                high = lateout(reg) high
            );
            ((high as u64) << u32::BITS) | (low as u64)
        }
    };
}

#[macropol::macropol]
#[allow(unused_macros)]
macro_rules! coproc_write_u64 {
    ($cp:ident, $opc:literal, $val:ident, $crm:ident) => {
        {
            let value = $val as u64;
            let low: u32 = (value & ((1 << u32::BITS) - 1)) as u32;
            let high: u32 = (value >> u32::BITS) as u32;
            core::arch::asm!(
                "mcrr $&cp, $&opc, {low}, {high}, $&crm",
                low = in(reg) low,
                high = in(reg) high
            )
        }
    };
}

mod cpsr;

mod cp15_c15;
mod cp15_cache;
mod cp15_fault;
mod cp15_ident;
mod cp15_mmu;
mod cp15_secext;
mod cp15_sysctrl;
mod cp15_thread;
mod cp15_tlb;

pub use cpsr::*;

pub use cp15_cache::*;
pub use cp15_fault::*;
pub use cp15_ident::*;
pub use cp15_mmu::*;
pub use cp15_sysctrl::*;
pub use cp15_thread::*;
pub use cp15_tlb::*;

#[allow(unused_imports)]
pub use cp15_secext::*;

#[allow(unused_imports)]
pub use cp15_c15::*;

pub use tock_registers::{fields::*, interfaces::*, registers::*, *};
