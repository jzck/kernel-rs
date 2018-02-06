#![feature(lang_items)]
#![feature(const_fn)]
#![feature(ptr_internals)]
#![feature(asm)]
#![no_std]

extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;
#[macro_use] extern crate bitflags;

#[macro_use] mod vga_buffer;


// mod memory;

#[no_mangle]
pub extern fn rust_main(multiboot_information_address: usize) {
    vga_buffer::clear_screen();

    use core::ptr::{read_volatile, write_volatile};
    let kbd1 = &0x60 as *const i32;
    let kbd2 = &0x64 as *const i32;

    loop{
        unsafe {
            if (read_volatile(kbd1) != 96) {
                println!("0x60: {} !!!!!!", read_volatile(kbd1));
                break;
            }
            println!("0x60: {} 0x64: {}",
                     read_volatile(kbd1), read_volatile(kbd2) );
        }
    };
}

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str,
                        line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);
    loop{}
}
