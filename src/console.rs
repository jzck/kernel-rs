use cpuio;
use acpi;

/// Reboot the kernel
///
/// If reboot failed, will loop on a halt cmd
///
pub fn reboot() -> ! {
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
pub fn shutdown() -> Result <(), &'static str> {
    println!("RECV SHUTDOWN");
    acpi::shutdown()?;
    println!("Unable to perform ACPI shutdown. Kernel will be halted");
    cpuio::halt();
}

/// Print the kernel stack
///
pub fn print_kernel_stack() -> Result <(), &'static str> {
    println!("It's a stack print");
    Ok(())

}

pub fn acpi_info() -> Result <(), &'static str> {
    acpi::info()?;
    Ok(())
}
