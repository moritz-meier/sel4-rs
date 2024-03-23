ENTRY(_start);

PROVIDE(__STACK_SIZE = 0x1000);

SECTIONS {
    . = 0x00100000;

    .text ALIGN(0x1000) : AT(ADDR(.text)) {
		__text_start = .;

		*(.start)
		*(.text .text.*)

		. = ALIGN(4);
		__text_end = .;
    }

	.rodata ALIGN(0x1000) : {
		__rodata_start = .;

		*(.rodata .rodata.*)

		. = ALIGN(4);
		__rodata_end = .;
	}

	.data ALIGN(0x1000) : {
		__data_start = .;

		*(.data .data.*)

		. = ALIGN(4);
		__data_end = .;
	}

	.bss ALIGN(4) (NOLOAD) : {
		__bss_start = .;

		*(.bss .bss.*)
		*(COMMON)

		. = ALIGN(4);
		__bss_end = .;
	}

	.stack ALIGN(0x1000) (NOLOAD) : {
		__stack_start = .;

		. += __STACK_SIZE;

		. = ALIGN(8);
		__stack_end = .;
	}

	/DISCARD/ : {
		*(*.comment)
		*(*.ARM.*)
		*(*.note.*)
	}
}