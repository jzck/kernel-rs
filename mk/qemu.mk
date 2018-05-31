ifeq ($(shell whoami), jack)
	PORT_MONIT := 4242
	PORT_GDB := 4244
else
	PORT_MONIT := 4342
	PORT_GDB := 4344
endif

QEMU		:= qemu-system-i386\
	-cdrom build/bluesnow-x86.iso\
	-S\
	-enable-kvm\
	-curses\
	-gdb tcp::$(PORT_GDB)\
	-monitor telnet:127.0.0.1:$(PORT_MONIT),server,nowait
qemu:
	$(QEMU)

GDB			:= gdb -q\
	-symbols "$(kernel)" \
	-ex "target remote :$(PORT_GDB)"\
	-ex "set arch i386"
gdb:
	$(GDB)

MONITOR 	:= telnet 127.0.0.1 $(PORT_MONIT);\
	kill \`ps -x | grep \"[g]db -q\" | cut -d \  -f 1 \`;\
	kill \`ps -x | grep \"[g]db -q\" | cut -d \  -f 2 \`
monitor:
	telnet 127.0.0.1 $(PORT_MONIT)

#not using this anymore
william:
	@tmux info >&- || { echo -e "\033[38;5;16mPlease run inside a tmux session\033[0m" ; exit 1; }
	@tmux new-window 'tmux split-window -h "$(MONITOR)"; tmux split-window -fv "$(GDB)"; tmux select-pane -t 1; tmux resize-pane -x 80 -y 25; $(QEMU)'
