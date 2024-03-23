INCLUDE memory.ld

PROVIDE(__NUM_CPU = 4);
PROVIDE(__CPU_STACKSIZE = 0x1000);

PROVIDE(__KERNEL_PHYS_BASE_ADDR = 0x0);
PROVIDE(__KERNEL_VIRT_BASE_ADDR = 0xE0000000);

ENTRY(_start);

SECTIONS {
	. = __kernel_end - __kernel_offset;

	/* -------------------------------------------------------------------------------------------- */
	/* Physical Memory (LMA == VMA)																	*/
	/* -------------------------------------------------------------------------------------------- */

	.devicetree ALIGN(4K) : AT(ADDR(.devicetree)) {
		__devicetree_start = .;

		KEEP(*(.kernel.devicetree))
		__builtin_devicetree_size = . - __devicetree_start;

		. = __devicetree_start + 64K;
		__devicetree_end = .;
	}

	__kernel_init_start = .;

    .kernel_init.text ALIGN(64K) : AT(ADDR(.kernel_init.text)) {
		__text_start = .;

		KEEP(*(.vectors))
		*(.text .text.*)

		__text_end = .;
    }

	.kernel_init.rodata : {
		__rodata_start = .;

		*(.rodata .rodata.*)

		__rodata_end = .;
	}

	.kernel_init.data : {
		__data_start = .;

		*(.data .data.*)

		__data_end = .;
	}

	.kernel_init.bss (NOLOAD) : {
		__bss_start = .;

		*(.bss .bss.*)
		*(COMMON)

		__bss_end = .;
	}

	.kernel_init.stack (NOLOAD) : {
		__stack_start = .;

		. += __NUM_CPU * __CPU_STACKSIZE;

		__stack_end = .;
	}

	__kernel_init_end = .;

	/* -------------------------------------------------------------------------------------------- */
	/* Device Virtual Memory																		*/
	/* -------------------------------------------------------------------------------------------- */

	. = (__KERNEL_VIRT_BASE_ADDR & 0xFFF00000) - 1M;

	.kernel_init.devices ALIGN(1M) (NOLOAD) : AT(ALIGN(__kernel_init_end, 1M)) {
		__device_memory_start = .;
		. += 1M;
		__device_memory_end = .;
	}

	/* -------------------------------------------------------------------------------------------- */
	/* Kernel Virtual Memory																		*/
	/* -------------------------------------------------------------------------------------------- */

	. = __KERNEL_VIRT_BASE_ADDR;

	__kernel_start = .;
	__kernel_offset = __KERNEL_VIRT_BASE_ADDR - __KERNEL_PHYS_BASE_ADDR;

	.kernel.boot ALIGN(64K) : AT(__KERNEL_PHYS_BASE_ADDR) {
		*(.kernel.boot.text)
		*(.kernel.boot.rodata)
		*(.kernel.boot.data)

		. = ALIGN(64K);
	}

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

	.kernel.boot.bss ALIGN(4) (NOLOAD) : {
		__kernel_boot_bss_start = .;

		*(.kernel.boot.bss)

		. = ALIGN(4);
		__kernel_boot_bss_end = .;
	}

	.kernel.bss ALIGN(4) (NOLOAD) : {
		__kernel_bss_start = .;

		*(.kernel.bss)
		/* libkernel.a:(COMMON)*/

		_breakpoint_stack_bottom = .;
		. += 4K;
		_breakpoint_stack_top = .;

		*(.kernel.bss.aligned)

		. = ALIGN(4);
		__kernel_bss_end = .;
	}

	. = ALIGN(4K);
	ki_end = .;

	__kernel_end = .;

	/* -------------------------------------------------------------------------------------------- */
	/* Misc																							*/
	/* -------------------------------------------------------------------------------------------- */

	.ARM.attributes : {
		*(*.ARM.attributes)
	}

	.debug_line : { *(*.debug_line) }
	.debug_line_str : { *(*.debug_line_str) }
	.debug_info : { *(*.debug_info) }
	.debug_abbrev : { *(*.debug_abbrev) }
	.debug_aranges : { *(*.debug_aranges) }
	.debug_str : { *(*.debug_str) }
	.debug_loclists : { *(*.debug_loclists) }
	.debug_rnglists : { *(*.debug_rnglists) }
	.debug_frame : { *(*.debug_frame) }

	/DISCARD/ : {
		*(*.ARM.exidx)
		*(*.comment)
		*(*.note *.note.*)
	}
}