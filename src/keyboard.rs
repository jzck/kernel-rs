// pub static KEY_CODE_TO_ASCII: [u8; 59] = *b"\0\01234567890-=\0\0qwertyuiop[]\n\0asdfghjkl;'`\0\\zxcvbnm,./\0*\0 \0";

const MAX_KEYS: usize = 59;
const TOUCH_RELEASE: u8 = 1 << 7;
// static ASCII_GUARD:[u8;2] = *b"\0\0";

static mut SHIFT: bool = false;
const KEYMAP_US: [[u8;2]; MAX_KEYS] = [
    *b"\0\0",
    *b"\0\0",//escape
    *b"1!",
    *b"2@",
    *b"3#",
    *b"4$",
    *b"5%",
    *b"6^",
    *b"7&",
    *b"8*",
    *b"9(",
    *b"0)",
    *b"-_",
    *b"=+",
    *b"\0\0",//backspace
    *b"\0\0",//tab
    *b"qQ",
    *b"wW",
    *b"eE",
    *b"rR",
    *b"tT",
    *b"yY",
    *b"uU",
    *b"iI",
    *b"oO",
    *b"pP",
    *b"[{",
    *b"]}",
    *b"\n\n",
    *b"\0\0",//left_control
    *b"aA",
    *b"sS",
    *b"dD",
    *b"fF",
    *b"gG",
    *b"hH",
    *b"jJ",
    *b"kK",
    *b"lL",
    *b";:",
    *b"'\"",
    *b"`~",
    *b"\0\0",//left shift
    *b"\\|",
    *b"zZ",
    *b"xX",
    *b"cC",
    *b"vV",
    *b"bB",
    *b"nN",
    *b"mM",
    *b",<",
    *b".>",
    *b"/?",
    *b"\0\0",//right shift
    *b"**",
    *b"\0\0",//left alt
    *b"  ",
    *b"\0\0",//capslock
    ];
// use vga_buffer;
use cpuio;

// use vga_buffer::buffer::{Writer, Screen};
use vga_buffer::WRITER;

fn check_touch_state(key: u8) -> (bool, usize) {
    if (key & TOUCH_RELEASE) == TOUCH_RELEASE {
        (true, (key - TOUCH_RELEASE) as usize)
    } else {
        (false, key as usize)
    }
}

pub fn kbd_loop() {
    // let terminal_one: vga::terminal::Terminal = vga::Screen::new();
    // let terminal_two: vga::terminal::Terminal = vga::Screen::new();

    let control = unsafe { cpuio::inb(0x64) };
    if (control & 1) == 1 {
        let scancode = unsafe { cpuio::inb(0x60) };
        let (is_release, scancode) = check_touch_state(scancode);
        if scancode < MAX_KEYS {
            let key_array = KEYMAP_US[scancode];
            if key_array  == *b"\0\0"{
                match scancode {
                    0x2A | 0x36 => {
                        unsafe {SHIFT = !is_release};
                    }
                    _ => {}
                }
            }
            else if !is_release {
                unsafe {
                    if SHIFT {
                        WRITER.lock().write_byte(key_array[1]);
                    }
                    else {
                        WRITER.lock().write_byte(key_array[0]);
                    }
                }
            }
        }
    }
    //match self::KEYMAP_US.get(scancode as usize) {
    //    Some(ASCII_GUARD) => {},
    //    Some(touch_array) if !is_release => {
    //        //TODO screen switching logic
    //        // if let Some(action) = Screen::keypress(current_screen, *ascii as char) {
    //        if let Some(action) = current_screen.keypress(*ascii as char) {
    //            writer.action(action);
    //        }
    //    },
    //    _ =>{},
    // None => println!("nokey ctrl {:x}", control),
    // }
    // if scancode < MAX_KEYS {
    //     let ascii = KEYMAP_US[scancode];
    //     if ascii != *b"\0\0" {
    //         if shift {
    //             writer.write(ascii[1] as char);
    //         }
    //         else {
    //             writer.write(ascii[0] as char);
    //         }

    //     }
    //     else {
    //         if scancode == 0x2A || scancode == 0x36

    //     }

    // }

    //TODO current_screen() with mutex lock?
    // let current_screen = terminal_one;

    //TODO implement logic to translate scancode->ascii
    //match self::SCANCODE_TO_ASCII.get(scancode as usize) {
    //    Some(ascii) => {
    //        //TODO screen switching logic
    //        // if let Some(action) = Screen::keypress(current_screen, *ascii as char) {
    //        if let Some(action) = current_screen.keypress(*ascii as char) {
    //            writer.action(action);
    //        }
    //    },
    //    None =>{},
    //    // None => println!("nokey ctrl {:x}", control),
    //}
}
