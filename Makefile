SHELL	:= /bin/bash

ifeq ($(shell whoami), william)
	PORT := 4242
	PORTG := 4244
else
	PORT := 4342
	PORTG := 4344
endif

project	:= bluesnow
arch	?= x86
NASM	:= nasm -f elf
LD		:= ld -m elf_i386 -n --gc-sections
QEMU	:= qemu-system-x86_64 -gdb tcp::$(PORTG) -enable-kvm -monitor telnet:127.0.0.1:$(PORT),server,nowait

kernel	:= build/kernel-$(arch).bin
iso		:= build/os-$(arch).iso
DIRISO	:= build/isofiles

target	?= $(arch)-$(project)
rust_os	:= target/$(target)/debug/lib$(project).a

linker_script	:= src/arch/$(arch)/linker.ld
grub.cfg		:= src/arch/$(arch)/grub.cfg
asm_source		:= $(wildcard src/arch/$(arch)/*.asm)
asm_object		:= $(patsubst src/arch/$(arch)/%.asm, build/arch/$(arch)/%.o, $(asm_source))

KERNEL_RUN		:= $(QEMU) -curses -cdrom $(iso)
MONITOR 		:= sleep 0.5;\
	telnet 127.0.0.1 $(PORT);\
	kill \`ps -x | grep gdb | head -n 2 | tail -n 1 | cut -d \  -f 1 \`
GDB 			:= gdb -q\
	-ex \"set arch i386:x86-64\"\
	-ex \"file build/kernel-x86.bin\"\
	-ex \"target remote localhost:$(PORTG)\"\
	-ex \"continue\"

all: $(kernel)

build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm Makefile
	@mkdir -p $(shell dirname $@)
	@$(NASM) $< -o $@

$(kernel): $(rust_os) $(asm_object) $(linker_script) Makefile
	@$(LD) -o $(kernel) -T $(linker_script) $(asm_object) $(rust_os)

$(iso): $(kernel) $(grub.cfg) Makefile
	@mkdir -p $(DIRISO)/boot/grub
	@cp $(grub.cfg) $(DIRISO)/boot/grub
	@cp $(kernel) $(DIRISO)/boot/kernel.bin
	@grub-mkrescue -o $(iso) $(DIRISO) 2>/dev/null

run: $(iso) Makefile
	@tmux info >&- || { echo -e "\033[38;5;16m ~~ NOT IN A VALID TMUX SESSION ~~\033[0m" ; exit 1; }
	@tmux new-window 'tmux split-window -h "$(MONITOR)"; tmux split-window -fv "$(GDB)"; tmux select-pane -t 1; tmux resize-pane -x 80 -y 25; $(KERNEL_RUN)'
	@# @$(QEMU) -curses -cdrom $(iso)

clean:
	@cargo clean
	@rm -r build

$(rust_os): $(target).json Makefile
	@RUST_TARGET_PATH="$(shell pwd)" xargo build --target $(target)

kernel: $(rust_os)
iso: $(iso)

.PHONY: run clean kernel iso $(rust_os)
