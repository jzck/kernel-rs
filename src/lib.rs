//! project hosted at (https://github.com/jzck/kernel)

#![no_std]
#![feature(lang_items)]
#![feature(const_fn)]
#![feature(ptr_internals)]
#![feature(asm)]                //needed by cpuio for inline asm

extern crate rlibc;
extern crate multiboot2;

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

use context::CONTEXT;

#[no_mangle]
pub extern fn kmain(multiboot_information_address: usize) -> ! {
    // use vga::{Color, ColorCode};
    // unsafe { CONTEXT.current_term().color_code = ColorCode::new(Color::White, Color::Cyan); }
    // print!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
    // format_args!("{: ^80}", r#"        ,--,               "#),
    // format_args!("{: ^80}", r#"      ,--.'|      ,----,   "#),
    // format_args!("{: ^80}", r#"   ,--,  | :    .'   .' \  "#),
    // format_args!("{: ^80}", r#",---.'|  : '  ,----,'    | "#),
    // format_args!("{: ^80}", r#";   : |  | ;  |    :  .  ; "#),
    // format_args!("{: ^80}", r#"|   | : _' |  ;    |.'  /  "#),
    // format_args!("{: ^80}", r#":   : |.'  |  `----'/  ;   "#),
    // format_args!("{: ^80}", r#"|   ' '  ; :    /  ;  /    "#),
    // format_args!("{: ^80}", r#"\   \  .'. |   ;  /  /-,   "#),
    // format_args!("{: ^80}", r#" `---`:  | '  /  /  /.`|   "#),
    // format_args!("{: ^80}", r#"      '  ; |./__;      :   "#),
    // format_args!("{: ^80}", r#"      |  : ;|   :    .'    "#),
    // format_args!("{: ^80}", r#"      '  ,/ ;   | .'       "#),
    // format_args!("{: ^80}", r#"      '--'  `---'          "#));
    // unsafe { CONTEXT.current_term().color_code = ColorCode::new(Color::White, Color::Black); }
    
    // let boot_info = unsafe{ multiboot2::load(multiboot_information_address) };
    // let memory_map_tag = boot_info.memory_map_tag()
    //     .expect("Memory map tag required");

    // println!("memory areas:");
    // for area in memory_map_tag.memory_areas() {
    //     println!("    start: 0x{:x}, length: 0x{:x}",
    //              area.base_addr, area.length);
    // }

    // let elf_sections_tag = boot_info.elf_sections_tag()
    //     .expect("Elf-sections tag required");

    // println!("kernel sections:");
    // for section in elf_sections_tag.sections() {
    //     println!("    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}",
    //              section.addr, section.size, section.flags);
    // }

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

