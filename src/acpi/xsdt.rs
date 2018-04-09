use super::{ACPISDTHeader, ACPISDTIter};

//TODO this can work only if pagging is disabled
static mut XSDT: Option<*const ACPISDTHeader> = None;

/// ## Initialize Root System Description Table (XSDT)
/// input param addr is contain in RSDP
pub fn init(addr: u64) -> Result<(), &'static str> {
    assert!((addr as u32) as u64 == addr);
    let addr: u32 = addr as u32;
    if ACPISDTHeader::valid(addr, "XSDT") {
        unsafe { XSDT = Some(addr as *const ACPISDTHeader) };
        return Ok(());
    }
    return Err("Can not find eXtended System Descriptor Table (XSDT).");
}

/// Return a iterable of ptr contained in XSDT
/// XSDT must have been initialized first
pub fn iter() -> Result<ACPISDTIter, &'static str> {
    ACPISDTIter::new(unsafe { XSDT }, 8)
}
