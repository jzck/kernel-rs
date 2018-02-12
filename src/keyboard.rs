extern crate core;

use cpuio;
use context::CONTEXT;
// use vga::color::{Color, ColorCode};

pub static SCANCODE_TO_ASCII: [u8; 59] = *b"??1234567890-=??qwertyuiop[]\n?asdfghjkl;'`?\\zxcvbnm,./?*? ?"; 

pub fn kbd_callback() {
    // let terminal_two: vga::terminal::Terminal = vga::Screen::new();
    let control = unsafe { cpuio::inb(0x64) };
    if (control & 1) == 1 {
        let scancode = unsafe { cpuio::inb(0x60) };
        unsafe {
            match self::SCANCODE_TO_ASCII.get(scancode as usize) {
                Some(&b'1') => {
                    CONTEXT.switch_term();
                    CONTEXT.current_term().flush();
                }
                Some(ascii) => {
                    CONTEXT.current_term().keypress(*ascii);
                },
                None =>{},
                // None => println!("nokey ctrl {:x}", control),
            }
        }
    }
}
