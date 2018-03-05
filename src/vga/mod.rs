pub mod color;

pub use self::color::{Color, ColorCode};

use context;
use cpuio;
use console;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::vga::print(format_args!($($arg)*));
    });
}

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

macro_rules! flush {
    () => (context::current_term().flush());
}

macro_rules! set_color {
    () => ($crate::context::current_term().color_code =
        $crate::vga::ColorCode::new($crate::vga::Color::White, $crate::vga::Color::Black));
    ($fg:ident) => ($crate::context::current_term().color_code =
        $crate::vga::ColorCode::new($crate::vga::Color::$fg, $crate::vga::Color::Black));
    ($fg:ident, $bg:ident) => ($crate::context::current_term().color_code =
        $crate::vga::ColorCode::new($crate::vga::Color::$fg, $crate::vga::Color::$bg));
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    context::current_term().write_fmt(args).unwrap();
}

extern crate core;

const BUFFER_ROWS: usize = 25;
const BUFFER_COLS: usize = 80 * 2;

pub struct Writer {
    pub buffer_pos: usize,
    pub color_code: ColorCode,
    buffer: [u8; BUFFER_ROWS * BUFFER_COLS], command: [u8; 10],
    command_len: usize,
}

impl Writer {
    pub const fn new() -> Writer {
        Writer {
            buffer_pos: 0,
            color_code: ColorCode::new(Color::White, Color::Black),
            buffer: [0; BUFFER_ROWS * BUFFER_COLS],
            command: [b'\0'; 10],
            command_len: 0,
        }
    }

    pub fn prompt(&mut self) {
        set_color!(Blue);
        self.write_str("> ");
        set_color!();
        flush!();
    }

    pub fn backspace(&mut self) {
        if self.command_len > 0 {
            self.command_len -= 1;
            self.erase_byte();
        }
    }

    pub fn get_command(&self) -> Result <&str, &'static str> {

        match core::str::from_utf8(&self.command) {
            Ok(y) => Ok(&y[..self.command_len]),
            Err(_) => Err("Command is not utf8 char")
        }

    }

    pub fn keypress(&mut self, ascii: u8) {
        match ascii {
            b'\n' if self.command_len == 0 => {
                self.write_byte(b'\n');
                self.prompt();
            }
            b'\n' => {
                self.write_byte(b'\n');
                if let Err(msg) = console::exec(&self) {
                    set_color!(Red, Black);
                    println!("Something wrong: {}", msg);
                    set_color!();
                }
                self.command_len = 0;
                self.prompt();
            }
            _ if self.command_len >= 10 => (),
            byte if self.command_len == 0  && byte == b' ' => (),
            byte => {
                if self.command_len >= 10 { return };
                self.command[self.command_len] = byte;
                self.write_byte(byte);
                self.command_len += 1;
            }
        }
        self.flush();
    }

    pub fn erase_byte(&mut self) {
        self.buffer_pos -= 2;
        let i = self.buffer_pos;
        self.buffer[i] = b' ';
        self.buffer[i + 1] = 0;
        self.flush();
        // flush!();
    }

    pub fn write_byte(&mut self, byte: u8) {
        let i = self.buffer_pos;

        match byte {

            b'\n' => {
                let current_line = self.buffer_pos / (BUFFER_COLS);
                self.buffer_pos = (current_line + 1) * BUFFER_COLS;
            }
            byte => {
                self.buffer[i] = byte;
                self.buffer[i + 1] = self.color_code.0;
                self.buffer_pos += 2;
            }
        }

        if self.buffer_pos >= self.buffer.len() {
            self.scroll();
        }
    }

    fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte)
        }
    }

    fn flush_cursor(&self) {
        let cursor_position = self.buffer_pos / 2;
        // 14 awaits the rightmost 8bits
        cpuio::outb(0x3D4, 14);
        cpuio::outb(0x3D5, (cursor_position >> 8) as u8);
        // 15 awaits the leftmost 8bits
        cpuio::outb(0x3D4, 15);
        cpuio::outb(0x3D5, (cursor_position >> 0) as u8 & 0x00ff);
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

        for col in 0..BUFFER_COLS/2 {
            self.buffer[((BUFFER_ROWS - 1) * BUFFER_COLS) + (col * 2)] = b' ';
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
