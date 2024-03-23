use paste::paste;

coproc_reg!(#[cfg(feature = "cortex-a9")] pub readonly CBAR<T = u32> => cp15(4, _, c15, c0, 0)); /* Configuration Base Address Register */
