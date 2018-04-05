use x86::structures::idt::*;
use arch::x86::device::pic;

#[macro_export]
macro_rules! interrupt {
    ($i:expr, $name:ident, $func:block) => {
        pub extern "x86-interrupt" fn $name(stack_frame: &mut ExceptionStackFrame)
        {
            unsafe { trigger(1); }
            #[allow(unused_variables)]
            fn inner(stack: &mut ExceptionStackFrame) {
                $func
            }
            inner(stack_frame);
            unsafe { acknowledge(1); }
        }
    }
}

pub unsafe fn trigger(irq: u8) {
    if irq < 16 {
        if irq >= 8 {
            pic::SLAVE.mask_set(irq - 8);
            pic::MASTER.ack();
            pic::SLAVE.ack();
        } else {
            pic::MASTER.mask_set(irq);
            pic::MASTER.ack();
        }
    }
}

pub unsafe fn acknowledge(irq: usize) {
    if irq < 16 {
        if irq >= 8 {
            pic::SLAVE.mask_clear(irq as u8 - 8);
            pic::SLAVE.ack();
        } else {
            pic::MASTER.mask_clear(irq as u8);
            pic::MASTER.ack();
        }
    }
}

interrupt!(0, pit, {});

interrupt!(1, keyboard, {
    ::keyboard::kbd_callback();
});

interrupt!(2, cascade, {});
interrupt!(3, com2, {});
interrupt!(4, com1, {});
interrupt!(5, lpt2, {});
interrupt!(6, floppy, {});
interrupt!(7, lpt1, {});
interrupt!(8, rtc, {});
interrupt!(9, pci1, {});
interrupt!(10, pci2, {});
interrupt!(11, pci3, {});
interrupt!(12, mouse, {});
interrupt!(13, fpu, {});
interrupt!(14, ata1, {});
interrupt!(15, ata2, {});
