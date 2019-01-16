QEMU_SOCKET := /tmp/qemu.sock
QEMU_MONITOR := socat - unix-connect:$(QEMU_SOCKET)
QEMU_GDB_PORT := 4242

qemu:
	qemu-system-i386\
		-cdrom $(ISO)\
		-S\
		-enable-kvm\
		-curses\
		-gdb tcp::$(QEMU_GDB_PORT)\
		-monitor unix:${QEMU_SOCKET},server,nowait

qemu-gdb:
	gdb\
	    	-q\
		-symbols "$(KERNEL)" \
		-ex "target remote :$(QEMU_GDB_PORT)"\
		-ex "set arch i386"

qemu-monitor:
	$(QEMU_MONITOR)
qemu-reload:
	echo "stop" | $(QEMU_MONITOR) &>/dev/null
	echo "change ide1-cd0 $(ISO)" | $(QEMU_MONITOR) &>/dev/null
	echo "system_reset" | $(QEMU_MONITOR) &>/dev/null
