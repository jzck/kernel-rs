// https://wiki.osdev.org/PCI

use x86::devices::io::{Pio};

pub static mut PCI_CONFIG_ADDRESS: Pio<u8> = Pio::new(0xCF8);
pub static mut PCI_CONFIG_DATA: Pio<u8> = Pio::new(0xCFC);

pub fn lspci() {
    pci_config_read_word(1, 1, 1, 2);
}

pub fn pci_config_read_word(bus: u32, slot: u32, func: u32, offset: u32) {
    // let address: u64 = (bus as u32) << 16
    //                     | (slot as u32) << 11
    //                     | (func as u32) << 8
    //                     | (offset as u32) & 0xfc
    //                     | 0x80000000;
    let address: u64 = 0x80000000;
    println!("{} {} {} {}", bus, slot, func, offset);
    println!("{:#x}", address);
    println!("{:#b}", address);
}
