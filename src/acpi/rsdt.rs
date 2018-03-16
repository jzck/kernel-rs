use super::{ACPISDTHeader,ACPISDTIter};

//TODO this can work only if pagging is disabled
static mut RSDT: Option<*const ACPISDTHeader> = None;

/// ## Initialize Root System Description Table (RSDT)
/// input param addr is contain in RSDP
pub fn init(addr: u32) -> Result <(), &'static str> {
    if ACPISDTHeader::valid(addr, "RSDT") {
        unsafe {RSDT = Some(addr as *const ACPISDTHeader)};
        return Ok(());
    }
    return Err("Can not find Root System Description Table (RSDT).");
}

/// Return a iterable of ptr contained in RSDT
/// RSDT must have been initialized first
pub fn iter() -> Result <ACPISDTIter, &'static str> {
    ACPISDTIter::new(unsafe {RSDT}, 4)
}
