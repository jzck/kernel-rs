extern crate core;

use vga;

pub static mut CONTEXT: Context = Context {
    current_term: 0,
    boot_info_addr: 0,
    vga1: vga::Writer::new(),
    vga2: vga::Writer::new(),
};

pub struct Context {
    pub current_term: u8,
    pub boot_info_addr: usize,
    pub vga1: vga::Writer,
    pub vga2: vga::Writer,
}

impl Context {
    pub fn switch_term(&mut self) {
        self.current_term = {
            if self.current_term == 0 { 1 }
            else { 0 }
        };
    }

    pub fn current_term(&mut self) -> &mut vga::Writer{
        if self.current_term == 0 {
            &mut self.vga1
        } else {
            &mut self.vga2
        }
    }
}

