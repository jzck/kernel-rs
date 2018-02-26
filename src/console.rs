extern crate core;

use cpuio;
use core::char;

//TODO implement ACPI to have such functionality 
/// Reboot the kernel
///
/// If reboot failed, will loop on a halt cmd
///
pub fn reboot()  {
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
pub fn shutdown() -> ! {
    cpuio::outb(0xf4, 0x00);//TODO doesn't work :(
    println!("Reicv shutdown command. System cannot shutdown properly yet, he is now halt\n");
    cpuio::halt();
}

pub fn print_hexdump(data: &[u8], offset: usize) {
    let mut address = 0;
    while address <= data.len() {
        let end = core::cmp::min(address + 16, data.len());
        print_line(&data[address..end], address + offset);
        address = address + 16;
    }
}

fn is_control(c: char) -> bool {
    !(c >= ' ' && c <= '~')
}

fn print_line(line: &[u8], address: usize) {
    print!("\n{:08x}: ", address);
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

/// Print the kernel stack
pub fn print_kernel_stack() {
    let esp: usize;
    let ebp: usize;
    unsafe { asm!("" : "={esp}"(esp), "={ebp}"(ebp):::) };
    println!("{:#x} -> {:#x} (size={} bytes)", ebp, esp, ebp - esp);
    let slice = unsafe { core::slice::from_raw_parts_mut(ebp as *mut u8, ebp - esp) };
    print_hexdump(slice, ebp);
    println!("");
}
