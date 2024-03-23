use paste::paste;
use tock_registers::register_bitfields;

coproc_reg!(pub readonly MIDR<T = u32, R = MIDR::Register> => cp15(0, _, c0, c0, 0)); /* Main ID Register */

coproc_reg!(pub readonly CTR<T = u32>     => cp15(0, _, c0, c0, 1)); /* Cache Type Register */
coproc_reg!(pub readonly TCMTR<T = u32>   => cp15(0, _, c0, c0, 2)); /* TCM Type Register */
coproc_reg!(pub readonly TLBTR<T = u32>   => cp15(0, _, c0, c0, 3)); /* TLB Type Register */
coproc_reg!(pub readonly MPIDR<T = u32>   => cp15(0, _, c0, c0, 5)); /* Multiprocessor Affinity Register */

/* Revision ID register */
coproc_reg! {
    #[cfg(any(feature = "cortex-a7", feature = "cortex-a9"))]
    pub readonly REVIDR<T = u32> => cp15(0, _, c0, c0, 6)
}

coproc_reg!(pub readonly MID_PFR0<T = u32>    => cp15(0, _, c0, c1, 0)); /* Processor Feature Register 0 */
coproc_reg!(pub readonly MID_PFR1<T = u32>    => cp15(0, _, c0, c1, 1)); /* Processor Feature Register 1 */
coproc_reg!(pub readonly MID_DFR0<T = u32>    => cp15(0, _, c0, c1, 2)); /* Debug Feature Register 0 */
coproc_reg!(pub readonly MID_AFR0<T = u32>    => cp15(0, _, c0, c1, 3)); /* Auxiliary Feature Register 0 */
coproc_reg!(pub readonly MID_MMFR0<T = u32>   => cp15(0, _, c0, c1, 4)); /* Memory Model Features Register 0 */
coproc_reg!(pub readonly MID_MMFR1<T = u32>   => cp15(0, _, c0, c1, 5)); /* Memory Model Features Register 1 */
coproc_reg!(pub readonly MID_MMFR2<T = u32>   => cp15(0, _, c0, c1, 6)); /* Memory Model Features Register 2 */
coproc_reg!(pub readonly MID_MMFR3<T = u32>   => cp15(0, _, c0, c1, 7)); /* Memory Model Features Register 3 */
coproc_reg!(pub readonly ID_ISAR0<T = u32>    => cp15(0, _, c0, c2, 0)); /* Instruction Set Attributes Register 0 */
coproc_reg!(pub readonly ID_ISAR1<T = u32>    => cp15(0, _, c0, c2, 1)); /* Instruction Set Attributes Register 1 */
coproc_reg!(pub readonly ID_ISAR2<T = u32>    => cp15(0, _, c0, c2, 2)); /* Instruction Set Attributes Register 2 */
coproc_reg!(pub readonly ID_ISAR3<T = u32>    => cp15(0, _, c0, c2, 3)); /* Instruction Set Attributes Register 3 */
coproc_reg!(pub readonly ID_ISAR4<T = u32>    => cp15(0, _, c0, c2, 4)); /* Instruction Set Attributes Register 4 */
coproc_reg!(pub readonly ID_ISAR5<T = u32>    => cp15(0, _, c0, c2, 5)); /* Instruction Set Attributes Register 5 */

coproc_reg!(pub readonly CCSIDR<T = u32, R = CCSIDR::Register>  => cp15(1, _, c0, c0, 0)); /* Cache Size Identification Register */
coproc_reg!(pub readonly CLIDR<T = u32, R = CLIDR::Register>    => cp15(1, _, c0, c0, 1)); /* Cache Level ID Register */
coproc_reg!(pub readonly AIDR<T = u32>                          => cp15(1, _, c0, c0, 7)); /* Auxiliary ID Register */
coproc_reg!(pub readwrite CSSELR<T = u32, R = CSSELR::Register> => cp15(2, _, c0, c0, 0)); /* Cache Size Selection Register */

coproc_reg!(#[cfg(feature = "virt_ext")] pub readwrite VPIDR<T = u32>     => cp15(4, _, c0, c0, 0)); /* Virtualization Processor ID Register */
coproc_reg!(#[cfg(feature = "virt_ext")] pub readwrite VMPIDR<T = u32>    => cp15(4, _, c0, c0, 5)); /* irtualization Multiprocessor ID Register */

register_bitfields! {
    u32,
    pub MIDR [
        Revision OFFSET(0) NUMBITS(4) [],
        PrimaryPartNumber OFFSET(4) NUMBITS(12) [],
        Architecture OFFSET(16) NUMBITS(4) [],
        Variant OFFSET(20) NUMBITS(4) [],
        Implementer OFFSET(24) NUMBITS(8) [],
    ]
}

register_bitfields! {
    u32,
    pub CCSIDR [
        LineSize OFFSET(0) NUMBITS(3),
        Associativity OFFSET(3) NUMBITS(10),
        NumSets OFFSET(13) NUMBITS(15),
        WA OFFSET(28) NUMBITS(1),
        RA OFFSET(29) NUMBITS(1),
        WB OFFSET(30) NUMBITS(1),
        WT OFFSET(31) NUMBITS(1)
    ]
}

register_bitfields! {
    u32,
    pub CLIDR [
        Ctype1 OFFSET(0) NUMBITS(3) [
            NoCache = 0b000,
            Instruction = 0b001,
            Data = 0b010,
            SeperateInstructionAndData = 0b011,
            Unified = 0b100,
        ],
        Ctype2 OFFSET(3) NUMBITS(3) [],
        Ctype3 OFFSET(6) NUMBITS(3) [],
        Ctype4 OFFSET(9) NUMBITS(3) [],
        Ctype5 OFFSET(12) NUMBITS(3) [],
        Ctype6 OFFSET(15) NUMBITS(3) [],
        Ctype7 OFFSET(18) NUMBITS(3) [],
        LoUIS OFFSET(21) NUMBITS(3) [],
        LoC OFFSET(24) NUMBITS(3) [],
        LoUU OFFSET(27) NUMBITS(3) []
    ]
}

register_bitfields! {
    u32,
    pub CSSELR [
        InD OFFSET(0) NUMBITS(1) [
            DataOrUnified = 0,
            Instruction = 1
        ],

        Level OFFSET(1) NUMBITS(3) [
            Level1 = 0b000,
            Level2 = 0b001,
            Level3 = 0b010,
            Level4 = 0b011,
            Level5 = 0b100,
            Level6 = 0b101,
            Level7 = 0b110,
        ]
    ]
}
