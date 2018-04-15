mod pio;

pub use self::pio::*;

use core::ops::{BitAnd, BitOr, Not};

pub trait Io {
    type Value: Copy
        + PartialEq
        + BitAnd<Output = Self::Value>
        + BitOr<Output = Self::Value>
        + Not<Output = Self::Value>;

    fn read(&self) -> Self::Value;
    fn write(&mut self, value: Self::Value);

    #[inline(always)]
    fn readf(&self, flags: Self::Value) -> bool {
        (self.read() & flags) as Self::Value == flags
    }

    #[inline(always)]
    fn writef(&mut self, flags: Self::Value, value: bool) {
        let tmp: Self::Value = match value {
            true => self.read() | flags,
            false => self.read() & !flags,
        };
        self.write(tmp);
    }
}

pub fn cli() {
    unsafe { asm!("cli" : : : : "volatile") };
}

pub fn halt() -> ! {
    cli();
    loop {
        unsafe { asm!("hlt" : : : : "volatile") };
    }
}
