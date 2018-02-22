global x86_start
extern kmain

section .text
bits 32
x86_start:
	mov ax, 0x10	; 16 bytes (0x10) is where the offset for data section (gdt_ds)
	mov ss, ax
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax

	call kmain

; if main return, loop forever ; that should NEVER append
	cli ; clear interrupt
HALT:
	hlt
	jmp HALT
