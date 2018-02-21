//! Rust wrappers around the x86-family I/O instructions.
//! this cmodule uses inline assembly so you need to add
//! `#![feature(asm)]` to yourcrate attributes

#![allow(dead_code)]

/// Read a `u8`-sized value from `port`.
pub fn inb(port: u16) -> u8 {
    // The registers for the `in` and `out` instructions are always the
    // same: `a` for value, and `d` for the port address.
    let result: u8;
    unsafe {asm!("inb %dx, %al" : "={al}"(result) : "{dx}"(port) :: "volatile")};
    result
}

/// Write a `u8`-sized `value` to `port`.
pub fn outb(value: u8, port: u16) {
    unsafe {asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(value) :: "volatile")};
}

/// Read a `u16`-sized value from `port`.
pub fn inw(port: u16) -> u16 {
    let result: u16;
    unsafe {asm!("inw %dx, %ax" : "={ax}"(result) : "{dx}"(port) :: "volatile")};
    result
}

/// Write a `u8`-sized `value` to `port`.
pub fn outw(value: u16, port: u16) {
    unsafe {asm!("outw %ax, %dx" :: "{dx}"(port), "{ax}"(value) :: "volatile")};
}

/// Read a `u32`-sized value from `port`.
pub fn inl(port: u16) -> u32 {
    let result: u32;
    unsafe {asm!("inl %dx, %eax" : "={eax}"(result) : "{dx}"(port) :: "volatile")};
    result
}

/// Write a `u32`-sized `value` to `port`.
pub fn outl(value: u32, port: u16) {
    unsafe {asm!("outl %eax, %dx" :: "{dx}"(port), "{eax}"(value) :: "volatile")};
}

pub fn halt() -> ! {
    loop {
        unsafe {asm!("hlt")}; //TODO volatile ?????
    }
}
