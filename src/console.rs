use cpuio;

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

/// Print the kernel stack
///
pub fn print_kernel_stack() {
    println!("It's a stack print");

}

