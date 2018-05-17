SHELL	:= /bin/bash

ARCH	:= x86
OS		:= bluesnow
TARGET	?= $(ARCH)-$(OS)

all:
	@printf "make kernel\t# build kernel binary\n"
	@printf "make iso\t# build iso cdrom with grub\n"
	@printf "make qemu\t# run qemu+gdb in tmux window\n"

## COMPILE ASM (nasm)
asm_source		:= $(wildcard src/arch/$(ARCH)/*.asm)
asm_object		:= $(patsubst src/arch/$(ARCH)/%.asm, build/arch/$(ARCH)/%.o, $(asm_source))
NASM			:= /usr/bin/nasm -f elf -g
build/arch/$(ARCH)/%.o: src/arch/$(ARCH)/%.asm Makefile
	@mkdir -p $(shell dirname $@)
	$(NASM) $< -o $@

## COMPILE RUST (xargo)
rust_os	:= target/$(TARGET)/debug/lib$(OS).a
$(rust_os): $(TARGET).json Makefile
	TERM=xterm RUST_TARGET_PATH="$(shell pwd)" xargo build --target $(TARGET)

## LINKAGE
kernel			:= build/$(OS)-$(ARCH).bin
linker_script	:= src/arch/$(ARCH)/linker.ld
LD				:= /usr/bin/ld -m elf_i386 -L ./ -n --gc-sections
$(kernel): $(rust_os) $(asm_object) $(linker_script) Makefile
	$(LD) -o $@ -T $(linker_script) $(asm_object) $(rust_os)

clean:
	xargo clean
	rm -rf build

.PHONY: clean kernel iso $(rust_os)

# Emulation recipes
include mk/qemu.mk

# Bootloader recipes
include mk/grub.mk
iso: $(grub-iso)

kernel: $(kernel)
