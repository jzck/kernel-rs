#![feature(lang_items)]
#![feature(const_fn)]
#![feature(ptr_internals)]
#![feature(asm)]
#![no_std]

extern crate spin;
extern crate volatile;
extern crate rlibc;

#[macro_use]
mod vga_buffer;

#[allow(dead_code)]
mod cpuio;
mod keyboard;

#[no_mangle]
pub extern fn kmain() -> ! {
    use vga_buffer::WRITER;
    use vga_buffer::Color;
    use vga_buffer::ColorCode;

    WRITER.lock().reset_screen();
    WRITER.lock().color_code = ColorCode::new(Color::Yellow, Color::Black);
    println!(r#"        ,--,               "#);
    println!(r#"      ,--.'|      ,----,   "#);
    println!(r#"   ,--,  | :    .'   .' \  "#);
    println!(r#",---.'|  : '  ,----,'    | "#);
    println!(r#";   : |  | ;  |    :  .  ; "#);
    println!(r#"|   | : _' |  ;    |.'  /  "#);
    println!(r#":   : |.'  |  `----'/  ;   "#);
    println!(r#"|   ' '  ; :    /  ;  /    "#);
    println!(r#"\   \  .'. |   ;  /  /-,   "#);
    println!(r#" `---`:  | '  /  /  /.`|   "#);
    println!(r#"      '  ; |./__;      :   "#);
    println!(r#"      |  : ;|   :    .'    "#);
    println!(r#"      '  ,/ ;   | .'       "#);
    println!(r#"      '--'  `---'          "#);
    WRITER.lock().color_code = ColorCode::new(Color::White, Color::Black);
    println!(">> Kernel startup...");
    loop {
        let control = unsafe { cpuio::inb(0x64) };
        if (control & 1) == 1 {
            let keycode = unsafe { cpuio::inb(0x60) };
            match keyboard::KEY_CODE_TO_ASCII.get(keycode as usize) {
                Some(ascii) => print!("{}", *ascii as char),
                None =>{},
                // None => println!("nokey ctrl {:x}", control),
            }
        }
    }
}

#[lang = "eh_personality"] #[no_mangle]
pub extern fn eh_personality() {

}

#[lang = "panic_fmt"] #[no_mangle]
pub extern fn panic_fmt(
    fmt: core::fmt::Arguments,
    file: &'static str,
    line: u32)
-> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);
    loop {}

}

