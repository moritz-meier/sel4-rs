use core::arch::asm;

use crate::vectors;

pub use arch_macros::entry;

#[cfg(feature = "multicore")]
pub use arch_macros::secondary_entry;

pub trait PrimaryEntry {
    unsafe extern "C" fn primary_entry() -> !;
}

#[cfg(feature = "multicore")]
pub trait SecondaryEntry {
    unsafe extern "C" fn secondary_entry(cpu_id: usize) -> !;
}

#[cfg(feature = "multicore")]
static mut NON_BOOT_LOCK: u32 = 1;

#[cfg(feature = "multicore")]
pub fn release_secondary_cores() {
    use crate::asm::*;
    use core::ptr::{self, addr_of_mut};

    unsafe {
        ptr::write_volatile(addr_of_mut!(NON_BOOT_LOCK), 0);
        dsb();
        sev();
    }
}

#[cfg(not(feature = "multicore"))]
pub trait Entry = PrimaryEntry;

#[cfg(feature = "multicore")]
pub trait Entry = PrimaryEntry + SecondaryEntry;

#[naked]
#[rustfmt::skip]
pub unsafe extern "C" fn start<EntryImpl: Entry>() -> ! {

    #[cfg(not(feature = "multicore"))]
    asm!(
        // Disable IRQs, FIQs, imprecise aborts
        "cpsid ifa",

        // Check processor mode, stop when unpriviledged
        "mrs r0, cpsr",
        "and r0, r0, #0x1F",            // CPSR M bitmask = 0x1F
        "cmp r0, #0x10",                // PSR_MODE_USR = 0x10
        "beq 1f",

        // todo: impl support for HYP mode
        // Keep HYP mode, otherwise switch to SYS mode
        // "cmp r0, #0x1A",                // PSR_MODE_HYP = 0x1A
        // "beq 2f",
        // "cps 0x1F",                     // PSR_MODE_SYS = 0x1F
        // "2:",

        // Always switch to SYS mode
        "cps 0x1F",
        
        //Get CPU index (into r5), CPU0 continue in pri_cpu_start, other cores stop
        "mrc p15, 0, r5, c0, c0, 5",    // Get MPIDR
        "and r5, r5, #0xFF",            // Affinitiy level 0
        "cmp r5, #0",
        "beq {pri_cpu_start}",

        // Stop
        "1: wfe",
        "b 1b",

        pri_cpu_start = sym primary_cpu_start::<EntryImpl>,
        options(noreturn)
    );

    #[cfg(feature = "multicore")]
    asm!(
        // Disable IRQs, FIQs, imprecise aborts
        "cpsid ifa",

        // Check processor mode, stop when unpriviledged
        "mrs r0, cpsr",
        "and r0, r0, #0x1F",            // CPSR M bitmask = 0x1F
        "cmp r0, #0x10",                // PSR_MODE_USR = 0x10
        "beq 1f",

        // todo: impl support for HYP mode
        // Keep HYP mode, otherwise switch to SYS mode
        // "cmp r0, #0x1A",                // PSR_MODE_HYP = 0x1A
        // "beq 2f",
        // "cps 0x1F",                     // PSR_MODE_SYS = 0x1F
        // "2:",

        // Always switch to SYS mode
        "cps 0x1F",
        
        //Get CPU index (into r5), CPU0 continue in pri_cpu_start, other cores wait for event
        "mrc p15, 0, r5, c0, c0, 5",    // Get MPIDR
        "and r5, r5, #0xFF",            // Affinity level 0
        "cmp r5, #0",
        "beq {pri_cpu_start}",

        // Secondary cores wait for release
        "2:",
        "wfe",
        "ldr r0, ={non_boot_lock}",
        "ldr r1, [r0]",
        "cmp r1, #0",
        "bgt 2b",

        // Secondary cores continue in sec_cpu_start
        "b {sec_cpu_start}",

        // Stop
        "1: wfe",
        "b 1b",

        non_boot_lock = sym NON_BOOT_LOCK,
        pri_cpu_start = sym primary_cpu_start::<EntryImpl>,
        sec_cpu_start = sym secondary_cpu_start::<EntryImpl>,
        options(noreturn)
    );
}

#[naked]
#[rustfmt::skip]
unsafe extern "C" fn primary_cpu_start<EntryImpl: PrimaryEntry>() -> ! {
    // r5 contains CPU index
    asm!(
        // Reset SCTLR
        "mrc p15, 0, r0, c1, c0, 0",    // Read CP15 System Control register
        "bic r0, r0, #(0x1 << 12)",     // Clear I bit 12 to disable I cache
        "bic r0, r0, #(0x1 << 2)",      // Clear C bit  2 to disable D cache
        "bic r0, r0, #0x1",             // Clear M bit  0 to disable MMU
        "bic r0, r0, #(0x1 << 11)",     // Clear Z bit 11 to disable branch prediction
        "bic r0, r0, #(0x1 << 13)",     // Clear V bit 13 to disable high vectors
        "mcr p15, 0, r0, c1, c0, 0",    // Write value back to CP15 System Control register
        "isb",

        // Enable SMP
        /*
        *   NOTE: (Cortex-A7 TRM and others)
        *   "You must ensure the ACTLR.SMP bit is set to 1 before the caches and MMU are enabled,
        *   or any cache and TLB maintenance operations are performed. ..."
        */
        "mrc p15, 0, r0, c1, c0, 1",    // Read CP15 Auxiliary Control Register
        "orr r0, r0, #(0x1 << 6)",      // Set SMP bit  6 to enable SMP

        "mrc p15, 0, r1, c0, c0, 0",    // Read CP15 Main ID Register
        "movt r2, #0xFF0F",             // Load ID mask (Implementer, Architecture, Primary Part Number)
        "movw r2, #0xFFF0",
        "and r1, r1, r2",               // Get ID
        "movt r2, #0x410F",             // Load Cortex-A9 ID
        "movw r2, #0xC090",
        "cmp r1, r2",                   // Check for Cortex-A9
        "orreq r0, r0, #0x1",           // If Cortex-A9, also set FW bit

        "mcr p15, 0, r0, c1, c0, 1",    // Write value back to CP15 Auxiliary Control Register

        // Invalidate caches and TLBs
        /*
        *   NOTE: (Cortex-A7 TRM and others)
        *   "The ARMv7 Virtual Memory System Architecture (VMSA) does not
        *   support a CP15 operation to invalidate the entire data cache. ...
        *   In normal usage the only time the entire data cache has to be
        *   invalidated is on reset."
        *
        * The instruction cache is virtually indexed and physically tagged but
        * the data cache is physically indexed and physically tagged.  So it
        * should not be an issue if the system comes up with a dirty Dcache;
        * the ICache, however, must be invalidated.
        */
        "mov r0, #0",
        "mcr p15, 0, r0, c8, c7, 0",    // TLBIALL  - Invalidate the entire unified TLB
        "mcr p15, 0, r0, c7, c5, 0",    // ICIALLU  - Invalidate I-cache
        "mcr p15, 0, r0, c7, c5, 6",    // BPIALL   - Invalidate entire branch prediction array
        "dsb",
        "isb",

        // Set Vector Base Address Register (VBAR)
        /*
        *   NOTE:
        *   Only valid when the ARM security extension (feature = "sec_ext") is implemented
        *   But currently it is not possible to use #[cfg(..)] within asm!(..)
        *   So assuming here that the security extension is available
        *   This is at least true for Cortex-A5, A7, A9, A15, A17
        */
        "ldr r0, ={vectors}",
        "mcr p15, 0, r0, c12, c0, 0",

        // Set stack pointer
        "ldr sp, =__stack_end",

        // Zero .bss section
        "mov r0, #0",
        "ldr r1, =__bss_start",
        "ldr r2, =__bss_end",
        "2: cmp r1, r2",
        "strlt r0, [r1], #4",
        "blt 2b",

        // Zero .stack section
        "movw r0, #0xFEFE",
        "movt r0, #0xFEFE",
        "ldr r1, =__stack_start",
        "ldr r2, =__stack_end",
        "3: cmp r1, r2",
        "strlt r0, [r1], #4",
        "blt 3b",

        // Jump to primary_entry
        "b {pri_entry}",

        // Stop, just in case primary_entry returns
        "1: wfe",
        "b 1b",

        vectors = sym vectors::vectors,
        pri_entry = sym EntryImpl::primary_entry,
        options(noreturn)
    );
}

#[naked]
#[rustfmt::skip]
#[cfg(feature = "multicore")]
unsafe extern "C" fn secondary_cpu_start<EntryImpl: SecondaryEntry>() -> ! {
    // r5 contains CPU index
    asm!(
        // Reset SCTLR
        "mrc p15, 0, r0, c1, c0, 0",    // Read CP15 System Control register
        "bic r0, r0, #(0x1 << 12)",     // Clear I bit 12 to disable I cache
        "bic r0, r0, #(0x1 << 2)",      // Clear C bit  2 to disable D cache
        "bic r0, r0, #0x1",             // Clear M bit  0 to disable MMU
        "bic r0, r0, #(0x1 << 11)",     // Clear Z bit 11 to disable branch prediction
        "bic r0, r0, #(0x1 << 13)",     // Clear V bit 13 to disable high vectors
        "mcr p15, 0, r0, c1, c0, 0",    // Write value back to CP15 System Control register
        "isb",

        // Enable SMP
        /*
        *   NOTE: (Cortex-A7 TRM and others)
        *   "You must ensure the ACTLR.SMP bit is set to 1 before the caches and MMU are enabled,
        *   or any cache and TLB maintenance operations are performed. ..."
        */
        "mrc p15, 0, r0, c1, c0, 1",    // Read CP15 Auxiliary Control Register
        "orr r0, r0, #(0x1 << 6)",      // Set SMP bit  6 to enable SMP

        "mrc p15, 0, r1, c0, c0, 0",    // Read CP15 Main ID Register
        "movt r2, #0xFF0F",             // Load ID mask (Implementer, Architecture, Primary Part Number)
        "movw r2, #0xFFF0",
        "and r1, r1, r2",               // Get ID
        "movt r2, #0x410F",             // Load Cortex-A9 ID
        "movw r2, #0xC090",
        "cmp r1, r2",                   // Check for Cortex-A9
        "orreq r0, r0, #0x1",           // If Cortex-A9, also set FW bit

        "mcr p15, 0, r0, c1, c0, 1",    // Write value back to CP15 Auxiliary Control Register

        // Invalidate caches and TLBs
        /*
        *   NOTE:
        *   "The ARMv7 Virtual Memory System Architecture (VMSA) does not
        *   support a CP15 operation to invalidate the entire data cache. ...
        *   In normal usage the only time the entire data cache has to be
        *   invalidated is on reset."
        *
        * The instruction cache is virtually indexed and physically tagged but
        * the data cache is physically indexed and physically tagged.  So it
        * should not be an issue if the system comes up with a dirty Dcache;
        * the ICache, however, must be invalidated.
        */
        "mov r0, #0",
        "mcr p15, 0, r0, c8, c7, 0",    // TLBIALL  - Invalidate the entire unified TLB
        "mcr p15, 0, r0, c7, c5, 0",    // ICIALLU  - Invalidate I-cache
        "mcr p15, 0, r0, c7, c5, 6",    // BPIALL   - Invalidate entire branch prediction array
        "dsb",
        "isb",

        // Set Vector Base Address Register (VBAR)
        /*
        *   NOTE:
        *   Only valid when the ARM security extension (feature = "sec_ext") is implemented
        *   But currently it is not possible to use #[cfg(..)] within asm!(..)
        *   So assuming here that the security extension is available
        *   This is at least true for Cortex-A5, A7, A9, A15, A17
        */
        "ldr r0, ={vectors}",
        "mcr p15, 0, r0, c12, c0, 0",

        // Set stack pointer
        "ldr r0, =__NUM_CPU",
        "ldr r1, =__CPU_STACKSIZE",
        "ldr r2, =__stack_start",
        "sub r0, r0, r5",               // NUM_CPU - CPU_ID
        "mla r0, r0, r1, r2",           // (NUM_CPU - CPU_ID) * __CPU_STACKSIZE + __stack_start
        "mov sp, r0",

        // Jump to secondary_entry(cpu_id)
        "mov r0, r5",
        "b {sec_entry}",

        // Stop, just in case secondary_entry returns
        "1: wfe",
        "b 1b",

        vectors = sym vectors::vectors,
        sec_entry = sym EntryImpl::secondary_entry,
        options(noreturn)
    );
}
