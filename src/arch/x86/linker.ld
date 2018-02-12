ENTRY(start)
OUTPUT_FORMAT(elf32-i386)

SECTIONS {
	. = 1M;

	.boot : {
		/* ensure that the multiboot header is at the beginning */
		KEEP(*(.multiboot_header))
	}

	.text : {
		*(.text)
	}

	.stack : {
		*(.bss)
	}
}
