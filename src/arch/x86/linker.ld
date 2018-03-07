ENTRY(start)
OUTPUT_FORMAT(elf32-i386)

SECTIONS {
	/* GDT for the win */
	. = 0x800;
	.gdt : {KEEP(*(.gdt))}

	 /* VGA, cannot use section for this */
	VGA_PTR = 0xb8000;
	. = 0xb8000;
	. += 80 * 25 * 2;

	. = 1M;
	/* ensure that the multiboot header is at the beginning */
	.multiboot :
	{
		KEEP(*(.multiboot))
		. = ALIGN(4K);
	}

	.text :
	{
		*(.text .text.*)
		. = ALIGN(4K);
	}

	.rodata :
	{
		*(.rodata .rodata.*)
		. = ALIGN(4K);
	}

	.data :
	{
		*(.data.rel.ro.local*) *(.data.rel.ro .data.rel.ro.*) *(.data.*)
		. = ALIGN(4K);
	}

	.debug :
	{
		*(.debug_*)
		. = ALIGN(4K);
	}

	.bss :
	{
		*(.bss .bss.*)}
		. = ALIGN(4K);
	}
