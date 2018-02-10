//! terminal.rs implements a basic cursor terminal using the vga::buffer module

use super::{Color, ColorCode};
use vga;

// cursor is lightgray for everyone
static CURSOR: ColorCode = ColorCode::new(Color::LightGray, Color::White);

pub struct Terminal { }

impl vga::Screen for Terminal {
    fn new() -> Terminal {
        Terminal { }
    }

    fn keypress(self, ascii: char) -> Option<vga::BufferAction> {
        Some(vga::BufferAction::WRITE_BYTE(ascii))
    }
}
