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

#[allow(dead_code)]
mod cpuio;


//TODO implement ACPI to have such functionality 
/// Reboot the kernel
///
/// If reboot failed, will loop on a halt cmd
///
#[allow(dead_code)]
fn reboot()  {
    //TODO disable interrupt here something like : asm volatile ("cli");

    // I will now clear the keyboard buffer
    let mut buffer: u8 = 0x02;
    while buffer == 0x02 {
        buffer = cpuio::inb(0x64);
    }
    cpuio::outb(0x64, 0xFE);//Send reset value to CPU //TODO doesn't work
    println!("Reicv reboot command. System cannot reboot yet, he is now halt\n");
    cpuio::halt();
}

/// Shutdown the kernel
///
/// # Pre-requist:
/// Seems that he have to use following line command :
/// `-device isa-debug-exit,iobase=0xf4,iosize=0x04`
///
/// If shutdown failed, will loop on a halt cmd
///
#[allow(dead_code)]
fn shutdown() -> ! {
    cpuio::outb(0xf4, 0x00);//TODO doesn't work :(
    println!("Reicv shutdown command. System cannot shutdown properly yet, he is now halt\n");
    cpuio::halt();
}

#[no_mangle]
pub extern fn kmain() -> ! {
    // use vga::VgaScreen;
    // use vga::color::Color;
    // use vga::color::ColorCode;

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
    println!(">> Kernel startup...");

    loop {
        keyboard::kbd_callback();
    }
    // let control = unsafe { cpuio::inb(0x64) };
    // if (control & 1) == 1 {
    //     let keycode = unsafe { cpuio::inb(0x60) };
    //     match keyboard::KEY_CODE_TO_ASCII.get(keycode as usize) {
    //         Some(ascii) => print!("{}", *ascii as char),
    //         None =>{},
    //         // None => println!("nokey ctrl {:x}", control),
    //     }
    // }
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

