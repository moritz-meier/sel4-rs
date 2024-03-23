use paste::paste;

coproc_reg!(#[cfg(feature = "mpcore")] pub writeonly TLBIALLIS<T = u32>     => cp15(0, _, c8, c3, 0)); /* Invalidate entire TLB IS */
coproc_reg!(#[cfg(feature = "mpcore")] pub writeonly TLBIMVAIS<T = u32>     => cp15(0, _, c8, c3, 1)); /* Invalidate unified TLB entry by MVA and ASID IS */
coproc_reg!(#[cfg(feature = "mpcore")] pub writeonly TLBIASIDIS<T = u32>    => cp15(0, _, c8, c3, 2)); /* Invalidate unified TLB by ASID match IS */
coproc_reg!(#[cfg(feature = "mpcore")] pub writeonly TLBIMVAAIS<T = u32>    => cp15(0, _, c8, c3, 3)); /* Invalidate unified TLB entry by MVA all ASID IS */

coproc_reg!(pub writeonly ITLBIALL<T = u32>     => cp15(0, _, c8, c5, 0)); /* Invalidate instruction TLB */
coproc_reg!(pub writeonly ITLBIMVA<T = u32>     => cp15(0, _, c8, c5, 1)); /* Invalidate instruction TLB entry by MVA and ASID */
coproc_reg!(pub writeonly ITLBIASID<T = u32>    => cp15(0, _, c8, c5, 2)); /* Invalidate instruction TLB by ASID match */
coproc_reg!(pub writeonly DTLBIALL<T = u32>     => cp15(0, _, c8, c6, 0)); /* Invalidate data TLB */
coproc_reg!(pub writeonly DTLBIMVA<T = u32>     => cp15(0, _, c8, c6, 1)); /* Invalidate data TLB entry by MVA and ASID */
coproc_reg!(pub writeonly DTLBIASID<T = u32>    => cp15(0, _, c8, c6, 2)); /* Invalidate data TLB by ASID match */
coproc_reg!(pub writeonly TLBIALL<T = u32>      => cp15(0, _, c8, c7, 0)); /* Invalidate unified TLB */
coproc_reg!(pub writeonly TLBIMVA<T = u32>      => cp15(0, _, c8, c7, 1)); /* Iinvalidate unified TLB entry by MVA and ASID */
coproc_reg!(pub writeonly TLBIASID<T = u32>     => cp15(0, _, c8, c7, 2)); /* Invalidate unified TLB by ASID match */

coproc_reg!(#[cfg(feature = "mpcore")] pub writeonly TLBIMVAA<T = u32>          => cp15(0, _, c8, c7, 3)); /* Invalidate unified TLB entries by MVA all ASID */
coproc_reg!(#[cfg(feature = "virt_ext")] pub writeonly TLBIALLHIS<T = u32>      => cp15(4, _, c8, c3, 0)); /* Invalidate entire Hyp unified TLB IS */
coproc_reg!(#[cfg(feature = "virt_ext")] pub writeonly TLBIMVAHIS<T = u32>      => cp15(4, _, c8, c3, 1)); /* Invalidate Hyp unified TLB entry by MVA IS */
coproc_reg!(#[cfg(feature = "virt_ext")] pub writeonly TLBIALLNSNHIS<T = u32>   => cp15(4, _, c8, c3, 4)); /* Invalidate entire Non-secure non-Hyp unified TLB IS */
coproc_reg!(#[cfg(feature = "virt_ext")] pub writeonly TLBIALLH<T = u32>        => cp15(4, _, c8, c7, 0)); /* Invalidate entire Hyp unified TLB */
coproc_reg!(#[cfg(feature = "virt_ext")] pub writeonly TLBIMVAH<T = u32>        => cp15(4, _, c8, c7, 1)); /* Invalidate Hyp unified TLB entry by MVA */
coproc_reg!(#[cfg(feature = "virt_ext")] pub writeonly TLBIALLNSNH<T = u32>     => cp15(4, _, c8, c7, 4)); /* Invalidate entire Non-secure non-Hyp unified TLB */
