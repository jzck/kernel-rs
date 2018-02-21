use super::{Color, ColorCode};
use ::context::CONTEXT;
use cpuio;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::vga::buffer::print(format_args!($($arg)*));
    });
}

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    unsafe { CONTEXT.current_term().write_fmt(args).unwrap() };
    unsafe { CONTEXT.current_term().flush() };
}

extern crate core;

const BUFFER_ROWS: usize = 25;
const BUFFER_COLS: usize = 80 * 2;

pub struct Writer {
    pub position: usize,
    pub color_code: ColorCode,
    buffer: [u8; BUFFER_ROWS * BUFFER_COLS],
}

impl Writer {
    pub const fn new() -> Writer {
        Writer {
            position: 0,
            color_code: ColorCode::new(Color::White, Color::Black),
            buffer: [0; BUFFER_ROWS * BUFFER_COLS],
        }
    }

    pub fn keypress(&mut self, ascii: u8) {
        self.write_byte(ascii);
        self.flush();
    }

    pub fn write_byte(&mut self, byte: u8) {
        let i = self.position;

        match byte {
            b'\n' => {
                let current_line = self.position / (BUFFER_COLS);
                self.position = (current_line + 1) * BUFFER_COLS;
            }
            byte => {
                self.buffer[i] = byte;
                self.buffer[i + 1] = self.color_code.0;
                self.position += 2;
            }
        }

        if self.position >= self.buffer.len() {
            self.scroll();
        }

        let cursor_position = self.position / 2;
        cpuio::outb(14, 0x3D4);
        cpuio::outb((cursor_position >> 8) as u8, 0x3D5);
        cpuio::outb(15, 0x3D4);
        cpuio::outb((cursor_position >> 0) as u8 & 0x00ff, 0x3D5);
    }

    pub fn flush(&mut self) {
        let slice = unsafe { core::slice::from_raw_parts_mut(0xb8000 as *mut u8, 4000) };
        slice.as_mut().clone_from_slice(&self.buffer);
    }

    fn scroll(&mut self) {
        for row in 1..BUFFER_ROWS {
            for col in 0..BUFFER_COLS {
                let prev_position = ((row - 1) * BUFFER_COLS) + col;
                let current_position = (row * BUFFER_COLS) + col;
                self.buffer[prev_position] = self.buffer[current_position];
            }
        }

        for col in 0..BUFFER_COLS/2 {
            self.buffer[((BUFFER_ROWS - 1) * BUFFER_COLS) + (col * 2)] = b' ';
        }

        self.position = (BUFFER_ROWS - 1) * BUFFER_COLS;
    }
}

use core::fmt;
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte)
        }
        Ok(())
    }
}
