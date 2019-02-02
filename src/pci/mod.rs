// https://wiki.osdev.org/PCI
use x86::devices::io::{Pio, Io};
use alloc::vec::*;

pub mod ide;

pub static mut PCI_CONFIG_ADDRESS: Pio<u32> = Pio::new(0xCF8);
pub static mut PCI_CONFIG_DATA: Pio<u32> = Pio::new(0xCFC);

pub struct Pci {
    pub bus: u32,
    pub slot: u32,
    pub func: u32,
    pub device: u16,
    pub vendor: u16,
    pub class: u8,
    pub subclass: u8,
    pub header_type: u8,
}

pub fn lspci() {
    for pci in self::get_all() {
        println!("{}", pci);
    }
}

pub fn get_all() -> Vec<Pci> {
    let bus = 0;
    let mut slot = 0;
    let mut pcis: Vec<Pci> = Vec::new();

    while let Some(pci)= pci_get(bus, slot, 0) {
        if pci.header_type & 0x80 != 0 {
            let mut func = 0;
            while let Some(pci)= pci_get(bus, slot, func) {
                // device has multiple functions
                pcis.push(pci);
                func += 1;
                if func == 7 {break;}
            }
        }
        pcis.push(pci);
        slot += 1;
        if slot == 32 {break;}
    }
    pcis
}

pub fn get_first(class: u8, subclass: u8) -> Option<Pci> {
    let bus = 0;
    let mut slot = 0;

    while let Some(pci)= pci_get(bus, slot, 0) {
        if pci.class == class && pci.subclass == subclass {
            return Some(pci);
        }
        if pci.header_type & 0x80 != 0 {
            let mut func = 0;
            while let Some(pci)= pci_get(bus, slot, func) {
                // device has multiple functions
                if pci.class == class && pci.subclass == subclass {
                    return Some(pci);
                }
                func += 1;
                if func == 7 {break;}
            }
        }
        slot += 1;
        if slot == 32 {break;}
    }
    None
}

pub fn pci_get(bus: u32, slot: u32, func: u32) -> Option<Pci> {
    let vendor = pci_config_read_word(bus, slot, func, 0);
    if vendor == 0xffff { return None }
    let pci = Pci {bus, slot, func, vendor,
        device: pci_config_read_word(bus, slot, func, 2),
        subclass: pci_config_read_byte(bus, slot, func, 10),
        class: pci_config_read_byte(bus, slot, func, 11),
        header_type: pci_config_read_byte(bus, slot, func, 14),
    };
    Some(pci)
}

use core::fmt;
impl fmt::Display for Pci {
     fn fmt(&self, f: &mut core::fmt::Formatter) -> fmt::Result {
             write!(f, "{}:{}.{} {:#x},{:#x} {:#x} {:#x}",
                    self.bus, self.slot, self.func,
                    self.class, self.subclass,
                    self.vendor, self.device);
             Ok(())
     }
}

// pub fn pci_display(bus: u32, slot: u32, function: u32) {
//     let vendor = pci_config_read_word(bus, slot, function, 0);
//     let device = pci_config_read_word(bus, slot, function, 2);
//     println!("{}:{}.{} {:#x},{:#x}: {:#x} {:#x}",
//              bus, slot, function, class, subclass, vendor, device);
// }

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
