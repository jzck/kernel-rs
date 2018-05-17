ifeq ($(shell whoami), jack)
	PORT := 4242
	PORTG := 4244
else
	PORT := 4342
	PORTG := 4344
endif

QEMU		:= qemu-system-x86_64\
	-gdb tcp::$(PORTG)\
	-S\
	-enable-kvm\
	-monitor telnet:127.0.0.1:$(PORT),server,nowait\
	-curses\
	-cdrom build/bluesnow-x86.iso

MONITOR 		:= sleep 0.5;\
	telnet 127.0.0.1 $(PORT);\
	kill \`ps -x | grep \"[g]db -q\" | cut -d \  -f 1 \`;\
	kill \`ps -x | grep \"[g]db -q\" | cut -d \  -f 2 \`
GDB 			:= gdb -q\
	-ex \"target remote :$(PORTG)\"

qemu:
	@tmux info >&- || { echo -e "\033[38;5;16mPlease run inside a tmux session\033[0m" ; exit 1; }
	@tmux new-window 'tmux split-window -h "$(MONITOR)"; tmux split-window -fv "$(GDB)"; tmux select-pane -t 1; tmux resize-pane -x 80 -y 25; $(QEMU)'

