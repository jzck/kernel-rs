use multiboot2;
use memory;
use vga;

pub static mut CONTEXT: Option<Context> = None;

pub struct Context {
    pub current_term: u8,
    pub multiboot_start: usize,
    pub multiboot_end: usize,
    pub kernel_start: usize,
    pub kernel_end: usize,
    pub boot_info: multiboot2::BootInformation,
    pub frame_allocator: memory::AreaFrameAllocator,
    pub vga1: vga::Writer,
    pub vga2: vga::Writer,
}

impl Context
{
    pub fn new(multiboot_start: usize) -> Context {
        let boot_info = unsafe { multiboot2::load(multiboot_start) };
        let multiboot_end = multiboot_start + boot_info.total_size();

        let elf_sections_tag = boot_info.elf_sections_tag().unwrap();
        let memory_map_tag = boot_info.memory_map_tag().unwrap();

        let kernel_start = elf_sections_tag.sections().map(
            |s| s.start_address())
            .min().unwrap() as usize;
        let kernel_end = elf_sections_tag.sections().map(
            |s| s.start_address() + s.size())
            .max().unwrap() as usize;

        let frame_allocator = memory::AreaFrameAllocator::new(
            kernel_start, kernel_end, multiboot_start,
            multiboot_end, memory_map_tag.memory_areas());

        let vga1 = vga::Writer::new();
        let vga2 = vga::Writer::new();

        Context {
            current_term: 0,
            multiboot_start,
            multiboot_end,
            kernel_start,
            kernel_end,
            boot_info,
            frame_allocator,
            vga1,
            vga2,
        }
    }

    pub fn init_screen(&mut self) {
        self.vga1.prompt();
        self.vga2.prompt();
        self.vga1.flush();
    }

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

pub fn context() -> &'static mut Context {
    unsafe {
        match CONTEXT {
            Some(ref mut x) => &mut *x,
            None => panic!(),
        }
    }
}
