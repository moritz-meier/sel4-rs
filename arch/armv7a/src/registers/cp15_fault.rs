use paste::paste;

coproc_reg!(pub readwrite DFSR<T = u32>     => cp15(0, _, c5, c0, 0)); /* Data Fault Status Register */
coproc_reg!(pub readwrite IFSR<T = u32>     => cp15(0, _, c5, c0, 1)); /* Instruction Fault Status Register */
coproc_reg!(pub readwrite ADFSR<T = u32>    => cp15(0, _, c5, c1, 0)); /* Auxiliary Data Fault Status Register */
coproc_reg!(pub readwrite AIFSR<T = u32>    => cp15(0, _, c5, c1, 1)); /* Auxiliary Instruction Fault Status Register */

coproc_reg!(#[cfg(feature = "virt_ext")] pub readwrite HADFSR<T = u32>  => cp15(4, _, c5, c1, 0)); /* Hyp Auxiliary Data Fault Status Register */
coproc_reg!(#[cfg(feature = "virt_ext")] pub readwrite HAIFSR<T = u32>  => cp15(4, _, c5, c1, 1)); /* Hyp Auxiliary Instruction Fault Status Register */
coproc_reg!(#[cfg(feature = "virt_ext")] pub readwrite HSR<T = u32>     => cp15(4, _, c5, c2, 0)); /* Hyp Syndrome Register */

coproc_reg!(pub readwrite DFAR<T = u32> => cp15(0, _, c6, c0, 0)); /* Data Fault Address Register */
coproc_reg!(pub readwrite IFAR<T = u32> => cp15(0, _, c6, c0, 2)); /* Instruction Fault Address Register */

coproc_reg!(#[cfg(feature = "virt_ext")] pub readwrite HDFAR<T = u32> => cp15(4, _, c6, c0, 0)); /* Hyp Data Fault Address Register */
coproc_reg!(#[cfg(feature = "virt_ext")] pub readwrite HIFAR<T = u32> => cp15(4, _, c6, c0, 2)); /* Hyp Instruction Fault Address Register */
coproc_reg!(#[cfg(feature = "virt_ext")] pub readwrite HPFAR<T = u32> => cp15(4, _, c6, c0, 4)); /* Hyp IPA Fault Address Register */
