ENTRY(start)
OUTPUT_FORMAT(elf32-i386)

SECTIONS {
	/* GDT for the win */
	. = 0x800;
	.gdt : {KEEP(*(.gdt))}
	 /* VGA, cannot use section for this */
	. = 0xb8000;
	VGA_PTR = . ;
	. += 80 * 25 * 2;

	. = 1M;
	/* ensure that the multiboot header is at the beginning */
	.multiboot_header : { KEEP(*(.multiboot_header)) }
	.text : { *(.text .text.*) }
	.rodata : { *(.rodata .rodata.*) }
	.data : { *(.data.rel.ro.local*) *(.data.rel.ro .data.rel.ro.*) *(.data.*) }
	.debug : { *(.debug_*) }
	.bss : {*(.bss .bss.*)}
}
