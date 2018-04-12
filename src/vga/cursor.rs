// https://wiki.osdev.org/Text_Mode_Cursor
// Protected mode cursor abstraction

use io::{Io, Pio};

pub static mut CURSOR: Cursor = Cursor::new(0x3D4);

pub struct Cursor {
    cmd: Pio<u8>,
    data: Pio<u8>,
}

impl Cursor {
    pub const fn new(port: u16) -> Cursor {
        Cursor {
            cmd: Pio::new(port),
            data: Pio::new(port + 1),
        }
    }

    pub fn flush(&mut self, position: usize) {
        // 14 awaits the rightmost 8bits
        self.cmd.write(14);
        self.data.write((position >> 8) as u8 & 0xff);

        // 15 awaits the leftmost 8bits
        self.cmd.write(15);
        self.data.write((position >> 0) as u8 & 0xff);
    }
}
