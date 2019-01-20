// https://wiki.osdev.org/PCI

use x86::devices::io::{Pio, Io};

pub static mut PCI_CONFIG_ADDRESS: Pio<u32> = Pio::new(0xCF8);
pub static mut PCI_CONFIG_DATA: Pio<u32> = Pio::new(0xCFC);

pub fn get_ide1() -> Result<u32, ()> {
    let bus = 0;

    for slot in 0..31 {
        let vendor = pci_config_read_word(bus, slot, 0, 0);
        if vendor == 0xffff {
            continue;
        }
        let class = pci_config_read_byte(bus, slot, 0, 11);
        let subclass = pci_config_read_byte(bus, slot, 0, 10);
        if class == 0x01 && subclass == 0x01 {
            return Ok(slot);
        }
    }
    Err(())
}

pub fn lspci() {
    let bus = 0;

    for slot in 0..31 {
        let vendor = pci_config_read_word(bus, slot, 0, 0);
        if vendor == 0xffff {
            continue;
        }
        let header_type = pci_config_read_byte(bus, slot, 0, 14) ;
        if header_type & 0x80 != 0 {
            // device has multiple functions
            for function in 0..0x7 {
                let vendor = pci_config_read_word(bus, slot, function, 0);
                if vendor != 0xffff {
                    pci_display(bus, slot, function);
                }
            }
        } else {
            pci_display(bus, slot, 0);
        }
    }
}

pub fn pci_display(bus: u32, slot: u32, function: u32) {
    let vendor = pci_config_read_word(bus, slot, function, 0);
    let device = pci_config_read_word(bus, slot, function, 2);
    let class = pci_config_read_byte(bus, slot, function, 11);
    let subclass = pci_config_read_byte(bus, slot, function, 10);
    println!("{}:{}.{} {:#x},{:#x}: {:#x} {:#x}",
             bus, slot, function, class, subclass, vendor, device);
}

pub fn pci_access(bus: u32, slot: u32, func: u32, offset: u32) {
    let address = (bus as u32) << 16
        | (slot as u32) << 11
        | (func as u32) << 8
        | (offset as u32) & 0xfc
        | 0x80000000;
    unsafe { PCI_CONFIG_ADDRESS.write(address); }
}

pub fn pci_config_read_doubleword(bus: u32, slot: u32, func: u32, offset: u32) -> u32 {
    pci_access(bus, slot, func, offset);
    unsafe { PCI_CONFIG_DATA.read() }
}

pub fn pci_config_read_word(bus: u32, slot: u32, func: u32, offset: u32) -> u16 {
    pci_access(bus, slot, func, offset);
    unsafe { (PCI_CONFIG_DATA.read() >> ((offset & 2) * 8)) as u16 }
}

pub fn pci_config_read_byte(bus: u32, slot: u32, func: u32, offset: u32) -> u8 {
    pci_access(bus, slot, func, offset);
    unsafe { (PCI_CONFIG_DATA.read() >> ((offset & 3) * 8)) as u8 }
}
