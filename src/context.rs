use multiboot2;
use memory;
use vga;

pub static mut CONTEXT: Option<Context> = None;

pub struct Context {
    pub current_term: u8,
    pub vga1: vga::Writer,
    pub vga2: vga::Writer,
}

impl Context
{
    pub fn new(multiboot_start: usize) -> Context {


        Context {
            current_term: 0,
            boot_info,
            vga1,
            vga2,
        }
    }
}

