use core::{
    marker::PhantomData,
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
    ptr::addr_of,
};

use crate::{asm::*, registers::*};

use super::{attributes::*, bitfields, level2::*};

pub struct PageDirectory {
    entries: PageDirectoryEntries,
}

impl PageDirectory {
    pub const fn new() -> Self {
        Self {
            entries: PageDirectoryEntries::EMPTY,
        }
    }

    pub unsafe fn map_section<T>(&mut self, section: &Section<T>) {
        self.entries[section.virt_addr >> 20].write_section(section.phys_addr, section.attrs);
        isb();
    }

    pub unsafe fn unmap_section<T>(&mut self, section: Section<T>) {
        self.entries[section.virt_addr >> 20].invalidate();
        isb()
    }

    pub unsafe fn map_page_table(&mut self, pt: &PageTable) {
        self.entries[pt.virt_addr() >> 20].write_page_table(pt.base_ptr());
        isb()
    }

    pub unsafe fn unmap_page_table(&mut self, pt: PageTable) {
        self.entries[pt.virt_addr() >> 20].invalidate();
        isb()
    }

    pub(super) fn base_ptr(&self) -> *const u32 {
        addr_of!(self.entries) as *const u32
    }
}

impl Default for PageDirectory {
    fn default() -> Self {
        Self {
            entries: PageDirectoryEntries::EMPTY,
        }
    }
}

#[repr(align(0x4000))]
struct PageDirectoryEntries {
    pub(super) entries: [PageDirectoryEntry; 4096],
}

impl PageDirectoryEntries {
    #[allow(clippy::declare_interior_mutable_const)]
    const EMPTY: Self = Self {
        entries: [PageDirectoryEntry::INVALID; 4096],
    };
}

impl Deref for PageDirectoryEntries {
    type Target = [PageDirectoryEntry];

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}

impl DerefMut for PageDirectoryEntries {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entries
    }
}

union PageDirectoryEntry {
    invalid: ManuallyDrop<InMemoryRegister<u32, ()>>,
    section: ManuallyDrop<InMemoryRegister<u32, bitfields::Section::Register>>,
    page_table: ManuallyDrop<InMemoryRegister<u32, bitfields::PageTable::Register>>,
}

impl PageDirectoryEntry {
    #[allow(clippy::declare_interior_mutable_const)]
    const INVALID: Self = Self {
        invalid: ManuallyDrop::new(InMemoryRegister::new(0x0)),
    };

    const DEFAULT_SECTION_ENTRY: FieldValue<u32, bitfields::Section::Register> =
        FieldValue::<u32, bitfields::Section::Register>::new(0xFFFF_FFFF, 0, 0x2);

    const DEFAULT_PAGE_TABLE_ENTRY: FieldValue<u32, bitfields::PageTable::Register> =
        FieldValue::<u32, bitfields::PageTable::Register>::new(0xFFFF_FFFF, 0, 0x1);

    unsafe fn invalidate(&mut self) {
        self.invalid.set(0)
    }

    unsafe fn write_section(&mut self, phys_addr: usize, attrs: MemoryAttributes) {
        self.section.write(
            Self::DEFAULT_SECTION_ENTRY
                + bitfields::Section::BASE_ADDR.val((phys_addr >> 20) as u32)
                + attrs.encode(),
        );
    }

    unsafe fn write_page_table(&mut self, base_ptr: *const u32) {
        let base_addr = base_ptr as usize;
        self.page_table.write(
            Self::DEFAULT_PAGE_TABLE_ENTRY
                + bitfields::PageTable::BASE_ADDR.val(base_addr as u32 >> 10),
        )
    }
}

pub struct Section<T = u32> {
    virt_addr: usize,
    phys_addr: usize,
    attrs: MemoryAttributes,
    _phantom: PhantomData<T>,
}

impl<T> Section<T> {
    pub const fn new(virt_addr: usize, phys_addr: usize, attrs: MemoryAttributes) -> Self {
        assert!(
            virt_addr & 0x000F_FFFF == 0,
            "Virtual address of section not aligned to 1MiB."
        );
        assert!(
            phys_addr & 0x000F_FFFF == 0,
            "Physical address of section not aligned to 1MiB."
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

impl<T> Deref for Section<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.virt_addr as *const T) }
    }
}

impl<T> DerefMut for Section<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self.virt_addr as *mut T) }
    }
}
