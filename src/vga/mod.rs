pub mod color;
pub mod cursor;

pub use self::color::{Color, ColorCode};
use self::cursor::CURSOR;

pub static mut VGA: Writer = self::Writer::new();

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

// print wrapper macro around vga
#[allow(unused_macros)]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::vga::print(format_args!($($arg)*));
    });
}

// flushed print
#[allow(unused_macros)]
macro_rules! fprint {
    ($($arg:tt)*) => ({
        print!($($arg)*);
        flush!();
    });
}

// print with a line feed
#[allow(unused_macros)]
macro_rules! println {
    () => ({print!("\n")});
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

// flushed println
#[allow(unused_macros)]
macro_rules! fprintln {
    ($($arg:tt)*) => ({
        println!($($arg)*);
        flush!();
    });
}

macro_rules! flush {
    () => (#[allow(unused_unsafe)] unsafe { $crate::vga::VGA.flush() });
}

macro_rules! set_color {
    () => (unsafe { $crate::vga::VGA.color_code =
        $crate::vga::ColorCode::new($crate::vga::Color::White, $crate::vga::Color::Black)} );
    ($fg:ident) => (unsafe { $crate::vga::VGA.color_code =
        $crate::vga::ColorCode::new($crate::vga::Color::$fg, $crate::vga::Color::Black)} );
    ($fg:ident, $bg:ident) => (unsafe { $crate::vga::VGA.color_code =
        $crate::vga::ColorCode::new($crate::vga::Color::$fg, $crate::vga::Color::$bg)} );
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    unsafe {
        self::VGA.write_fmt(args).unwrap();
    }
}

extern crate core;

const BUFFER_ROWS: usize = 25;
const BUFFER_COLS: usize = 80 * 2;

pub struct Writer {
    pub buffer_pos: usize,
    pub color_code: ColorCode,
    buffer: [u8; BUFFER_ROWS * BUFFER_COLS],
}

impl Writer {
    pub const fn new() -> Writer {
        Writer {
            buffer_pos: 0,
            color_code: ColorCode::new(Color::White, Color::Black),
            buffer: [0; BUFFER_ROWS * BUFFER_COLS],
        }
    }

    pub fn erase_byte(&mut self) {
        self.buffer_pos -= 2;
        let i = self.buffer_pos;
        self.buffer[i] = b' ';
        self.buffer[i + 1] = self.color_code.0;
        self.flush();
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => {
                let current_line = self.buffer_pos / (BUFFER_COLS);
                self.buffer_pos = (current_line + 1) * BUFFER_COLS;
            }
            byte => {
                self.buffer[self.buffer_pos] = byte;
                self.buffer[self.buffer_pos + 1] = self.color_code.0;
                self.buffer_pos += 2;
            }
        }

        if self.buffer_pos >= self.buffer.len() {
            self.scroll();
            self.flush();
        }
        // flushing here is correct but slow
        // self.flush();
    }

    pub fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte)
        }
    }

    fn flush_cursor(&self) {
        unsafe { CURSOR.flush(self.buffer_pos / 2); }
    }

    pub fn flush(&mut self) {
        let slice = unsafe { core::slice::from_raw_parts_mut(0xb8000 as *mut u8, 4000) };
        slice.as_mut().clone_from_slice(&self.buffer);
        self.flush_cursor();
    }

    fn scroll(&mut self) {
        for row in 1..BUFFER_ROWS {
            for col in 0..BUFFER_COLS {
                let prev_position = ((row - 1) * BUFFER_COLS) + col;
                let current_position = (row * BUFFER_COLS) + col;
                self.buffer[prev_position] = self.buffer[current_position];
            }
        }

        for col in (0..BUFFER_COLS / 2).map(|x| x * 2) {
            self.buffer[((BUFFER_ROWS - 1) * BUFFER_COLS) + (col)] = b' ';
            self.buffer[((BUFFER_ROWS - 1) * BUFFER_COLS) + (col + 1)] =
                ColorCode::new(Color::White, Color::Black).0;
        }
        self.buffer_pos = (BUFFER_ROWS - 1) * BUFFER_COLS;
    }
}

// trait needed by formatting macros
use core::fmt;
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte)
        }
        Ok(())
    }
}
