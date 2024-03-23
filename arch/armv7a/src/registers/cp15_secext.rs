use paste::paste;

coproc_reg!(#[cfg(feature = "sec_ext")] pub readwrite VBAR<T = u32>     => cp15(0, _, c12, c0, 0)); /* Vector Base Address Register */
coproc_reg!(#[cfg(feature = "sec_ext")] pub readwrite MVBAR<T = u32>    => cp15(0, _, c12, c0, 1)); /* Monitor Vector Base Address Register */
coproc_reg!(#[cfg(feature = "sec_ext")] pub readonly ISR<T = u32>       => cp15(0, _, c12, c1, 0)); /* Interrupt Status Register */
coproc_reg!(#[cfg(feature = "virt_ext")] pub readwrite HVBAR<T = u32>   => cp15(4, _, c12, c0, 0)); /* Hyp Vector Base Address Register */
