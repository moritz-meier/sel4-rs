#![no_std]

extern "C" {
    pub fn _kernel_entry(
        phys_start: usize,
        phys_end: usize,
        phys_virt_offset: usize,
        virt_entry: usize,
        phys_dtb_addr: usize,
        dtb_size: usize,
    ) -> !;
}
