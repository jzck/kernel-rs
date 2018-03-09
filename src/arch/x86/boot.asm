global start
extern x86_start

section .text
bits 32
start:
	; our stack, located in bss, linker.ld puts bss at the end of the binary
	mov esp, stack_top
	; multiboot information pointer
	push ebx

	call check_multiboot

	call set_up_page_tables
	call enable_paging

	lgdt [GDTR.ptr]					; load the new gdt
	jmp GDTR.gdt_cs:x86_start

check_multiboot:
	cmp eax, 0x36d76289
	jne .no_multiboot
	ret
.no_multiboot:
	mov al, "0"
	jmp error

set_up_page_tables:
	; map P2 table recursively
	mov eax, p2_table
	or eax, 0b11 ; present + writable
	mov [p2_table + 1023 * 4], eax

    ; map each P2 entry to a huge 2MiB page
    mov ecx, 0         ; counter variable

.map_p2_table:
    ; map ecx-th P2 entry to a huge page that starts at address 2MiB*ecx
    mov eax, 0x200000  ; 2MiB
    mul ecx            ; start address of ecx-th page
    or eax, 0b10000011 ; present + writable + huge
    mov [p2_table + ecx * 4], eax ; map ecx-th entry

    inc ecx            ; increase counter
    cmp ecx, 1023      ; if counter == 1023, the whole P2 table is mapped
    jne .map_p2_table  ; else map the next entry

    ; load P2 to cr3 register (cpu uses this to access the P2 table)
    mov eax, p2_table
    mov cr3, eax

    ret

enable_paging:
    ; enable paging in the cr0 register
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax

    ret

error:
	mov dword [0xb8000], 0x4f524f45
	mov dword [0xb8004], 0x4f3a4f52
	mov dword [0xb8008], 0x4f204f20
	mov byte  [0xb800a], al
	cli
HALT:
	hlt
	jmp HALT

section .bss
align 4096
p2_table:
    resb 4096
p1_table:
	resb 4096
stack_bottom:
    resb 4096 * 4
stack_top:

section .gdt
GDTR:
; http://tuttlem.github.io/2014/07/11/a-gdt-primer.html
.gdt_top:
	DD 0, 0
.gdt_cs: equ $ - .gdt_top; the code segment Aka KERNEL CODE
	DW 0xffff		; Limit ( bits 0 -15 )
	DW 0x0			; Base ( bits 0 -15 )
	DB 0x0			; Base ( bits 16 -23 )
	DB 0x9A			; [ Access Flags: 0x9A=10011010b = (present)|(Privilege Ring 0=00b)|(1)|(code => 1)|(expand down => 0)|(readable)|(0) ]
	DB 0xCF			; [ Flags: C=1100b = (granularity)|(32bit)|(!64bit)|(0) ] / [ Limits: (bits 16-19): F=1111b ]
	DB 0x0			; Base ( bits 24 -31 )

.gdt_ds: equ $ - .gdt_top; the data segment Aka KERNEL DATA
	DW 0xffff		; Limit ( bits 0 -15 )
	DW 0x0			; Base ( bits 0 -15 )
	DB 0x0			; Base ( bits 16 -23 )
	DB 0x92			; [ Access Flags: 0x92=10010010b = (present)|(Privilege Ring 0=00b)|(1)|(data => 0)|(expand down => 0)|(readable)|(0) ]
	DB 0xCF			; [ Flags: C=1100b = (granularity)|(32bit)|(!64bit)|(0) ] / [ Limits: (bits 16-19): F=1111b ]
	DB 0x0			; Base ( bits 24 -31 )

.gdt_ss: equ $ - .gdt_top; the stack segment Aka KERNEL STACK
	DW 0x0			; Limit ( bits 0 -15 )
	DW 0x0			; Base ( bits 0 -15 )
	DB 0x0			; Base ( bits 16 -23 )
	DB 0x96			; [ Access Flags: 0x96=10010110b = (present)|(Privilege Ring 0=00b)|(1)|(data => 0)|(expand up => 1)|(readable)|(0) ]
	DB 0xCF			; [ Flags: C=1100b = (granularity)|(32bit)|(!64bit)|(0) ] / [ Limits: (bits 16-19): F=1111b ]
	DB 0x0			; Base ( bits 24 -31 )

.gdt_es: equ $ - .gdt_top; the extra segment Aka USER CODE
	DW 0xffff		; Limit ( bits 0 -15 )
	DW 0x0			; Base ( bits 0 -15 )
	DB 0x0			; Base ( bits 16 -23 )
	DB 0xFE			; [ Access Flags: 0x9A=11111110b = (present)|(Privilege Ring 3=11b)|(1)|(code => 1)|(expand up => 1)|(readable)|(0) ]
	DB 0xCF			; [ Flags: C=1100b = (granularity)|(32bit)|(!64bit)|(0) ] / [ Limits: (bits 16-19): F=1111b ]
	DB 0x0			; Base ( bits 24 -31 )

.gdt_fs: equ $ - .gdt_top; the other segment Aka USER DATA
	DW 0xffff		; Limit ( bits 0 -15 )
	DW 0x0			; Base ( bits 0 -15 )
	DB 0x0			; Base ( bits 16 -23 )
	DB 0xF2			; [ Access Flags: 0x9A=11110010b = (present)|(Privilege Ring 3=11b)|(1)|(data => 0)|(expand down => 0)|(readable)|(0) ]
	DB 0xCF			; [ Flags: C=1100b = (granularity)|(32bit)|(!64bit)|(0) ] / [ Limits: (bits 16-19): F=1111b ]
	DB 0x0			; Base ( bits 24 -31 )

.gdt_gs: equ $ - .gdt_top; the other segment Aka USER STACK
	DW 0x0			; Limit ( bits 0 -15 )
	DW 0x0			; Base ( bits 0 -15 )
	DB 0x0			; Base ( bits 16 -23 )
	DB 0x00			; [ Access Flags: 0x9A=11110110b = (present)|(Privilege Ring 3=11b)|(1)|(data => 0)|(expand up => 1)|(readable)|(0) ]
	DB 0x00			; [ Flags: C=1100b = (granularity)|(32bit)|(!64bit)|(0) ] / [ Limits: (bits 16-19): F=1111b ]
	DB 0x0			; Base ( bits 24 -31 )

.gdt_bottom:
.ptr:
	DW .gdt_bottom - .gdt_top - 1	; length of the structure minus 1
	DD .gdt_top						; pointer to top of gdt

