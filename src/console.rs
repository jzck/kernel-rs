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

pub fn print_hexdump(data: &[u8], offset: usize, display: char, bytes: usize) {
    let mut address = 0;
    while address <= data.len() {
        let end = core::cmp::min(address + 16, data.len());
        print_line(&data[address..end], address + offset, display, bytes);
        address = address + 16;
    }
}

fn is_control(c: char) -> bool {
    !(c >= ' ' && c <= '~')
}

fn print_line(line: &[u8], address: usize, display: char, bytes: usize) {
    // print address
    print!("\n{:08x}:", address);
    let words = match (line.len() % bytes) == 0 {
        true  =>  line.len() / bytes,
        false => (line.len() / bytes) + 1,
    };
    for b in 0..words {
        let word = match bytes {
            1 => line[b] as u16,
            _ => match line.len() == bytes*b + 1 {
                     true  => u16::from_be(((line[bytes * b] as u16) << 8) + 0),
                     false => u16::from_be(((line[bytes * b] as u16) << 8) + (line[bytes * b + 1] as u16)),
                 },
        };
        match display {
            'b' => print!(" {:03o}",  word),
            'c' => match is_control((word as u8) as char) {
                       true  => print!(" "),
                       false => print!(" {:03}", (word as u8) as char),
                   },
            'C' => print!(" {:02x}",  word),
            'x' => print!(" {:04x}",  word),
            'o' => print!(" {:06o} ", word),
            'd' => print!("  {:05} ", word),
            _   => print!(" {:04x}",  word),
        }
    }

    if display != 'c' {
        if (line.len() % 16) > 0 {
            // align
            let words_left = (16 - line.len()) / bytes;
            let word_size = match display {
                'b' => 4,
                'c' => 4,
                'C' => 3,
                'x' => 5,
                'o' => 8,
                'd' => 8,
                _   => 5,
            };
            for _ in 0..word_size * words_left {
                print!(" ");
            }
        }

        print!("  ");
        for c in line {
            // replace all control chars with dots
            match is_control(*c as char) { 
                true  => print!("."),
                false => print!("{}", (*c as char)),
            }
        }
    }
}

/// Print the kernel stack
///
pub fn print_kernel_stack() {
    let esp: usize;
    let ebp: usize;
    unsafe { asm!("": "={esp}"(esp), "={ebp}"(ebp):::) };
    println!("{:#x} -> {:#x} (size={} bytes)", ebp, esp, ebp - esp);
    let slice = unsafe { core::slice::from_raw_parts_mut(ebp as *mut u8, ebp - esp) };
    print_hexdump(slice, 0, 'x', ebp - esp);
    println!("");
}
