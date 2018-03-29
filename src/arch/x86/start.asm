global x86_start
extern x86_rust_start

section .text
bits 32
x86_start:
	mov ax, 0x10	; 16 bytes (0x10) is where the offset for data section (gdt_ds)
	mov ds, ax
	mov ss, ax
	mov es, ax
	mov fs, ax
	mov gs, ax

	call x86_rust_start

	cli ; clear interrupt
HALT:
	hlt
	jmp HALT
