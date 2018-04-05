use ::io::{Io, Pio};

pub static mut MASTER: Pic = Pic::new(0x20);
pub static mut SLAVE: Pic = Pic::new(0xA0);
pub static mut WAIT_PORT: Pio<u8> = Pio::new(0x80);

pub unsafe fn init() {
    let wait = || {WAIT_PORT.write(0)};
    let master_mask = MASTER.data.read();
    let slave_mask = SLAVE.data.read();

    // Start initialization
    MASTER.cmd.write(0x11); wait();
    SLAVE.cmd.write(0x11); wait();

    // Set offsets
    MASTER.data.write(0x20); wait();
    SLAVE.data.write(0x28); wait();

    // Set up cascade
    MASTER.data.write(4); wait();
    SLAVE.data.write(2); wait();

    // Set up interrupt mode (1 is 8086/88 mode, 2 is auto EOI)
    MASTER.data.write(1); wait();
    SLAVE.data.write(1); wait();

    // Unmask interrupts
    MASTER.data.write(0); wait();
    SLAVE.data.write(0); wait();

    // Ack remaining interrupts
    MASTER.ack(); wait();
    SLAVE.ack(); wait();

    MASTER.data.write(master_mask); wait();
    SLAVE.data.write(slave_mask); wait();

    // disable all irqs
    MASTER.data.write(!0); wait();
    SLAVE.data.write(!0); wait();
    
    // keyboard active
    MASTER.mask_clear(1); wait();

    // asm!("sti");
    ::x86::instructions::interrupts::enable();
}

pub struct Pic {
    cmd: Pio<u8>,
    data: Pio<u8>,
}

impl Pic {
    pub const fn new(port: u16) -> Pic {
        Pic {
            cmd: Pio::new(port),
            data: Pio::new(port + 1),
        }
    }

    pub fn ack(&mut self) {
        self.cmd.write(0x20);
    }

    pub fn mask_set(&mut self, irq: u8) {
        assert!(irq < 8);

        let mut mask = self.data.read();
        mask |= 1 << irq;
        self.data.write(mask);
    }

    pub fn mask_clear(&mut self, irq: u8) {
        assert!(irq < 8);

        let mut mask = self.data.read();
        mask &= !(1 << irq);
        self.data.write(mask);
    }
}
