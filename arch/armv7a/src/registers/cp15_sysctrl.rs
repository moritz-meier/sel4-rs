use paste::paste;
use tock_registers::register_bitfields;

coproc_reg!(pub readwrite SCTLR<T = u32, R = SCTLR::Register>   => cp15(0, _, c1, c0, 0)); /* System Control Register */
coproc_reg!(pub readwrite ACTLR<T = u32, R = ACTLR::Register>   => cp15(0, _, c1, c0, 1)); /* Auxiliary Control Register */
coproc_reg!(pub readwrite CPACR<T = u32>                        => cp15(0, _, c1, c0, 2)); /* Coprocessor Access Control Register */

coproc_reg!(#[cfg(feature = "sec_ext")] pub readwrite SCR<T = u32>      => cp15(0, _, c1, c1, 0)); /* Secure Configuration Register */
coproc_reg!(#[cfg(feature = "sec_ext")] pub readwrite SDER<T = u32>     => cp15(0, _, c1, c1, 1)); /* Secure Debug Enable Register */
coproc_reg!(#[cfg(feature = "sec_ext")] pub readwrite NSACR<T = u32>    => cp15(0, _, c1, c1, 2)); /* Non-secure Access Control Register */

coproc_reg!(#[cfg(feature = "virt_ext")] pub readwrite HSCTLR<T = u32>  => cp15(4, _, c1, c0, 0)); /* Hyp System Control Register */
coproc_reg!(#[cfg(feature = "virt_ext")] pub readwrite HACTLR<T = u32>  => cp15(4, _, c1, c0, 1)); /* Hyp Auxiliary Control Register */
coproc_reg!(#[cfg(feature = "virt_ext")] pub readwrite HCR<T = u32>     => cp15(4, _, c1, c1, 0)); /* Hyp Configuration Register */
coproc_reg!(#[cfg(feature = "virt_ext")] pub readwrite HDCR<T = u32>    => cp15(4, _, c1, c1, 1)); /* Hyp Debug Configuration Register */
coproc_reg!(#[cfg(feature = "virt_ext")] pub readwrite HCPTR<T = u32>   => cp15(4, _, c1, c1, 2)); /* Hyp Coprocessor Trap Register */
coproc_reg!(#[cfg(feature = "virt_ext")] pub readwrite HSTR<T = u32>    => cp15(4, _, c1, c1, 3)); /* Hyp System Trap Register */
coproc_reg!(#[cfg(feature = "virt_ext")] pub readwrite HACR<T = u32>    => cp15(4, _, c1, c1, 7)); /* Hyp Auxiliary Configuration Register */

register_bitfields! {
    u32,
    pub SCTLR [
        M OFFSET(0) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],
        A OFFSET(1) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],
        C OFFSET(2) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],
        CP15BEN OFFSET(5) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],

        // In ARMv7 this bit is RAZ/SBZP
        // B OFFSET(7) NUMBITS(1) [],

        SW OFFSET(10) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],
        Z OFFSET(11) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],
        I OFFSET(12) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],
        V OFFSET(13) NUMBITS(1) [
            LowVectors = 0,
            HighVectors = 1
        ],
        RR OFFSET(14) NUMBITS(1) [],
        HA OFFSET(17) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],

        #[cfg(feature = "virt_ext")]
        WXN OFFSET(19) NUMBITS(1) [
            Disable = 0,
            WriteableImpliesNeverExecute = 1
        ],

        #[cfg(feature = "virt_ext")]
        UWXN OFFSET(20) NUMBITS(1) [
            Disable = 0,
            WriteableImpliesNeverExecute = 1
        ],

        FI OFFSET(21) NUMBITS(1) [],

        // In ARMv7 this bit is RAO/SBOP
        // U OFFSET(22) NUMBITS(1) [],

        VE OFFSET(24) NUMBITS(1) [],
        EE OFFSET(25) NUMBITS(1) [],
        NMFI OFFSET(27) NUMBITS(1) [],
        /// TEX remap enable
        TRE OFFSET(28) NUMBITS(1) [
            /// TEX remap disabled. `TEX[2:0]` are used, with the `C` and `B` bits, to describe the memory region attributes.
            Disable = 0,
            /// TEX remap enabled. `TEX[2:1]` are reassigned for use as bits managed by the operating system.
            /// The `TEX[0]`, `C` and `B` bits, with the MMU remap registers, describe the memory region attributes.
            Enable = 1
        ],
        /// Access flag enable
        AFE OFFSET(29) NUMBITS(1) [
            /// Full three-bit access model is used
            Disable = 0,
            /// Simplified two-bit access model is used, AP[0] is used as access flag
            Enable = 1
        ],
        TE OFFSET(30) NUMBITS(1) [
            ARM = 0,
            Thumb = 1
        ]
    ]
}

register_bitfields! {
    u32,
    pub ACTLR [
        #[cfg(feature = "cortex-a9")]
        FW OFFSET(0) NUMBITS(1) [],

        #[cfg(any(feature = "mpcore"))]
        SMP OFFSET(6) NUMBITS(1) []
    ]
}
