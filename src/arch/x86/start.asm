global x86_start
extern kmain

section .text
bits 32
x86_start:
; we should clear register but it does not work, it's okay with 0x10 instead of 0
mov ax, 0x10
mov ss, ax
mov ds, ax
mov es, ax
mov fs, ax
mov gs, ax

; PRINT OK
; mov dword [0xb8000], 0x2f4b2f4f
; hlt

	call kmain

; if main return, loop forever ; that should NEVER append
	cli ; clear interrupt
HALT:
	hlt
	jmp HALT
