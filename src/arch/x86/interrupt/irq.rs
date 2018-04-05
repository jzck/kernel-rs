use x86::structures::idt::*;
use arch::x86::device::pic;

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
        } else {
            pic::MASTER.mask_clear(irq as u8);
        }
    }
}

interrupt!(pit, {});

interrupt!(keyboard, {
    unsafe { trigger(1); }

    println!("IT WOOOOOOOOOORKS WOUHOU!!!!!!");
    flush!();

    unsafe { acknowledge(1); }
});

interrupt!(cascade, {});
interrupt!(com2, {});
interrupt!(com1, {});
interrupt!(lpt2, {});
interrupt!(floppy, {});
interrupt!(lpt1, {});
interrupt!(rtc, {});
interrupt!(pci1, {});
interrupt!(pci2, {});
interrupt!(pci3, {});
interrupt!(mouse, {});
interrupt!(fpu, {});
interrupt!(ata1, {});
interrupt!(ata2, {});
