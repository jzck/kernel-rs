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
