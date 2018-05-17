grub-iso	:= $(kernel:.bin=.iso)
grub-cfg	:= src/arch/$(ARCH)/grub.cfg
isodir		:= build/isofiles

$(grub-iso): $(kernel) $(grub-cfg) Makefile
	@mkdir -p $(isodir)/boot/grub
	@cp $(grub-cfg) $(isodir)/boot/grub
	@cp $(kernel) $(isodir)/boot/$(OS)
	grub-mkrescue -o $@ $(isodir) 2>/dev/null
