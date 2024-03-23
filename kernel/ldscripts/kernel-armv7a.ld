INCLUDE memory.ld

PROVIDE(__NUM_CPU = 4);
PROVIDE(__CPU_STACKSIZE = 0x1000);

PROVIDE(__PHYS_BASE_ADDR = 0x0);
PROVIDE(__KERNEL_VIRT_BASE_ADDR = 0xE0000000);

ENTRY(_start);

SECTIONS {
	. = __KERNEL_VIRT_BASE_ADDR;
	__KERNEL_OFFSET = __KERNEL_VIRT_BASE_ADDR - __PHYS_BASE_ADDR;

	.kernel.boot ALIGN(64K) : AT(__PHYS_BASE_ADDR) {
		*(.kernel.boot.text)
		*(.kernel.boot.rodata)
		*(.kernel.boot.data)
	}

	. = ALIGN(64K);
	ki_boot_end = .;

	.kernel.text ALIGN(64K) : {
		*(.kernel.vectors)
		*(.kernel.vectors.fastpath_call)
		*(.kernel.vectors.fastpath_reply_recv)
		*(.kernel.vectors.text)
		*(.kernel.vectors.*)
		*(.kernel.text)
	}

	.kernel.rodata : {
		*(.kernel.rodata)
		*(.kernel.rodata.*)
	}

	.kernel.data : {
		*(.kernel.data)
	}

	.kernel.idle_thread : {
		*(.kernel._idle_thread)
	}

	.kernel.boot.bss (NOLOAD) : {
		__kernel_boot_bss_start = .;

		*(.kernel.boot.bss)

		__kernel_boot_bss_end = .;
	}

	.kernel.bss (NOLOAD) : {
		__kernel_bss_start = .;

		*(.kernel.bss)
		libkernel.a:(COMMON)

		_breakpoint_stack_bottom = .;
		. += 4K;
		_breakpoint_stack_top = .;

		*(.kernel.bss.aligned)

		__kernel_bss_end = .;
	}

	. = ALIGN(4K);
	ki_end = .;

	. -= __KERNEL_OFFSET;

	.devicetree ALIGN(4K) : AT(ADDR(.devicetree)) {
		__devicetree_start = .;

		KEEP(*(.kernel.devicetree))
		__devicetree_size = . - __devicetree_start;

		. = ALIGN(1M) - 256K;
		__devicetree_end = .;
	}

    .kernel_init.text ALIGN(4K) : AT(ADDR(.kernel_init.text)) {
		__text_start = .;

		KEEP(*(.vectors))
		*(.text .text.*)

		. = ALIGN(4);
		__text_end = .;
    }

	.kernel_init.rodata ALIGN(4K) : {
		__rodata_start = .;

		*(.rodata .rodata.*)

		. = ALIGN(4);
		__rodata_end = .;
	}

	.kernel_init.data ALIGN(4K) : {
		__data_start = .;

		*(.data .data.*)

		. = ALIGN(4);
		__data_end = .;
	}

	.kernel_init.bss ALIGN(4) (NOLOAD) : {
		__bss_start = .;

		*(.bss .bss.*)
		*(COMMON)

		. = ALIGN(4);
		__bss_end = .;
	}

	.kernel_init.stack ALIGN(4K) (NOLOAD) : {
		__stack_start = .;

		. += __NUM_CPU * __CPU_STACKSIZE;

		. = ALIGN(8);
		__stack_end = .;
	}

	. = ALIGN(1M) + 1M;

	.rootserver.text ALIGN(4K) : {
		*(.rootserver.text)
	}

	.ARM.attributes : {
		*(.kernel.ARM.attributes)
	}

	.ARM.exidx : {
		*(.kernel.ARM.exidx)
	}

	.debug_line : { *(.kernel.debug_line) }
	.debug_line_str : { *(.kernel.debug_line_str) }
	.debug_info : { *(.kernel.debug_info) }
	.debug_abbrev : { *(.kernel.debug_abbrev) }
	.debug_aranges : { *(.kernel.debug_aranges) }
	.debug_str : { *(.kernel.debug_str) }
	.debug_loclists : { *(.kernel.debug_loclists) }
	.debug_rnglists : { *(.kernel.debug_rnglists) }
	.debug_frame : { *(.kernel.debug_frame) }

	/DISCARD/ : {
		*(*.comment)
		*(*.note.*)
	}
}