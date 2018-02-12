extern crate core;

use vga;


pub struct Context {
    pub current_term: u8,
    pub vga1: vga::Writer<&'static mut [u8]>,
    pub vga2: vga::Writer<&'static mut [u8]>,
}

impl Context {
    pub fn new() -> Context {
        let slice1 = unsafe { 
            core::slice::from_raw_parts_mut(0xb8000 as *mut u8, 4000)
        };

        let slice2 = unsafe { 
            core::slice::from_raw_parts_mut(0xb8000 as *mut u8, 4000)
        };

        Context {
            current_term: 0,
            vga1: vga::Writer::new(slice1),
            vga2: vga::Writer::new(slice2),
        }
    }

    pub fn switch_term(&mut self) {
        self.current_term = {
            if self.current_term == 0 { 1 }
            else { 0 }
        };
    }

    pub fn current_term(&mut self) -> &mut vga::Writer<&'static mut [u8]>{
        if self.current_term == 0 {
            &mut self.vga1
        } else {
            &mut self.vga2
        }
    }
}

