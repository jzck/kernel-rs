pub static SCANCODE_TO_ASCII: [u8; 59] = *b"??1234567890-=??qwertyuiop[]\n?asdfghjkl;'`?\\zxcvbnm,./?*? ?"; 

use vga;
use cpuio;

use vga::buffer::{Writer, Screen};

pub fn kbd_loop(writer: &mut Writer) {
    let terminal_one: vga::terminal::Terminal = vga::Screen::new();
    // let terminal_two: vga::terminal::Terminal = vga::Screen::new();

    let control = unsafe { cpuio::inb(0x64) };
    if (control & 1) == 1 {
        let scancode = unsafe { cpuio::inb(0x60) };

        //TODO current_screen() with mutex lock?
        let current_screen = terminal_one;

        //TODO implement logic to translate scancode->ascii
        match self::SCANCODE_TO_ASCII.get(scancode as usize) {
            Some(ascii) => {
                //TODO screen switching logic
                // if let Some(action) = Screen::keypress(current_screen, *ascii as char) {
                if let Some(action) = current_screen.keypress(*ascii as char) {
                    writer.action(action);
                }
            },
            None =>{},
            // None => println!("nokey ctrl {:x}", control),
        }
    }
}
