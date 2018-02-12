#![feature(lang_items)]
#![feature(const_fn)]
#![feature(ptr_internals)]
#![feature(asm)]
#![no_std]

extern crate spin;
extern crate volatile;
extern crate rlibc;

#[macro_use]
mod vga;

#[allow(dead_code)]
#[macro_use]
mod context;
mod keyboard;

use context::CONTEXT;
use vga::{Color, ColorCode};

#[allow(dead_code)]
mod cpuio;

// fn check_shift(key: u8) -> u8 {
//     print!("{:b} vs {:b}\n", key as u8, (1<<7) as u8);
//     if (key >> 7  & 1) == 1 {
//         print!("MATCH");
//         key - (1 << 7)
//     } else {
//         key
//     }
// }
#[no_mangle]
pub extern fn kmain() -> ! {
    // use vga::VgaScreen;
    // use vga::color::Color;
    // use vga::color::ColorCode;

    unsafe { CONTEXT.current_term().color_code = ColorCode::new(Color::White, Color::Cyan); }
    print!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}\n",
    format_args!("{: ^80}", r#"        ,--,               "#),
    format_args!("{: ^80}", r#"      ,--.'|      ,----,   "#),
    format_args!("{: ^80}", r#"   ,--,  | :    .'   .' \  "#),
    format_args!("{: ^80}", r#",---.'|  : '  ,----,'    | "#),
    format_args!("{: ^80}", r#";   : |  | ;  |    :  .  ; "#),
    format_args!("{: ^80}", r#"|   | : _' |  ;    |.'  /  "#),
    format_args!("{: ^80}", r#":   : |.'  |  `----'/  ;   "#),
    format_args!("{: ^80}", r#"|   ' '  ; :    /  ;  /    "#),
    format_args!("{: ^80}", r#"\   \  .'. |   ;  /  /-,   "#),
    format_args!("{: ^80}", r#" `---`:  | '  /  /  /.`|   "#),
    format_args!("{: ^80}", r#"      '  ; |./__;      :   "#),
    format_args!("{: ^80}", r#"      |  : ;|   :    .'    "#),
    format_args!("{: ^80}", r#"      '  ,/ ;   | .'       "#),
    format_args!("{: ^80}", r#"      '--'  `---'          "#));
    unsafe { CONTEXT.current_term().color_code = ColorCode::new(Color::White, Color::Black); }
    print!(">");

    loop {
        keyboard::kbd_callback();
    }
}

#[lang = "eh_personality"] #[no_mangle]
pub extern fn eh_personality() {

}

#[lang = "panic_fmt"] #[no_mangle]
pub extern fn panic_fmt(
    // fmt: core::fmt::Arguments, file: &'static str, line: u32
    )
-> ! {
    // println!("PANIC: {}", fmt);
    // println!("FILE: {}", file);
    // println!("LINE: {}", line);
    loop {}

}

