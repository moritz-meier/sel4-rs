INCLUDE memory.ld

PROVIDE(__NUM_CPU = 4);
PROVIDE(__CPU_STACKSIZE = 0x1000);

ENTRY(_start);

SECTIONS {
	. = ORIGIN(RAM);

    .text ALIGN(0x1000) : AT(ADDR(.text)) {
		__text_start = .;

		*(.vectors)
		*(.text .text.*)

		. = ALIGN(4);
		__text_end = .;
    } > RAM

	.rodata ALIGN(0x1000) : {
		__rodata_start = .;

		*(.rodata .rodata.*)

		. = ALIGN(4);
		__rodata_end = .;
	} > RAM

	.data ALIGN(0x1000) : {
		__data_start = .;

		*(.data .data.*)

		. = ALIGN(4);
		__data_end = .;
	} > RAM

	.bss ALIGN(4) (NOLOAD) : {
		__bss_start = .;

		*(.bss .bss.*)
		*(COMMON)

		. = ALIGN(4);
		__bss_end = .;
	} > RAM

	.stack ALIGN(0x1000) (NOLOAD) : {
		__stack_start = .;

		. += __NUM_CPU * __CPU_STACKSIZE;

		. = ALIGN(8);
		__stack_end = .;
	} > RAM

	/DISCARD/ : {
		*(*.comment)
		*(*.ARM.*)
		*(*.note.*)
	}
}