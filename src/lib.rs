#![no_std]

#![feature(lang_items)]
#![feature(const_fn)]
#![feature(ptr_internals)]
#![feature(asm)]                //needed by cpuio for inline asm

// extern crate spin;
// extern crate volatile;
extern crate rlibc;

#[macro_use]
/// 80x25 screen and simplistic terminal driver
pub mod vga;
/// kernel init and environment
pub mod context;
/// PS/2 detection and processing
pub mod keyboard;
/// simplisitc kernel commands
pub mod console;
/// wrappers around the x86-family I/O instructions.
pub mod cpuio;

use context::CONTEXT;
use vga::{Color, ColorCode};

#[no_mangle]
pub extern fn kmain() -> ! {
    unsafe { CONTEXT.current_term().color_code = ColorCode::new(Color::White, Color::Cyan); }
    print!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
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
    unsafe { CONTEXT.vga1.prompt();CONTEXT.vga1.flush(); }
    unsafe { CONTEXT.vga2.prompt(); }

    loop {
        keyboard::kbd_callback();
    }
}

#[lang = "eh_personality"] #[no_mangle]
pub extern fn eh_personality() {

}

#[lang = "panic_fmt"] #[no_mangle]
pub extern fn panic_fmt(
    fmt: core::fmt::Arguments, file: &'static str, line: u32
    )
-> ! {
    println!("PANIC: {}", fmt);
    println!("FILE: {}", file);
    println!("LINE: {}", line);
    loop {}

}

