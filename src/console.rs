extern crate core;
// extern crate multiboot2;

use acpi;
use cpuio;
use x86;
use core::char;
use vga::*;

fn dispatch(command: &str) -> Result <(), &'static str> {
    match command {
        "help" | "h"                => self::help(),

        // multiboot
        // "memory"                    => self::mb2_memory(),
        // "multiboot"                 => self::mb2_info(),
        // "sections"                  => self::mb2_sections(),

        // ACPI
        "acpi"                      => self::acpi_info(),
        "reboot"                    => self::reboot(),
        "shutdown" | "halt" | "q"   => self::shutdown(),

        // others
        "stack"                     => self::print_stack(),
        "regs"                      => self::regs(),

        _                           => Err("Command unknown. (h|help for help)"),
    }
}

pub fn exec(cli: &Writer) -> Result <(), &'static str> {
    let command = cli.get_command()?;
    if let Err(msg) = self::dispatch(command) {
        set_color!(Red);
        println!("`{}`: {}", command, msg);
        set_color!();
    }
    Ok(())
}

fn help() -> Result <(), &'static str> {
    print!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
    "acpi                         => Return acpi state (ENABLED|DISABLE)",
    "help | h                     => Print this help",
    "memory                       => print memory areas", // TODO
    "multiboot                    => print multiboot information", // TODO
    "reboot                       => reboot",
    "sections                     => print elf sections", // TODO
    "shutdown | halt | q          => Kill a kitten, then shutdown",
    "stack                        => Print kernel stack in a fancy way");
    Ok(())
}

/// Reboot the kernel
///
/// If reboot failed, will loop on a halt cmd
///
fn reboot() -> ! {
    unsafe {asm!("cli")}; //TODO volatile ?????
    // I will now clear the keyboard buffer
    let mut buffer: u8 = 0x02;
    while buffer & 0x02 != 0 {
        cpuio::inb(0x60);
        buffer = cpuio::inb(0x64);
    }
    cpuio::outb(0x64, 0xFE);//Send reset value to CPU //TODO doesn't work in QEMU ==> it seems that qemu cannot reboot
    println!("Unable to perform reboot. Kernel will be halted");
    cpuio::halt();
}

/// Shutdown the kernel
///
/// If shutdown is performed but failed, will loop on a halt cmd
/// If shutdown cannot be called, return a Err(&str)
///
fn shutdown() -> Result <(), &'static str> {
    acpi::shutdown()?;
    println!("Unable to perform ACPI shutdown. Kernel will be halted");
    cpuio::halt();
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
    let length : usize = 16 - line.len();
    for _ in 0..length {
        print!("   ");
    }
    print!("|");
    for byte in line {
        match is_control(*byte as char) {
            true  => print!("."),
            false => print!("{}", *byte as char),
        };
    }
    print!("|");
}

/// Print the kernel stack
///
fn print_stack() -> Result <(), &'static str> {
    let esp: usize;
    let ebp: usize;
    unsafe { asm!("" : "={esp}"(esp), "={ebp}"(ebp):::) };
    println!("esp = {:#x}", esp);
    println!("ebp = {:#x}", ebp);
    println!("size = {:#X} bytes", ebp - esp);
    hexdump(esp, ebp);
    Ok(())
}

// fn mb2_memory() -> Result <(), &'static str> {
//     let boot_info = context::boot_info();

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
//     let boot_info = context::boot_info();

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

pub fn acpi_info() -> Result <(), &'static str> {
    acpi::info()?;
    Ok(())
}

pub fn regs() -> Result <(), &'static str> {
    println!("cr0={:#b}", x86::cr0());
    println!("cr3={:#x}", x86::cr3());
    println!("cr4={:#b}", x86::cr4());
    Ok(())
}

