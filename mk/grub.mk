grub-iso	:= $(kernel).iso
grub-cfg	:= src/arch/$(ARCH)/grub.cfg
isodir		:= build/isofiles

$(grub-iso): $(kernel) $(grub-cfg) Makefile
	@mkdir -p $(isodir)/boot/grub
	@cp $(grub-cfg) $(isodir)/boot/grub
	@cp $(kernel) $(isodir)/boot/$(OS)
	@grub-mkrescue -o $@ $(isodir) 2>/dev/null
	@printf "\r\033[38;5;117m✓ GRUB ==> $(grub-iso)\033[0m\033[K\n"
