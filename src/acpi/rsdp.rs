use super::{check_signature,check_checksum};
use core::mem;

#[repr(C)]
struct RSDP {
    signature: [u8; 8],
    checksum: u8,
    oemid: [u8; 6],
    revision: u8,
    rsdtaddr: u32,
}

#[repr(C)]
pub struct RSDP20 {
    rsdp: RSDP,
    length: u32,
    xsdtaddress: u64,
    extendedchecksum: u8,
    reserved: [u8; 3],
}

static mut RSDPTR: Option<*const RSDP20> = None;

/// RSDP load will check is RSDP is present at the addr sent.
/// Return a bool
///         true => RSDP is V2
///         false => RSDP is V1
pub fn load(addr: u32) -> Result <bool, &'static str> {
    if check_signature(addr, "RSD PTR ") {
        let ptr_tmp = addr as *const RSDP20;
        let revision = unsafe {(*ptr_tmp).rsdp.revision};
        if (revision == 0 && check_checksum(addr, mem::size_of::<RSDP>())) || (revision == 2 && check_checksum(addr, mem::size_of::<RSDP20>())) {
            unsafe {RSDPTR = Some(ptr_tmp)};
            return Ok(revision == 2);
        }
    }
    Err("Not a valid RSD ptr")
}

fn memory_finding() -> Result <bool, &'static str> {
    let mut i = 0;
    while i < 0x1000000 {
        i += 8;
        if let Ok(result) = load(i) {
            return Ok(result)
        }
    }
    Err("Can not find Root System Description Pointer (RSDP).")
}

fn is_init() -> Result <*const RSDP20, &'static str> {
    match unsafe {RSDPTR} {
        Some(ptr)   => Ok(ptr),
        None        => Err("Root System Description Pointer (RSDP) is not initialized")
    }
}

/// Return a ptr on xsdt
/// RSDP must have been initialized first
pub fn xsdtaddr() -> Result <u64, &'static str> {
    let ptr = is_init()?;
    let revision = unsafe {(*ptr).rsdp.revision};
    if revision != 2 {
        return Err("Wrong RSDP version asked");
    }
    return Ok(unsafe {(*ptr).xsdtaddress});
}

/// Return a ptr on rsdt
/// RSDP must have been initialized first
pub fn rsdtaddr() -> Result <u32, &'static str> {
    let ptr = is_init()?;
    return Ok(unsafe {(*ptr).rsdp.rsdtaddr});
}

/// RSDP init will iter on addr in [0x0 - 0x1000000] to find "RSDP PTR "
/// if you already know the location, you should prefer to use load function
/// return an Error if there is no RSDP in memory, or return the value of load function
pub fn init() -> Result <bool, &'static str> {
    memory_finding()
}
