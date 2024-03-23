use core::{
    marker::PhantomData,
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
    ptr::addr_of,
};

use crate::{asm::*, registers::*};

use super::{attributes::*, bitfields};

pub struct PageTable {
    entries: PageTableEntries,
    virt_addr: usize,
}

impl PageTable {
    pub const fn new(virt_addr: usize) -> Self {
        assert!(
            virt_addr & 0x000F_FFFF == 0,
            "Virtual address of page table region not aligned to 1MiB."
        );

        Self {
            entries: PageTableEntries::EMPTY,
            virt_addr,
        }
    }

    pub(super) fn virt_addr(&self) -> usize {
        self.virt_addr
    }

    pub unsafe fn map_page(&mut self, page: &Page) {
        self.entries[(page.virt_addr >> 12) & 0xFF].write_page(page.phys_addr, page.attrs);
        isb()
    }

    pub unsafe fn unmap_page(&mut self, page: Page) {
        self.entries[(page.virt_addr >> 12) & 0xFF].invalidate();
        isb()
    }

    pub(super) fn base_ptr(&self) -> *const u32 {
        addr_of!(self.entries) as *const u32
    }
}

#[repr(align(0x400))]
struct PageTableEntries {
    entries: [PageTableEntry; 256],
}

impl PageTableEntries {
    #[allow(clippy::declare_interior_mutable_const)]
    const EMPTY: Self = Self {
        entries: [PageTableEntry::INVALID; 256],
    };
}

impl Deref for PageTableEntries {
    type Target = [PageTableEntry];

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}

impl DerefMut for PageTableEntries {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entries
    }
}

union PageTableEntry {
    invalid: ManuallyDrop<InMemoryRegister<u32, ()>>,
    page: ManuallyDrop<InMemoryRegister<u32, bitfields::Page::Register>>,
}

impl PageTableEntry {
    #[allow(clippy::declare_interior_mutable_const)]
    const INVALID: Self = Self {
        invalid: ManuallyDrop::new(InMemoryRegister::new(0x0)),
    };

    const DEFAULT_PAGE_ENTRY: FieldValue<u32, bitfields::Page::Register> =
        FieldValue::<u32, bitfields::Page::Register>::new(0xFFFF_FFFF, 0, 0x2);

    unsafe fn invalidate(&mut self) {
        self.invalid.set(0)
    }

    unsafe fn write_page(&mut self, phys_addr: usize, attrs: MemoryAttributes) {
        self.page.write(
            Self::DEFAULT_PAGE_ENTRY
                + bitfields::Page::BASE_ADDR.val(phys_addr as u32 >> 12)
                + attrs.encode(),
        );
    }
}

pub struct Page<T = u32> {
    virt_addr: usize,
    phys_addr: usize,
    attrs: MemoryAttributes,
    _phantom: PhantomData<T>,
}

impl<T> Page<T> {
    pub const fn new(virt_addr: usize, phys_addr: usize, attrs: MemoryAttributes) -> Self {
        assert!(
            virt_addr & 0x0000_0FFF == 0,
            "Virtual address of page not aligned to 4KiB."
        );
        assert!(
            phys_addr & 0x0000_0FFF == 0,
            "Physical address of section not aligned to 4KiB."
        );

        Self {
            virt_addr,
            phys_addr,
            attrs,
            _phantom: PhantomData,
        }
    }

    pub fn virt_addr(&self) -> usize {
        self.virt_addr
    }

    pub fn phys_addr(&self) -> usize {
        self.phys_addr
    }

    pub fn attrs(&self) -> MemoryAttributes {
        self.attrs
    }
}

impl<T> Deref for Page<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.virt_addr as *const T) }
    }
}

impl<T> DerefMut for Page<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self.virt_addr as *mut T) }
    }
}
