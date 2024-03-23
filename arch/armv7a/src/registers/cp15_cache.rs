use paste::paste;

coproc_reg!(#[cfg(feature = "mpcore")] pub writeonly ICIALLUIS<T = u32> => cp15(0, _, c7, c1, 0)); /* Invalidate all instruction caches to PoU Inner Shareable */
coproc_reg!(#[cfg(feature = "mpcore")] pub writeonly BPIALLIS<T = u32>  => cp15(0, _, c7, c1, 6)); /* Invalidate all branch predictors Inner Shareable */

coproc_reg!(pub readwrite PAR<T = u32>      => cp15(0, _, c7, c4, 0)); /* Physical Address Register */
coproc_reg!(pub writeonly ICIALLU<T = u32>  => cp15(0, _, c7, c5, 0)); /* Invalidate all instruction caches to PoU */
coproc_reg!(pub writeonly ICIMVAU<T = u32>  => cp15(0, _, c7, c5, 1)); /* Invalidate instruction caches by MVA to PoU */
coproc_reg!(pub writeonly CP15ISB<T = u32>  => cp15(0, _, c7, c5, 4)); /* Instruction Synchronization Barrier operation */
coproc_reg!(pub writeonly BPIALL<T = u32>   => cp15(0, _, c7, c5, 6)); /* Invalidate all branch predictors */
coproc_reg!(pub writeonly BPIMVA<T = u32>   => cp15(0, _, c7, c5, 7)); /* Invalidate MVA from branch predictors */
coproc_reg!(pub writeonly DCIMVAC<T = u32>  => cp15(0, _, c7, c6, 1)); /* Invalidate data or unified cache line by MVA to PoC */
coproc_reg!(pub writeonly DCISW<T = u32>    => cp15(0, _, c7, c6, 2)); /* Invalidate data or unified cache line by set/way */
coproc_reg!(pub writeonly ATS1CPR<T = u32>  => cp15(0, _, c7, c8, 0)); /* PL1 read translation */
coproc_reg!(pub writeonly ATS1CPW<T = u32>  => cp15(0, _, c7, c8, 1)); /* PL1 write translation */
coproc_reg!(pub writeonly ATS1CUR<T = u32>  => cp15(0, _, c7, c8, 2)); /* unprivileged read translation */
coproc_reg!(pub writeonly ATS1CUW<T = u32>  => cp15(0, _, c7, c8, 3)); /* unprivileged write translation */

coproc_reg!(#[cfg(feature = "sec_ext")] pub writeonly ATS12NSOPR<T = u32> => cp15(0, _, c7, c8, 4)); /* PL1 read translation */
coproc_reg!(#[cfg(feature = "sec_ext")] pub writeonly ATS12NSOPW<T = u32> => cp15(0, _, c7, c8, 5)); /* PL1 write translation */
coproc_reg!(#[cfg(feature = "sec_ext")] pub writeonly ATS12NSOUR<T = u32> => cp15(0, _, c7, c8, 6)); /* unprivileged read translation */
coproc_reg!(#[cfg(feature = "sec_ext")] pub writeonly ATS12NSOUW<T = u32> => cp15(0, _, c7, c8, 7)); /* unprivileged write translation */

coproc_reg!(pub writeonly DCCMVAC<T = u32>  => cp15(0, _, c7, c10, 1)); /* Clean data or unified cache line by MVA to PoC */
coproc_reg!(pub writeonly DCCSW<T = u32>    => cp15(0, _, c7, c10, 2)); /* Clean data or unified cache line by set/way */
coproc_reg!(pub writeonly CP15DSB<T = u32>  => cp15(0, _, c7, c10, 4)); /* Data Synchronization Barrier operation */
coproc_reg!(pub writeonly CP15DMB<T = u32>  => cp15(0, _, c7, c10, 5)); /* Data Memory Barrier operation */
coproc_reg!(pub writeonly DCCMVAU<T = u32>  => cp15(0, _, c7, c11, 1)); /* Clean data or unified cache line by MVA to PoU */
coproc_reg!(pub writeonly DCCIMVAC<T = u32> => cp15(0, _, c7, c14, 1)); /* Clean and invalidate data or unified cache line by MVA to PoC */
coproc_reg!(pub writeonly DCCISW<T = u32>   => cp15(0, _, c7, c14, 2)); /* Clean and invalidate data or unified cache line by set/way */

coproc_reg!(#[cfg(feature = "virt_ext")] pub writeonly ATS1HR<T = u32> => cp15(4, _, c7, c8, 0)); /* Hyp mode read translation */
coproc_reg!(#[cfg(feature = "virt_ext")] pub writeonly ATS1HW<T = u32> => cp15(4, _, c7, c8, 1)); /* Hyp mode write translation */
