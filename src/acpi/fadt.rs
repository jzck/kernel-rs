use super::{ACPISDTHeader,ACPISDTIter};
use cpuio;

#[repr(C)]
#[derive(Debug)]
struct GenericAddressStructure {
    addressspace: u8,
    bitwidth: u8,
    bitoffset: u8,
    accesssize: u8,
    noused: u32,
    address: u64
}

#[repr(C)]
#[derive(Debug)]
struct FADT
{
    header: ACPISDTHeader,
    firmwarectrl: u32,
    dsdt: u32,

    // field used in acpi 1.0; no longer in use, for compatibility only
    reserved: u8,

    preferredpowermanagementprofile: u8,
    sci_interrupt: u16,
    smi_commandport: u32,
    acpi_enable: u8,
    acpidisable: u8,
    s4bios_req: u8, //no use
    pstate_control: u8, //no use
    pm1aeventblock: u32, //no use
    pm1beventblock: u32, //no use
    pm1acontrolblock: u32,
    pm1bcontrolblock: u32,
    pm2controlblock: u32, //no use
    pmtimerblock: u32, //no use
    gpe0block: u32, //no use
    gpe1block: u32, //no use
    pm1eventlength: u8, //no use
    pm1controllength: u8,
    pm2controllength: u8, //no use
    pmtimerlength: u8, //no use
    gpe0length: u8, //no use
    gpe1length: u8, //no use
    gpe1base: u8, //no use
    cstatecontrol: u8, //no use
    worstc2latency: u16, //no use
    worstc3latency: u16, //no use
    flushsize: u16, //no use
    flushstride: u16, //no use
    dutyoffset: u8, //no use
    dutywidth: u8, //no use
    dayalarm: u8, //no use
    monthalarm: u8, //no use
    century: u8, //no use

    // reserved in acpi 1.0; used since acpi 2.0+
    bootarchitectureflags: u16,

    reserved2: u8,
    flags: u32,

    // 12 byte structure; see below for details
    resetreg: GenericAddressStructure,

    resetvalue: u8,
    reserved3: [u8; 3],

    // 64bit pointers - Available on ACPI 2.0+
    x_firmwarecontrol: u64,
    x_dsdt: u64,

    x_pm1aeventblock: GenericAddressStructure,
    x_pm1beventblock: GenericAddressStructure,
    x_pm1acontrolblock: GenericAddressStructure,
    x_pm1bcontrolblock: GenericAddressStructure,
    x_pm2controlblock: GenericAddressStructure,
    x_pmtimerblock: GenericAddressStructure,
    x_gpe0block: GenericAddressStructure,
    x_gpe1block: GenericAddressStructure,

}

static mut FADT: Option<&'static FADT> = None;

/// ## Initialize Fixed ACPI Description Table (FADT)
/// input param addr is contain in other ptr of rsdt
pub fn init(sdt_iter: ACPISDTIter) -> Result <(), &'static str> {
    for sdt_ptr in sdt_iter {
        if ACPISDTHeader::valid(sdt_ptr, "FACP") { // Where is "FADT"? Shut up is magic
            let ref fadt_tmp: &FADT = unsafe{ &(*(sdt_ptr as *const FADT)) };
            unsafe {FADT = Some(fadt_tmp)};
            if !is_enable()? { // TODO do i have to check if enabled before init ???
                let smi_cmd = fadt_tmp.smi_commandport as u16; // TODO WHY DO I NEED THIS FUCKING CAST
                let acpi_enable = fadt_tmp.acpi_enable;
                cpuio::outb(smi_cmd, acpi_enable); // send acpi enable command
            }
            return Ok(());
        }
    }
    return Err("Can not find Fixed ACPI Description Table (FADT).");
}

fn is_init() -> Result <&'static FADT, &'static str> {
    match unsafe {FADT} {
        Some(fadt)   => Ok(fadt),
        None        => Err("Fixed ACPI Description Table (FADT) is not initialized")
    }
}

/// Return Dsdt address
/// FADT must have been initialized first
pub fn dsdtaddr() -> Result <u32, &'static str> {
    let fadt = is_init()?;
    return Ok(fadt.dsdt);
}

fn get_cnt(fadt: &'static FADT) -> [u16; 2] {
    [fadt.pm1acontrolblock as u16, fadt.pm1bcontrolblock as u16] // TODO WHY DO I NEED THIS FUCKING CAST
}

/// Return true/false depending of acpi is enable
pub fn is_enable() -> Result <bool, &'static str> {
    let fadt = is_init()?;
    let pm1_cnt = get_cnt(fadt);
    if pm1_cnt[1] == 0 {
        Ok(cpuio::inw(pm1_cnt[0]) & 0x1 == 0x1)
    } else {
        Ok(cpuio::inw(pm1_cnt[0]) & 0x1 == 0x1 || cpuio::inw(pm1_cnt[1]) & 0x1 == 0x1)
    }
}

/// Return a array with [pm1a, pm1b]
/// FADT must have been initialized first
pub fn get_controlblock() -> Result <[u16; 2], &'static str> {
    if !is_enable()? {
        Err("ACPI is not enabled")
    } else {
        Ok(get_cnt(is_init()?)) // TODO redondant call to is_init
    }
}
