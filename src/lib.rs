#![feature(lang_items)]
#![feature(const_fn)]
#![feature(ptr_internals)]
#![no_std]

extern crate spin;
extern crate volatile;
extern crate rlibc;

#[macro_use]
mod vga_buffer;

#[no_mangle]
pub extern fn kmain() -> ! {
    use vga_buffer::WRITER;
    use vga_buffer::Color;
    use vga_buffer::ColorCode;

    WRITER.lock().reset_screen();
    println!(">> Kernel startup...");
    println!(">> Kernel startup...");

    WRITER.lock().color_code = ColorCode::new(Color::Blue, Color::Yellow);
    println!(">> Kernel startup...");
    println!(">> Kernel startup...");
    println!(">> Kernel startup...");

    WRITER.lock().color_code = ColorCode::new(Color::Red, Color::Green);
    println!(">> Kernel startup...");
    println!(">> Kernel startup...");
    println!(">> Kernel startup...");

    WRITER.lock().color_code = ColorCode::new(Color::White, Color::Black);

    loop {
        
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

