grub-cfg	:= src/arch/$(ARCH)/grub.cfg
isodir		:= build/isofiles

$(ISO): $(KERNEL) $(grub-cfg) Makefile
	@mkdir -p $(isodir)/boot/grub
	@cp $(grub-cfg) $(isodir)/boot/grub
	@cp $(KERNEL) $(isodir)/boot/$(OS)
	grub-mkrescue -o $@ $(isodir) 2>/dev/null
