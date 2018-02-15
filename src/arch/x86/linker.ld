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
	kernel_start = . ;
	/* ensure that the multiboot header is at the beginning */
	.boot : {KEEP(*(.multiboot_header))}
	.text : {*(.text)}
	/* .rodata BLOCK(4K) : ALIGN(4K) {*(.rodata)} */
	/* .data BLOCK(4K) : ALIGN(4K) {*(.data)} */
/* load stack */
	.bss : {*(.bss)}
	kernel_end = . ;
}
