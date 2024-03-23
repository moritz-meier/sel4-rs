use paste::paste;

coproc_reg!(pub readwrite CONTEXTIDR<T = u32>   => cp15(0, _, c13, c0, 1)); /* Context ID Register */
coproc_reg!(pub readwrite TPIDRURW<T = u32>     => cp15(0, _, c13, c0, 2)); /* PL0 Software Thread ID Registers*/
coproc_reg!(pub readwrite TPIDRURO<T = u32>     => cp15(0, _, c13, c0, 3)); /* PL0 Software Thread ID Registers*/
coproc_reg!(pub readwrite TPIDRPRW<T = u32>     => cp15(0, _, c13, c0, 4)); /* PL1 Software Thread ID Registers*/

coproc_reg!(#[cfg(feature = "virt_ext")] pub readwrite HTPIDR<T = u32> => cp15(4, _, c13, c0, 2)); /* Hyp Software Thread ID Registers*/
