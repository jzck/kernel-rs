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
mod cpuio;
mod keyboard;

#[no_mangle]
pub extern fn kmain() -> ! {
    // use vga::VgaScreen;
    // use vga::color::Color;
    // use vga::color::ColorCode;

    // WRITER.lock().reset_screen();
    // WRITER.lock().color_code = ColorCode::new(Color::Yellow, Color::Black);
    // println!(r#"        ,--,               "#);
    // println!(r#"      ,--.'|      ,----,   "#);
    // println!(r#"   ,--,  | :    .'   .' \  "#);
    // println!(r#",---.'|  : '  ,----,'    | "#);
    // println!(r#";   : |  | ;  |    :  .  ; "#);
    // println!(r#"|   | : _' |  ;    |.'  /  "#);
    // println!(r#":   : |.'  |  `----'/  ;   "#);
    // println!(r#"|   ' '  ; :    /  ;  /    "#);
    // println!(r#"\   \  .'. |   ;  /  /-,   "#);
    // println!(r#" `---`:  | '  /  /  /.`|   "#);
    // println!(r#"      '  ; |./__;      :   "#);
    // println!(r#"      |  : ;|   :    .'    "#);
    // println!(r#"      '  ,/ ;   | .'       "#);
    // println!(r#"      '--'  `---'          "#);
    // WRITER.lock().color_code = ColorCode::new(Color::White, Color::Black);
    // println!(">> Kernel startup...");

    unsafe { loop { keyboard::kbd_loop(&mut vga::buffer::WRITER); } };
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

