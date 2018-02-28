extern crate core;
extern crate multiboot2;

use cpuio;
use core::char;
use context::CONTEXT;
use vga::*;

pub fn dispatch(command: &str) {
    match command {
        "shutdown" | "halt" => self::shutdown(),
        "reboot" => self::reboot(),
        "stack" => self::print_stack(),
        "multiboot" => self::mb2_info(),
        "memory" => self::mb2_memory(),
        "sections" => self::mb2_sections(),
        _ => {
            set_color!(Red);
            println!("`{}': Command unknown ", command);
            set_color!();
        }
    }
}

//TODO implement ACPI to have such functionality 
/// Reboot the kernel
///
/// If reboot failed, will loop on a halt cmd
///
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
fn shutdown() -> ! {
    cpuio::outb(0xf4, 0x00);//TODO doesn't work :(
    println!("Reicv shutdown command. System cannot shutdown properly yet, he is now halt\n");
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
    print!("|");
    for byte in line {
        match is_control(*byte as char) {
            true  => print!("."),
            false => print!("{}", *byte as char),
        };
    }
    print!("|");
}

fn print_stack() {
    let esp: usize;
    let ebp: usize;
    unsafe { asm!("" : "={esp}"(esp), "={ebp}"(ebp):::) };
    println!("esp = {:#x}", esp);
    println!("ebp = {:#x}", ebp);
    println!("size = {:#X} bytes", ebp - esp);
    hexdump(esp, ebp);
}

fn mb2_memory() {
    let boot_info = unsafe { multiboot2::load(CONTEXT.boot_info_addr) };

    let memory_map_tag = boot_info.memory_map_tag()
        .expect("Memory map tag required");

    println!("memory areas:");
    for area in memory_map_tag.memory_areas() {
        println!("    start: 0x{:x}, length: 0x{:x}",
                 area.start_address(), area.size());
    }
}

fn mb2_sections() {
    let boot_info = unsafe { multiboot2::load(CONTEXT.boot_info_addr) };

    let elf_sections_tag = boot_info.elf_sections_tag()
        .expect("Elf-sections tag required");

    println!("kernel sections:");
    for section in elf_sections_tag.sections() {
        println!("    {: <10} {:#x}, size: {:#x}, flags: {:#X}",
             section.name(), section.start_address(), section.size(), section.flags());
    }
}

fn mb2_info() {
    let boot_info = unsafe { multiboot2::load(CONTEXT.boot_info_addr) };

    let command_line_tag = boot_info.command_line_tag()
        .expect("Elf-sections tag required");

    let bootloader_tag = boot_info.boot_loader_name_tag()
        .expect("Elf-sections tag required");

    println!("bootloader: {}", bootloader_tag.name());
    if command_line_tag.command_line().len() != 0 {
        println!("command line: {}", command_line_tag.command_line());
    }
}
