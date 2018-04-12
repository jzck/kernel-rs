extern crate core;

use cpuio;
use vga;
use io::Pio;
use io::Io;

const MAX_KEYS: usize = 59;
const KEYMAP_US: [[u8; 2]; MAX_KEYS] = [
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


pub static mut PS2: Ps2 = Ps2::new();

pub struct Ps2 {
    status: Pio<u8>,
    data: Pio<u8>,
}

impl Ps2 {
    pub const fn new() -> Ps2 {
        Ps2 {
            status: Pio::new(0x64),
            data: Pio::new(0x60),
        }
    }

    pub fn clear_buffer(&self) {
        let mut buffer: u8 = 0x02;
        while buffer & 0x02 != 0 {
            self.data.read();
            buffer = self.status.read();
        }
    }

    pub fn ps2_8042_reset(&mut self) {
        cpuio::cli();
        self.clear_buffer();
        self.status.write(0xFE);
    }

    pub fn get_scancode(&self) -> u8 {
        let mut scancode = 0;
        loop {
            if self.data.read() != scancode {
                scancode = self.data.read();
                if scancode > 0 {
                    return scancode;
                }
            }
        }
    }
}

const TOUCH_RELEASE: u8 = 1 << 7;

fn check_key_state(key: u8) -> (bool, usize) {
    if (key & TOUCH_RELEASE) == TOUCH_RELEASE {
        (true, (key - TOUCH_RELEASE) as usize)
    } else {
        (false, key as usize)
    }
}

pub fn kbd_callback() {
    static mut SHIFT: bool = false;
    static mut CTRL: bool = false;
    static mut ALT: bool = false;
    let scancode = unsafe { PS2.get_scancode() };
    let (is_release, scancode) = check_key_state(scancode);
    unsafe {
        //TODO remove unsafe
        match self::KEYMAP_US.get(scancode as usize) {
            Some(b"\0\0") => match scancode {
                0x2A | 0x36 => SHIFT = !is_release,
                0x38 => ALT = !is_release,
                0x1D => CTRL = !is_release,
                0x0E if !is_release => {
                    vga::VGA.backspace();
                }
                _ => {}
            },
            Some(ascii) if !is_release => {
                let sym = if SHIFT { ascii[1] } else { ascii[0] };
                vga::VGA.keypress(sym);
            }
            _ => {}
        }
    }
}
