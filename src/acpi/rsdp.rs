use super::{check_checksum, check_signature};
use core::mem;

#[repr(C)]
#[derive(Clone)]
struct RSDP {
    signature: [u8; 8],
    checksum: u8,
    oemid: [u8; 6],
    revision: u8,
    rsdtaddr: u32,
}

#[repr(C)]
#[derive(Clone)]
pub struct RSDP20 {
    rsdp: RSDP,
    length: u32,
    xsdtaddress: u64,
    extendedchecksum: u8,
    reserved: [u8; 3],
}

static mut RSDPTR: Option<RSDP20> = None;

/// RSDP load will check is RSDP is present at the addr sent.
/// Return a bool
///         true => RSDP is V2
///         false => RSDP is V1
pub fn load(addr: u32) -> Result<bool, &'static str> {
    if check_signature(addr, "RSD PTR ") {
        let rsdp_tmp: RSDP20 = unsafe { (*(addr as *const RSDP20)).clone() };
        let revision = rsdp_tmp.rsdp.revision;
        if (revision == 0 && check_checksum(addr, mem::size_of::<RSDP>()))
            || (revision == 2 && check_checksum(addr, mem::size_of::<RSDP20>()))
        {
            unsafe { RSDPTR = Some(rsdp_tmp) };
            return Ok(revision == 2);
        }
    }
    Err("Not a valid RSD ptr")
}

fn memory_finding() -> Result<bool, &'static str> {
    let mut i = 0;
    while i < 0x1000000 {
        i += 8;
        if let Ok(result) = load(i) {
            return Ok(result);
        }
    }
    Err("Can not find Root System Description Pointer (RSDP).")
}

fn is_init() -> Result<RSDP20, &'static str> {
    match unsafe { RSDPTR.clone() } {
        Some(rsdptr) => Ok(rsdptr),
        None => Err("Root System Description Pointer (RSDP) is not initialized"),
    }
}

/// Return a ptr on xsdt
/// RSDP must have been initialized first
pub fn xsdtaddr() -> Result<u64, &'static str> {
    let rsdptr = is_init()?;
    let revision = rsdptr.rsdp.revision;
    if revision != 2 {
        return Err("Wrong RSDP version asked");
    }
    return Ok(rsdptr.xsdtaddress);
}

/// Return a ptr on rsdt
/// RSDP must have been initialized first
pub fn rsdtaddr() -> Result<u32, &'static str> {
    let rsdptr = is_init()?;
    return Ok(rsdptr.rsdp.rsdtaddr);
}

/// RSDP init will iter on addr in [0x0 - 0x1000000] to find "RSDP PTR "
/// if you already know the location, you should prefer to use load function
/// return an Error if there is no RSDP in memory, or return the value of load function
pub fn init() -> Result<bool, &'static str> {
    memory_finding()
}
