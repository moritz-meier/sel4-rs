use paste::paste;
use tock_registers::register_bitfields;

coproc_reg!(#[cfg(not(feature = "lpa_ext"))] pub readwrite TTBR0<T = u32, R = TTBR0::Register>  => cp15(0, _, c2, c0, 0)); /* Translation Table Base Register 0 */
coproc_reg!(#[cfg(not(feature = "lpa_ext"))] pub readwrite TTBR1<T = u32>                       => cp15(0, _, c2, c0, 1)); /* Translation Table Base Register 1 */

coproc_reg!(pub readwrite TTBCR<T = u32, R = TTBCR::Register> => cp15(0, _, c2, c0, 2)); /* Translation Table Base Control Register */

coproc_reg!(#[cfg(feature = "virt_ext")] pub readwrite HTCR<T = u32> => cp15(4, _, c2, c0, 2)); /* Hyp Translation Control Register */
coproc_reg!(#[cfg(feature = "virt_ext")] pub readwrite VTCR<T = u32> => cp15(4, _, c2, c1, 2)); /* Virtualization Translation Control Register */

coproc_reg!(pub readwrite DACR<T = u32, R = DACR::Register> => cp15(0, _, c3, c0, 0)); /* Domain Access Control Register */

coproc_reg!(#[cfg(feature = "lpa_ext")] pub readwrite TTBR0<T = u64>    => cp15(0, _, c2)); /* Translation Table Base Register 0 */
coproc_reg!(#[cfg(feature = "lpa_ext")] pub readwrite TTBR1<T = u64>    => cp15(1, _, c2)); /* Translation Table Base Register 1 */
coproc_reg!(#[cfg(feature = "virt_ext")] pub readwrite HTTBR<T = u64>   => cp15(4, _, c2)); /* Hyp Translation Table Base Register */
coproc_reg!(#[cfg(feature = "virt_ext")] pub readwrite VTTBR<T = u64>   => cp15(6, _, c2)); /* Virtualization Translation Table Base Register */

register_bitfields! {
    u32,
    pub TTBCR [
        N OFFSET(0) NUMBITS(3) [],
        #[cfg(feature = "sec_ext")]
        PD0 OFFSET(1) NUMBITS(1) [
            Enable = 0,
            Disable = 1,
        ],
        #[cfg(feature = "sec_ext")]
        PD1 OFFSET(1) NUMBITS(1) [
            Enable = 0,
            Disable = 1,
        ],
        #[cfg(feature = "lpa_ext")]
        EAE OFFSET(31) NUMBITS(1) [
            Bit32 = 0,
            Bit40 = 1,
        ]
    ]
}

register_bitfields! {
    u32,
    pub TTBR0 [
        #[cfg(not(feature = "mpcore"))]
        C OFFSET(0) NUMBITS(1) [
            InnerNonCacheable = 0,
            InnerCacheable = 1
        ],
        #[cfg(feature = "mpcore")]
        IRGN1 OFFSET(0) NUMBITS(1) [],
        S OFFSET(1) NUMBITS(1) [
            NonShareable = 0,
            Shareable = 1
        ],
        RGN OFFSET(3) NUMBITS(2) [
            OuterNonCacheable = 0b00,
            OuterWriteBackWriteAllocate = 0b01,
            OuterWriteThrough = 0b10,
            OuterWriteBackNoWriteAllocate = 0b11
        ],
        NOS OFFSET(5) NUMBITS(1) [
            OuterShareable = 0,
            InnerShareable = 1
        ],
        #[cfg(feature = "mpcore")]
        IRGN0 OFFSET(6) NUMBITS(1) [],
        BASE OFFSET(14) NUMBITS(18) [],
    ]
}

register_bitfields! {
    u32,
    pub DACR [
        D0 OFFSET(0) NUMBITS(2) [
            NoAccess = 0b00,
            Client = 0b01,
            Manager = 0b11
        ],

        D1 OFFSET(2) NUMBITS(2) [
            NoAccess = 0b00,
            Client = 0b01,
            Manager = 0b11
        ],

        D2 OFFSET(4) NUMBITS(2) [
            NoAccess = 0b00,
            Client = 0b01,
            Manager = 0b11
        ],

        D3 OFFSET(6) NUMBITS(2) [
            NoAccess = 0b00,
            Client = 0b01,
            Manager = 0b11
        ],

        D4 OFFSET(8) NUMBITS(2) [
            NoAccess = 0b00,
            Client = 0b01,
            Manager = 0b11
        ],

        D5 OFFSET(10) NUMBITS(2) [
            NoAccess = 0b00,
            Client = 0b01,
            Manager = 0b11
        ],

        D6 OFFSET(12) NUMBITS(2) [
            NoAccess = 0b00,
            Client = 0b01,
            Manager = 0b11
        ],

        D7 OFFSET(14) NUMBITS(2) [
            NoAccess = 0b00,
            Client = 0b01,
            Manager = 0b11
        ],

        D8 OFFSET(16) NUMBITS(2) [
            NoAccess = 0b00,
            Client = 0b01,
            Manager = 0b11
        ],

        D9 OFFSET(18) NUMBITS(2) [
            NoAccess = 0b00,
            Client = 0b01,
            Manager = 0b11
        ],

        D10 OFFSET(20) NUMBITS(2) [
            NoAccess = 0b00,
            Client = 0b01,
            Manager = 0b11
        ],

        D11 OFFSET(22) NUMBITS(2) [
            NoAccess = 0b00,
            Client = 0b01,
            Manager = 0b11
        ],

        D12 OFFSET(24) NUMBITS(2) [
            NoAccess = 0b00,
            Client = 0b01,
            Manager = 0b11
        ],

        D13 OFFSET(26) NUMBITS(2) [
            NoAccess = 0b00,
            Client = 0b01,
            Manager = 0b11
        ],

        D14 OFFSET(28) NUMBITS(2) [
            NoAccess = 0b00,
            Client = 0b01,
            Manager = 0b11
        ],

        D15 OFFSET(30) NUMBITS(2) [
            NoAccess = 0b00,
            Client = 0b01,
            Manager = 0b11
        ],
    ]
}
