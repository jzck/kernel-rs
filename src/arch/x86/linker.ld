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
	kernel_start = . ;

	/* ensure that the multiboot header is at the beginning */
	.multiboot : { KEEP(*(.multiboot)) }
	.text : { *(.text .text.*) }
	.rodata : { *(.rodata .rodata.*) }
	.data : { *(.data.rel.ro.local*) *(.data.rel.ro .data.rel.ro.*) *(.data.*) }
	.debug : { *(.debug_*) }
	.bss : {*(.bss .bss.*)}

	/* kernel_end = . ; */
}
