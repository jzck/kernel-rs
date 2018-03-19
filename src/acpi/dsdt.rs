use super::{check_signature,ACPISDTHeader};
use core::mem;
use cpuio;

static mut DSDT: DSDT = DSDT {
    valid: false,
    dsdt: None,
    s5_ptr: 0,
    slp_typ_a: 0,
    slp_typ_b: 0
};

struct DSDT {
    valid: bool,
    dsdt: Option<&'static ACPISDTHeader>,
    s5_ptr: u32,
    slp_typ_a: u16,
    slp_typ_b: u16
}

impl DSDT {
    fn init(&mut self, addr: u32) -> Result <(), &'static str> {
        self.dsdt = Some(unsafe{ &(*(addr as *const ACPISDTHeader)) });
        self.s5_ptr = self.find_s5(addr)?;
        self.parse_s5();
        self.valid = true;
        Ok(())
    }

    fn find_s5(&self, addr: u32) -> Result <u32, &'static str> {
        let dsdt_start = addr + mem::size_of::<ACPISDTHeader>() as u32;
        let dsdt_end = dsdt_start + self.dsdt.unwrap().length;
        for addr in dsdt_start..dsdt_end {
            if check_signature(addr, "_S5_") {
                if (check_signature(addr - 1, "\x08") || check_signature(addr - 2, "\x08\\")) && check_signature(addr + 4, "\x12") {
                    return Ok(addr);
                }
            }
        }
        Err("Can not find S5 section in DSDT")
    }

    fn parse_s5(&mut self) {
        let ptr = self.s5_ptr + 5;
        let ptr = ((unsafe{*(ptr as *const u8)} & 0xC0) >> 6) + 2;
        let ptr = if unsafe{*(ptr as *const u8)} == 0x0A { ptr + 1 } else { ptr };// Skip bytePrefix
        self.slp_typ_a = (unsafe {*(ptr as *const u8)} as u16) << 10;
        let ptr = ptr + 1;
        let ptr = if unsafe{*(ptr as *const u8)} == 0x0A { ptr + 1 } else { ptr };// Skip bytePrefix
        self.slp_typ_b = (unsafe {*(ptr as *const u8)} as u16) << 10;
    }
}

fn is_init() -> Result <(), &'static str> {
    match unsafe {DSDT.valid} {
        true => Ok(()),
        false => match unsafe {DSDT.dsdt} {
            Some(_) => Err("Differentiated System Description Pointer (DSDP) is not valid"),
            None => Err("Differentiated System Description Pointer (DSDP) is not initialized")
        }
    }
}

/// ## Initialize Differentiated System Description Table (DSDT)
/// input param addr is contain in FADT
pub fn init(addr: u32) -> Result <(), &'static str> {
    if ACPISDTHeader::valid(addr, "DSDT") {
        return unsafe {DSDT.init(addr)};
    }
    return Err("Can not find Differentiated System Description Table (DSDT).");
}

/// NOT COMPATIBLE WITH VIRTUALBOX
/// Send shutdown signal
/// outw(PM1a_CNT_BLK, SLP_TYPx | SLP_EN) 
pub fn shutdown(pm1_cnt: [u16; 2]) -> Result <(), &'static str> {
    is_init()?;
    let slp_typ = unsafe{ DSDT.slp_typ_a } | (1 << 13);
    cpuio::outw(pm1_cnt[0], slp_typ);
    if pm1_cnt[1] != 0 {
        let slp_typ = unsafe{ DSDT.slp_typ_b } | (1 << 13);
        cpuio::outw(pm1_cnt[1], slp_typ);
    }
    Ok(())
}
