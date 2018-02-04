arch	?= x86_64
kernel	:= build/kernel-$(arch).bin
iso		:= build/os-$(arch).iso

target	?= $(arch)-KFS
rust_os	:= target/$(target)/debug/libkfs.a

linker_script		:= src/arch/$(arch)/linker.ld
grub.cfg			:= src/arch/$(arch)/grub.cfg
asm_source_files	:= $(wildcard src/arch/$(arch)/*.asm)
asm_object_files	:= $(patsubst src/arch/$(arch)/%.asm, \
	build/arch/$(arch)/%.o, $(asm_source_files)) 

.PHONY: all clean run iso kernel

all: $(kernel)

clean:
	@rm -r build

run: $(iso)
	@qemu-system-x86_64 -cdrom $(iso)

iso: $(iso)

$(iso): $(kernel) $(grub.cfg)
	@mkdir -p build/isofiles/boot/grub
	cp $(kernel) build/isofiles/boot/kernel.bin
	cp $(grub.cfg) build/isofiles/boot/grub
	grub-mkrescue -o $(iso) build/isofiles 2>/dev/null
	rm -r build/isofiles

$(kernel): kernel $(asm_object_files) $(linker_script)
	@ld -n -T $(linker_script) -o $(kernel) \
		$(asm_object_files) $(rust_os)

kernel:
	@xargo build --target $(target)

# compile asm files
build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 $< -o $@
