#![no_std]
#![no_main]
#![feature(naked_functions)]

use core::{
    arch::asm,
    cell::RefCell,
    fmt::Write,
    panic::PanicInfo,
    ptr::{self, addr_of, addr_of_mut},
};

use critical_section::Mutex;

use armv7a::{
    asm::*,
    cache::{BranchPredictor, DCache, ICache, SnoopControlUnit},
    mmu::{PageDirectory, Section, MMU, NORMAL},
    *,
};

static PAGE_DIRECTORY: Mutex<RefCell<PageDirectory>> =
    Mutex::new(RefCell::new(PageDirectory::new()));

#[entry]
unsafe fn main() -> ! {
    ICache::invalidate_all();
    DCache::clean_invalidate_all();
    BranchPredictor::invalidate_all();

    #[cfg(feature = "cortex-a9")]
    SnoopControlUnit::enable();
    SnoopControlUnit::enable_smp();

    Uart::init();

    extern "C" {
        static mut __kernel_boot_bss_start: u32;
        static mut __kernel_boot_bss_end: u32;

        static mut __kernel_bss_start: u32;
        static mut __kernel_bss_end: u32;
    }

    critical_section::with(|cs| {
        let mut pd = PAGE_DIRECTORY.borrow_ref_mut(cs);

        pd.map_section(&Section::<u32>::new(
            0x0000_0000,
            0x0000_0000,
            NORMAL.read_writeable().executeable(),
        ));

        pd.map_section(&Section::<u32>::new(
            0xE000_0000,
            0x0000_0000,
            NORMAL.read_writeable().executeable(),
        ));

        MMU::setup(&pd);
    });

    MMU::enable();
    ICache::enable();
    DCache::enable();
    BranchPredictor::enable();

    ICache::invalidate_all();

    let mut addr: *mut u32 = addr_of_mut!(__kernel_boot_bss_start);
    let end: *mut u32 = addr_of_mut!(__kernel_boot_bss_end);
    while addr < end {
        ptr::write_volatile(addr, 0);
        addr = addr.offset(1);
    }

    let mut addr: *mut u32 = addr_of_mut!(__kernel_bss_start);
    let end: *mut u32 = addr_of_mut!(__kernel_bss_end);
    while addr < end {
        ptr::write_volatile(addr, 0);
        addr = addr.offset(1);
    }

    dsb();

    extern "C" {
        static mut __devicetree_start: u32;
        static mut __devicetree_size: u32;
        static mut __devicetree_end: u32;
    }

    sel4_kernel_sys::_kernel_entry(
        0x0020_0000,
        0x0030_0000,
        0,
        rootserver_entry as *const fn() -> ! as usize,
        addr_of!(__devicetree_start) as usize,
        addr_of!(__devicetree_size) as usize,
    );
}

#[secondary_entry]
unsafe fn non_boot_main(_cpu_id: usize) -> ! {
    #[allow(clippy::empty_loop)]
    loop {
        unsafe { asm!("nop") }
    }
}

#[link_section = ".rootserver.text"]
fn rootserver_entry() -> ! {
    #[allow(clippy::empty_loop)]
    loop {
        unsafe { asm!("nop") }
    }
}

struct Uart;

impl Uart {
    unsafe fn init() {
        let cr = 0xE0000000 as *mut u32;
        ptr::write_volatile(cr, 1 << 4);
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let sr = 0xE000002C as *mut u32;
        let ch = 0xE0000030 as *mut u32;

        for b in s.as_bytes() {
            while unsafe { ptr::read_volatile(sr) & (1 << 3) == 0 } {}
            unsafe { ptr::write_volatile(ch, *b as u32) };
        }

        Ok(())
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
