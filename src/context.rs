use multiboot2;
use memory;
use vga;

pub static mut CONTEXT: Option<Context> = None;

pub struct Context {
    pub current_term: u8,
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
            boot_info,
            frame_allocator,
            vga1,
            vga2,
        }
    }



}

pub fn init_screen() {
    set_color!(White, Cyan);
    print!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
           format_args!("{: ^80}", r#"        ,--,               "#),
           format_args!("{: ^80}", r#"      ,--.'|      ,----,   "#),
           format_args!("{: ^80}", r#"   ,--,  | :    .'   .' \  "#),
           format_args!("{: ^80}", r#",---.'|  : '  ,----,'    | "#),
           format_args!("{: ^80}", r#";   : |  | ;  |    :  .  ; "#),
           format_args!("{: ^80}", r#"|   | : _' |  ;    |.'  /  "#),
           format_args!("{: ^80}", r#":   : |.'  |  `----'/  ;   "#),
           format_args!("{: ^80}", r#"|   ' '  ; :    /  ;  /    "#),
           format_args!("{: ^80}", r#"\   \  .'. |   ;  /  /-,   "#),
           format_args!("{: ^80}", r#" `---`:  | '  /  /  /.`|   "#),
           format_args!("{: ^80}", r#"      '  ; |./__;      :   "#),
           format_args!("{: ^80}", r#"      |  : ;|   :    .'    "#),
           format_args!("{: ^80}", r#"      '  ,/ ;   | .'       "#),
           format_args!("{: ^80}", r#"      '--'  `---'          "#));
    set_color!();
    context().vga1.prompt();
    context().vga2.prompt();
    context().vga1.flush();
}

pub fn frame_allocator() -> &'static mut memory::AreaFrameAllocator {
    &mut context().frame_allocator
}

pub fn boot_info() -> &'static multiboot2::BootInformation {
    &context().boot_info
}

pub fn switch_term() {
    context().current_term = {
        if context().current_term == 0 { 1 }
        else { 0 }
    };
}

pub fn current_term() -> &'static mut vga::Writer{
    if context().current_term == 0 {
        &mut context().vga1
    } else {
        &mut context().vga2
    }
}

fn context() -> &'static mut Context {
    unsafe {
        match CONTEXT {
            Some(ref mut x) => &mut *x,
            None => panic!(),
        }
    }
}

pub fn init(multiboot_info_addr: usize) {
    unsafe { CONTEXT = Some(Context::new(multiboot_info_addr)) };

    memory::remap_the_kernel(frame_allocator(), boot_info());
    self::init_screen();
}
