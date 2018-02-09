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
    println!(">> Kernel startup...");
    println!(">> Kernel startup...");

    WRITER.lock().color_code = ColorCode::new(Color::Blue, Color::Yellow);
    println!(">> Kernel startup...");
    println!(">> Kernel startup...");
    println!(">> Kernel startup...");

    WRITER.lock().color_code = ColorCode::new(Color::Red, Color::Green);
    println!(">> Kernel startup...");
    print!("\n");
    println!(">> Kernel startup...");
    print!("\n");
    println!(">> Kernel startup...");
    print!("0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");

    // WRITER.lock().color_code = ColorCode::new(Color::White, Color::Black);

    loop {
        let control = unsafe { cpuio::inb(0x64) };
        if (control & 1) == 1 {
            let keycode = unsafe { cpuio::inb(0x60) };
            match keyboard::KEY_CODE_TO_ASCII.get(keycode as usize) {
                Some(ascii) => {
                    print!("{}", *ascii as char);
                    // unsafe { cpuio::outb(28, 0x64) };
                },
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

