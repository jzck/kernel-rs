//! x86 (32 bit) only

pub unsafe fn cr0_write(val: usize) {
     asm!("mov $0, %cr0" :: "r"(val) : "memory");
}

pub fn cr0() -> usize {
    let ret: usize;
    unsafe { asm!("mov %cr0, $0" : "=r" (ret)) };
    ret
}

pub fn cr3() -> usize {
    let ret: usize;
    unsafe { asm!("mov %cr3, $0" : "=r" (ret)) };
    ret
}

pub fn cr4() -> usize {
    let ret: usize;
    unsafe { asm!("mov %cr4, $0" : "=r" (ret)) };
    ret
}

pub unsafe fn cr3_write(val: usize) {
    asm!("mov $0, %cr3" :: "r" (val) : "memory");
}
