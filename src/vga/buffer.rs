// Copyright 2016 Philipp Oppermann. See the README.md
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use volatile::Volatile;

use super::{Color, ColorCode};
use core::ptr::Unique;
use core::fmt;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

pub struct Writer {
    pub column_position: usize,
    pub color_code: ColorCode,
    vgabuffer: Unique<Buffer>,
}

// Writer is the default object to use to print to screen
pub static mut WRITER: Writer = Writer {
    column_position: 0,
    color_code: ColorCode::new(Color::White, Color::Black),
    vgabuffer: unsafe { Unique::new_unchecked(0xb8000 as *mut _) },
};

// blank is black for everyone, could make screens choose this
static BLANK: ScreenChar = ScreenChar {
    ascii_character: b' ',
    color_code: ColorCode::new(Color::White, Color::Black),
};

impl Writer {
    pub fn action(&mut self, action: BufferAction) {
        match action {
            BufferAction::WRITE_BYTE(ascii) => self.write_byte(ascii as u8),
            BufferAction::CLEAR_SCREEN => self.reset_screen(),
        }
    }

    pub fn reset_screen(&mut self)
    {
        let color_code = self.color_code;
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                self.buffer().chars[row][col].write(ScreenChar {
                    ascii_character: b' ',
                    color_code,
                });
            }
        }
        self.column_position = 0;
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => { self.new_line(); }
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                let color_code = self.color_code;
                self.buffer().chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code: color_code,
                });
                self.column_position += 1;
            }
        }
    }

    fn buffer(&mut self) -> &mut Buffer {
        unsafe { self.vgabuffer.as_mut() }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let buffer = self.buffer();
                let character = buffer.chars[row][col].read();
                buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        for col in 0..BUFFER_WIDTH {
            self.buffer().chars[row][col].write(BLANK);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte)
        }
        Ok(())
    }
}

struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub enum BufferAction {
    WRITE_BYTE(char),
    CLEAR_SCREEN,
}

/// Implementors of this trait can use the vga screen for any purpose
pub trait Screen {
    fn new() -> Self; //
    fn keypress(self, keycode: char) -> Option<BufferAction>;
    // fn load();
    // fn unload() -> ;
}
