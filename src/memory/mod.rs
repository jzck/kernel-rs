mod bump;
mod recycle;
// mod stack_allocator;

use multiboot2;
use x86::structures::paging::*;

use self::bump::BumpFrameAllocator;
use self::recycle::RecycleAllocator;
// use self::stack_allocator::{Stack, StackAllocator};

pub trait FrameAllocator {
    fn allocate_frames(&mut self, size: usize) -> Option<PhysFrame>;
    fn deallocate_frames(&mut self, frame: PhysFrame, size: usize);
}

pub struct MemoryControler {
    frame_allocator: RecycleAllocator<BumpFrameAllocator>,
    // stack_allocator: StackAllocator,
}

static mut MEMORY_CONTROLER: Option<MemoryControler> = None;

pub fn init(boot_info: &multiboot2::BootInformation) {
    let elf_sections_tag = boot_info.elf_sections_tag().unwrap();
    let memory_map_tag = boot_info.memory_map_tag().unwrap();

    let kernel_start = elf_sections_tag
        .sections()
        .filter(|s| s.is_allocated())
        .map(|s| s.start_address())
        .min()
        .unwrap();

    let kernel_end = elf_sections_tag
        .sections()
        .filter(|s| s.is_allocated())
        .map(|s| s.start_address() + s.size())
        .max()
        .unwrap();

    let bump_allocator = BumpFrameAllocator::new(
        kernel_start as usize,
        kernel_end as usize,
        boot_info.start_address(),
        boot_info.end_address(),
        memory_map_tag.memory_areas(),
    );

    let frame_allocator = RecycleAllocator::new(bump_allocator);
    // let stack_allocator = StackAllocator::new(::USER_STACK_RANGE);

    unsafe {
        MEMORY_CONTROLER = Some(MemoryControler {
            frame_allocator,
            // stack_allocator,
        });
    }
}

pub fn allocate_frames(count: usize) -> Option<PhysFrame> {
    unsafe {
        if let Some(ref mut controler) = MEMORY_CONTROLER {
            controler.frame_allocator.allocate_frames(count)
        } else {
            panic!("frame allocator not initialized!");
        }
    }
}

pub fn deallocate_frames(frame: PhysFrame, count: usize) {
    unsafe {
        if let Some(ref mut controler) = MEMORY_CONTROLER {
            controler.frame_allocator.deallocate_frames(frame, count)
        } else {
            panic!("frame allocator not initialized!");
        }
    }
}

// pub fn allocate_stack(mut active_table: &mut ActivePageTable) -> Option<Stack> {
//     unsafe {
//         if let Some(ref mut controler) = MEMORY_CONTROLER {
//             controler
//                 .stack_allocator
//                 .allocate_stack(&mut active_table, 4)
//         } else {
//             panic!("frame allocator not initialized!");
//         }
//     }
// }

/// Init memory module after core
/// Must be called once, and only once,
pub fn init_noncore() {
    unsafe {
        if let Some(ref mut controler) = MEMORY_CONTROLER {
            controler.frame_allocator.set_core(true);
        } else {
            panic!("frame allocator not initialized");
        }
    }
}


/// for console
#[allow(unconditional_recursion)]
pub fn overflow() {
    overflow();
}

/// for console
pub fn page_fault() {
    unsafe {
        *(0xdead as *mut u32) = 42;
    };
}

/// Print the kernel stack
pub fn print_stack() {
    fn hexdump(start: usize, end: usize) {
        let mut address = 0;
        let data = unsafe { core::slice::from_raw_parts_mut(start as *mut u8, end - start) };
        while address <= data.len() {
            let next_end = core::cmp::min(address + 16, data.len());
            print_line(&data[address..next_end], address + start);
            address = address + 16;
        }
        println!("");
    }

    fn is_control(c: char) -> bool {
        !(c >= ' ' && c <= '~')
    }

    fn print_line(line: &[u8], address: usize) {
        print!("\n{:#08x}: ", address);
        for byte in line {
            print!("{:02x} ", *byte);
        }
        let length: usize = 16 - line.len();
        for _ in 0..length {
            print!("   ");
        }
        print!("|");
        for byte in line {
            match is_control(*byte as char) {
                true => print!("."),
                false => print!("{}", *byte as char),
            };
        }
        print!("|");
    }

    let esp: usize;
    let ebp: usize;
    unsafe { asm!("" : "={esp}"(esp), "={ebp}"(ebp):::) };
    println!("esp = {:#x}", esp);
    println!("ebp = {:#x}", ebp);
    println!("size = {:#X} bytes", ebp - esp);
    hexdump(esp, ebp);
    flush!();
}
