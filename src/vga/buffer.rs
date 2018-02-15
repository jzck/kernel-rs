// Copyright 2016 Philipp Oppermann. See the README.md
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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
    pub buffer_pos: usize,
    pub color_code: ColorCode,
    buffer: [u8; BUFFER_ROWS * BUFFER_COLS],
    command: [u8; 10],
    command_len: usize,
}

impl Writer {
    pub const fn new() -> Writer {
        Writer {
            buffer_pos: 0,
            color_code: ColorCode::new(Color::White, Color::Black),
            buffer: [0; BUFFER_ROWS * BUFFER_COLS],
            command: [0; 10],
            command_len: 0,
        }
    }

    pub fn keypress(&mut self, ascii: u8) {
        match ascii {
            b'\n' => {
                self.command_len = 0;
                self.write_byte(b'\n');
                println!("{:?}", self.command.iter());
                self.write_byte(b'>');
            }
            byte => {
                if self.command_len >= 10 { return };

                self.write_byte(byte);
                self.command_len += 1;
            }
        }
        self.flush();
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

    fn flush_cursor(&self)
    {
        let cursor_position = self.buffer_pos / 2;
        // 14 awaits the rightmost 8bits
        cpuio::outb(14, 0x3D4);
        cpuio::outb((cursor_position >> 8) as u8, 0x3D5);
        // 15 awaits the leftmost 8bits
        cpuio::outb(15, 0x3D4);
        cpuio::outb((cursor_position >> 0) as u8 & 0x00ff, 0x3D5);
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

use core::fmt;
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte)
        }
        Ok(())
    }
}
