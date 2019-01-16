extern crate core;
// extern crate multiboot2;

use acpi;
use time;
use keyboard::PS2;
use core::char;
use vga::*;

pub static mut CONSOLE: Console = self::Console::new();

pub struct Console {
    command: [u8; 10],
    command_len: usize,
}

impl Console {
    pub const fn new() -> Console {
        Console {
            command: [b'\0'; 10],
            command_len: 0,
        }
    }

    pub fn init(&self) {
        set_color!();
        // print!("{}", format_args!("{: ^4000}", r#" "#));
        unsafe {
            // VGA.buffer_pos = 0;
            self.prompt();
            VGA.flush();
        }
    }

    pub fn backspace(&mut self) {
        if self.command_len > 0 {
            self.command_len -= 1;
            unsafe { VGA.erase_byte(); }
        }
    }

    pub fn prompt(&self) {
        set_color!(Blue);
        unsafe { VGA.write_str("> "); }
        set_color!();
        flush!();
    }

    pub fn keypress(&mut self, ascii: u8) {
        match ascii {
            b'\n' if self.command_len == 0 => {
                unsafe { VGA.write_byte(b'\n'); }
                self.prompt();
            }
            b'\n' => {
                unsafe { VGA.write_byte(b'\n'); }
                self.exec();
                self.command_len = 0;
                self.prompt();
            }
            // _ if self.command_len >= 10 => (),
            // byte if self.command_len == 0 && byte == b' ' => (),
            byte => {
                if self.command_len >= 10 {
                    return;
                };
                self.command[self.command_len] = byte;
                unsafe { VGA.write_byte(byte); }
                self.command_len += 1;
            }
        }
        flush!();
    }


    fn get_command(&self) -> Result<&str, &'static str> {
        match core::str::from_utf8(&self.command) {
            Ok(y) => Ok(&y[..self.command_len]),
            Err(_) => Err("Command is not utf8"),
        }
    }

    pub fn exec(&self) -> core::result::Result<(), &'static str> {
        let command = self.get_command();
        if let Err(msg) = command {
            set_color!(Red);
            println!("{}", msg);
            set_color!();
        }
        match command.unwrap() {
            "help" | "h" => self::help(),

            // multiboot
            // "memory"                    => self::mb2_memory(),
            // "multiboot"                 => self::mb2_info(),
            // "sections"                  => self::mb2_sections(),

            // ACPI
            "acpi" => self::acpi_info(),
            "reboot" => self::reboot(),
            "shutdown" | "halt" | "q" => self::shutdown(),

            // x86 specific
            "stack" => self::print_stack(),
            "regs" => self::regs(),
            "cpu" => self::cpu(),
            "int3" => self::int3(),
            "overflow" => self::overflow(),
            "page_fault" => self::page_fault(),

            // time
            "uptime" => self::uptime(),

            _ => Err("Command unknown. (try help)"),
        }
    }
}

fn help() -> Result<(), &'static str> {
    println!("help | h                     => print this help");
    // println!("memory                       => Print memory areas");
    // println!("multiboot                    => Print multiboot information");
    // println!("sections                     => Print elf sections");
    println!("reboot                       => reboot");
    println!("shutdown | halt | q          => acpi shutdown");
    println!("acpi                         => acpi state");
    println!("stack                        => print kernel stack in a fancy way");
    println!("regs                         => print controle register");
    println!("cpu                          => print cpu information");
    println!("overflow                     => triggers a stack overflow");
    println!("page_fault                   => triggers a page fault on 0xdead");
    flush!();
    Ok(())
}

fn uptime() -> Result<(), &'static str> {
    let mut offset = time::OFFSET.lock();
    fprintln!("{}s", offset.0 + offset.1 / 1_000_000);
    flush!();
    Ok(())
}

use x86::instructions::halt;
/// Reboot the kernel
///
/// If reboot failed, will loop on a halt cmd
///
fn reboot() -> ! {
    match acpi::reboot() {
        Err(msg) => println!("{}", msg),
        _ => println!("Unable to perform ACPI reboot."),
    }
    flush!();
    unsafe { PS2.ps2_8042_reset() }; // TODO unsafe
    println!("Unable to perform 8042 reboot. Kernel will be halted");
    flush!();
    halt();
}

/// Shutdown the kernel
///
/// If shutdown is performed but failed, will loop on a halt cmd
/// If shutdown cannot be called, return a Err(&str)
///
fn shutdown() -> ! {
    match acpi::shutdown() {
        Err(msg) => println!("{}", msg),
        _ => println!("Unable to perform ACPI shutdown. Kernel will be halted"),
    }
    flush!();
    halt();
}

fn hexdump(start: usize, end: usize) {
    let mut address = 0;
    let data = unsafe { core::slice::from_raw_parts_mut(start as *mut u8, end - start) };
    while address <= data.len() {
        let next_end = core::cmp::min(address + 16, data.len());
        print_line(&data[address..next_end], address + start);
        address = address + 16;
    }
    println!("");
}

fn is_control(c: char) -> bool {
    !(c >= ' ' && c <= '~')
}

fn print_line(line: &[u8], address: usize) {
    print!("\n{:#08x}: ", address);
    for byte in line {
        print!("{:02x} ", *byte);
    }
    let length: usize = 16 - line.len();
    for _ in 0..length {
        print!("   ");
    }
    print!("|");
    for byte in line {
        match is_control(*byte as char) {
            true => print!("."),
            false => print!("{}", *byte as char),
        };
    }
    print!("|");
}

/// Print the kernel stack
pub fn print_stack() -> Result<(), &'static str> {
    let esp: usize;
    let ebp: usize;
    unsafe { asm!("" : "={esp}"(esp), "={ebp}"(ebp):::) };
    println!("esp = {:#x}", esp);
    println!("ebp = {:#x}", ebp);
    println!("size = {:#X} bytes", ebp - esp);
    hexdump(esp, ebp);
    flush!();
    Ok(())
}

// fn mb2_memory() -> Result <(), &'static str> {
//     let boot_info = ::multiboot2::boot_info();

//     let memory_map_tag = boot_info.memory_map_tag()
//         .expect("Memory map tag required");

//     println!("memory areas:");
//     for area in memory_map_tag.memory_areas() {
//         println!("    start: 0x{:x}, length: 0x{:x}",
//                  area.start_address(), area.size());
//     }
//     Ok(())
// }

// fn mb2_sections() -> Result <(), &'static str> {
//     let boot_info = ::multiboot2::boot_info();

//     let elf_sections_tag = boot_info.elf_sections_tag()
//         .expect("Elf-sections tag required");

//     println!("kernel sections:");
//     for section in elf_sections_tag.sections() {
//         println!("    {: <10} {:#x}, size: {:#x}, flags: {:#X}",
//                  section.name(), section.start_address(), section.size(), section.flags());
//     }
//     Ok(())
// }

// fn mb2_info() -> Result <(), &'static str> {
//     let boot_info = context::boot_info();

//     let command_line_tag = boot_info.command_line_tag()
//         .expect("Elf-sections tag required");

//     let bootloader_tag = boot_info.boot_loader_name_tag()
//         .expect("Elf-sections tag required");

//     println!("bootloader: {}", bootloader_tag.name());
//     if command_line_tag.command_line().len() != 0 {
//         println!("command line: {}", command_line_tag.command_line());
//     }
//     Ok(())
// }

pub fn acpi_info() -> Result<(), &'static str> {
    acpi::info()?;
    Ok(())
}

/// Dump control registers
pub fn regs() -> Result<(), &'static str> {
    use x86::registers::control::*;
    use x86::instructions::tables::tr;
    use x86::instructions::segmentation::*;
    use x86::registers::flags::*;
    use x86::structures::gdt;
    println!("cr0  = {:?}", Cr0::read());
    println!("cr3  = {:?}", Cr3::read());
    println!("cr4  = {:?}", Cr4::read());
    println!("flags= {:?}", flags());
    println!("tr   = {:?}", tr());
    println!("ss   = {:?}", ss());
    println!("cs   = {:?}", cs());
    println!("ds   = {:?}", ds());
    println!("es   = {:?}", es());
    println!("fs   = {:?}", fs());
    println!("gs   = {:?}", gs());
    unsafe {
        println!(
            "tss = {:#?}",
            gdt::Descriptor(::arch::x86::gdt::GDT.table[tr().index() as usize])
        );
    }
    flush!();
    Ok(())
}

/// Dump cpu info, should add power management info
pub fn cpu() -> Result<(), &'static str> {
    use arch::x86::devices::cpu;
    cpu::cpu_info().expect("cpu info not available");
    flush!();
    Ok(())
}

pub fn int3() -> Result<(), &'static str> {
    use x86;
    x86::instructions::interrupts::int3();
    Ok(())
}

#[allow(unconditional_recursion)]
pub fn overflow() -> Result<(), &'static str> {
    fn stack_overflow() {
        stack_overflow();
    }
    stack_overflow();
    Ok(())
}

pub fn page_fault() -> Result<(), &'static str> {
    unsafe {
        *(0xdead as *mut u32) = 42;
    };
    Ok(())
}
