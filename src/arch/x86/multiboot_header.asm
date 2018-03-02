section .multiboot_header
header_start:
align 4
	dd 0xe85250d6					; magic number (multiboot 2)
	dd 0							; TODO change it because architecture 0 means(protected mode i386) We could have problem here
	dd header_end - header_start	; header length

	dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start)); checksum

	; insert optional multiboot tags here

	; required end tag
	dw 0	; type
	dw 0	; flags
	dd 8	; size
header_end:
