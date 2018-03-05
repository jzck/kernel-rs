//! project hosted at (https://github.com/jzck/kernel)

#![no_std]
#![feature(lang_items)]
#![feature(const_fn)]
#![feature(ptr_internals)]
#![feature(asm)]

extern crate rlibc;
extern crate multiboot2;        //slightly modified fork from official 0.3.2
#[macro_use] extern crate bitflags;

/// 80x25 screen and simplistic terminal driver
#[macro_use] pub mod vga;
/// kernel init and environment
pub mod context;
/// PS/2 detection and processing
pub mod keyboard;
/// simplisitc kernel commands
pub mod console;
/// wrappers around the x86-family I/O instructions.
pub mod cpuio;
/// ACPI self-content module
pub mod acpi;
/// simple area frame allocator implementation
pub mod memory;

use context::*;
use memory::*;

use vga::{Color, ColorCode};

#[no_mangle]
pub extern fn kmain(multiboot_info_addr: usize) -> ! {
    unsafe { CONTEXT = Some(Context::new(multiboot_info_addr)) };
    acpi::init().unwrap();
    context().init_screen();

    set_color!(White, Cyan);
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
    set_color!();

    loop { keyboard::kbd_callback(); }
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

