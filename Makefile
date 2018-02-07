arch	?= x86_64
kernel	:= build/kernel-$(arch).bin
iso		:= build/os-$(arch).iso

target	?= $(arch)-kfs
rust_os	:= target/$(target)/debug/libkfs.a

linker_script		:= src/arch/$(arch)/linker.ld
grub.cfg			:= src/arch/$(arch)/grub.cfg
asm_source_files	:= $(wildcard src/arch/$(arch)/*.asm)
asm_object_files	:= $(patsubst src/arch/$(arch)/%.asm, \
	build/arch/$(arch)/%.o, $(asm_source_files)) 

.PHONY: all clean run iso kernel
SHELL 				:= /bin/bash

all: $(kernel)

clean:
	@cargo clean
	@rm -r build

run:
	@qemu-system-x86_64 -curses -cdrom $(iso)

devrun:
	@tmux info >&- || { echo -e "\033[38;5;16m ~~ NOT IN A VALID TMUX SESSION ~~\033[0m" ; exit 1; }
	@tmux split-window "tmux resize-pane -y 20; sleep 0.5; telnet 127.0.0.1 1234"
	@qemu-system-x86_64 -enable-kvm -monitor telnet:127.0.0.1:1234,server,nowait -curses -cdrom build/os-x86_64.iso

iso: $(iso)

$(iso): $(kernel) $(grub.cfg)
	@mkdir -p build/isofiles/boot/grub
	cp $(kernel) build/isofiles/boot/kernel.bin
	cp $(grub.cfg) build/isofiles/boot/grub
	grub-mkrescue -o $(iso) build/isofiles 2>/dev/null
	rm -r build/isofiles

$(kernel): kernel $(asm_object_files) $(linker_script)
	@ld -n --gc-sections -T $(linker_script) -o $(kernel) $(asm_object_files) $(rust_os)

kernel:
	@RUST_TARGET_PATH="$(shell pwd)" xargo build --target $(target)

# compile asm files
build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 $< -o $@
