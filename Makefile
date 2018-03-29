SHELL	:= /bin/bash

ARCH	:= x86
OS		:= bluesnow
target	?= $(ARCH)-$(OS)

NASM	:= /usr/bin/nasm -f elf -g
LD		:= /usr/bin/ld -m elf_i386 -L ./ -n --gc-sections
MKDIR	:= mkdir -p

kernel	:= build/$(OS)
iso		:= $(kernel).iso
DIRISO	:= build/isofiles

rust_os	:= target/$(target)/debug/lib$(OS).a

linker_script	:= src/arch/$(ARCH)/linker.ld
grub.cfg		:= src/arch/$(ARCH)/grub.cfg
asm_source		:= $(wildcard src/arch/$(ARCH)/*.asm)
asm_object		:= $(patsubst src/arch/$(ARCH)/%.asm, build/arch/$(ARCH)/%.o, $(asm_source))

all: $(kernel)

build/arch/$(ARCH)/%.o: src/arch/$(ARCH)/%.asm Makefile
	@$(MKDIR) $(shell dirname $@)
	$(NASM) $< -o $@

$(kernel): $(rust_os) $(asm_object) $(linker_script) Makefile
	$(LD) -o $@ -T $(linker_script) $(asm_object) $(rust_os)

$(iso): $(kernel) $(grub.cfg) Makefile
	@$(MKDIR) $(DIRISO)/boot/grub
	@cp $(grub.cfg) $(DIRISO)/boot/grub
	@cp $(kernel) $(DIRISO)/boot/$(OS)
	@grub-mkrescue -o $@ $(DIRISO) 2>/dev/null

clean:
	@xargo clean
	@rm -rf build

$(rust_os): $(target).json Makefile
	@RUST_TARGET_PATH="$(shell pwd)" xargo build --target $(target)

kernel: $(rust_os)
iso: $(iso)

.PHONY: clean kernel iso $(rust_os)

# Emulation recipes
include mk/qemu.mk
