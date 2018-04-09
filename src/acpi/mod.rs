mod rsdp;
mod rsdt;
mod xsdt;
mod fadt;
mod dsdt;

use core;
use core::mem;
// use cpuio;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct ACPISDTHeader {
    signature: [u8; 4],
    length: u32,
    revision: u8,
    checksum: u8,
    oemid: [u8; 6],
    oemtableid: [u8; 8],
    oemrevision: u32,
    creatorid: u32,
    creatorrevision: u32,
}

impl ACPISDTHeader {
    pub fn valid(addr: u32, signature: &str) -> bool {
        if check_signature(addr, signature) {
            let ptr_tmp = addr as *const ACPISDTHeader;
            if check_checksum(addr, unsafe { (*ptr_tmp).length } as usize) {
                return true;
            }
        }
        return false;
    }
}

static mut ACPI: Acpi = Acpi {
    valid: false,
    v2: false,
};

struct Acpi {
    valid: bool,
    v2: bool,
}

impl Acpi {
    fn common_init(&mut self) -> Result<(), &'static str> {
        if self.v2 {
            // Xsdt Address:
            // 64-bit physical address of the XSDT table. If you detect ACPI Version 2.0 you should use this table instead of RSDT even on x86, casting the address to uint32_t.
            xsdt::init(rsdp::xsdtaddr()?)?;
            fadt::init(xsdt::iter()?)?;
        } else {
            rsdt::init(rsdp::rsdtaddr()?)?;
            fadt::init(rsdt::iter()?)?;
        }
        dsdt::init(fadt::dsdtaddr()?)?;
        self.valid = true;
        Ok(())
    }
    fn init(&mut self) -> Result<(), &'static str> {
        self.v2 = rsdp::init()?;
        self.common_init()
    }
    fn load(&mut self, rsdp_addr: u32) -> Result<(), &'static str> {
        self.v2 = rsdp::load(rsdp_addr)?;
        self.common_init()
    }
}

fn check_signature(addr: u32, id: &str) -> bool {
    let signature = match core::str::from_utf8(unsafe {
        core::slice::from_raw_parts_mut(addr as *mut u8, id.len())
    }) {
        Ok(y) => y,
        Err(_) => return false,
    };
    return signature == id;
}

fn check_checksum(addr: u32, len: usize) -> bool {
    let byte_array = unsafe { core::slice::from_raw_parts_mut(addr as *mut u8, len) };
    let mut sum: u32 = 0;
    for byte in byte_array {
        sum += *byte as u32;
    }
    return sum as u8 == 0;
}

pub struct ACPISDTIter {
    pos: usize,
    width: usize,
    sdt: u32,
    len: usize,
}

impl ACPISDTIter {
    fn new(
        acpi_sdt: Option<*const ACPISDTHeader>,
        ptr_len: usize,
    ) -> Result<ACPISDTIter, &'static str> {
        match acpi_sdt {
            None => Err("There is no ACPI System Description Table (ACPISDTHeader) to iter on."),
            Some(ptr) => Ok(ACPISDTIter {
                pos: 0,
                width: ptr_len,
                sdt: ptr as u32 + mem::size_of::<ACPISDTHeader>() as u32,
                len: (unsafe { (*ptr).length } as usize - mem::size_of::<ACPISDTHeader>())
                    / ptr_len,
            }),
        }
    }
}

impl Iterator for ACPISDTIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.pos += 1;
        if self.pos > self.len {
            return None;
        }
        let ret = Some(unsafe { *(self.sdt as *const u32) });
        self.sdt += self.width as u32;
        return ret;
    }
}

fn is_init() -> Result<(), &'static str> {
    if unsafe { ACPI.valid } {
        Ok(())
    } else {
        Err("ACPI is not initialized")
    }
}

/// Initalized the ACPI module
pub fn init() -> Result<(), &'static str> {
    if let Ok(()) = is_init() {
        return Ok(());
    }
    unsafe { ACPI.init() }
}

/// Load the ACPI module, addr given is a ptr to RSDP
pub fn load(rsdp_addr: u32) -> Result<(), &'static str> {
    if let Ok(()) = is_init() {
        return Ok(());
    }
    unsafe { ACPI.load(rsdp_addr) }
}

/// Proceed to ACPI shutdown
/// This function doesn't work with Virtual Box yet
pub fn shutdown() -> Result<(), &'static str> {
    is_init()?;
    dsdt::shutdown(fadt::get_controlblock()?)
}

/// Display state of ACPI
pub fn info() -> Result<(), &'static str> {
    is_init()?;
    println!(
        "ACPI STATE:\n    {}",
        if fadt::is_enable()? {
            "ENABLED"
        } else {
            "DISABLED"
        }
    );
    Ok(())
}
