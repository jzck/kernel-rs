use memory::*;
use multiboot2::{MemoryAreaIter, MemoryArea};

pub struct AreaFrameAllocator {
    next_free_frame: PhysFrame,
    current_area: Option<&'static MemoryArea>,
    areas: MemoryAreaIter,
    kernel_start: PhysFrame,
    kernel_end: PhysFrame,
    multiboot_start: PhysFrame,
    multiboot_end: PhysFrame,
}

impl AreaFrameAllocator {
    pub fn new(kernel_start: usize, kernel_end: usize,
               multiboot_start: usize, multiboot_end: usize,
               memory_areas: MemoryAreaIter) -> AreaFrameAllocator {
        let mut allocator = AreaFrameAllocator {
            next_free_frame: PhysFrame::containing_address(0),
            current_area: None,
            areas: memory_areas,
            kernel_start: PhysFrame::containing_address(kernel_start),
            kernel_end: PhysFrame::containing_address(kernel_end),
            multiboot_start: PhysFrame::containing_address(multiboot_start),
            multiboot_end: PhysFrame::containing_address(multiboot_end),
        };
        allocator.choose_next_area();
        allocator
    }

    fn choose_next_area(&mut self) {
        // get next area with free frames
        self.current_area = self.areas.clone().filter(|area| {
            PhysFrame::containing_address(area.end_address()) >= self.next_free_frame
        }).min_by_key(|area| area.start_address());

        if let Some(area) = self.current_area {
            let start_frame = PhysFrame::containing_address(area.start_address());
            if self.next_free_frame < start_frame {
                self.next_free_frame = start_frame;
            }
        }
    }
}

impl FrameAllocator for AreaFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        if let Some(area) = self.current_arPhysea {
            let frame = PhysFrame { number: self.next_free_frame.number };
            let current_area_last_frame = PhysFrame::containing_address(area.end_address());
            if frame > current_area_last_frame {
                // all frames are taken in this area
                self.choose_next_area();
            } else if frame >= self.kernel_start && frame <= self.kernel_end {
                // frame used by kernel
                self.next_free_frame = PhysFrame {
                    number: self.kernel_end.number + 1,
                }
            } else if frame >= self.multiboot_start && frame <= self.multiboot_end {
                // frame used by multiboot
                self.next_free_frame = PhysFrame {
                    number: self.multiboot_end.number + 1,
                }
            } else {
                self.next_free_frame.number += 1;
                return Some(frame);
            }
            // try again with next_free_frame
            self.allocate_frame()
        } else {
            None 
        }
    }

    fn deallocate_frame(&mut self, frame: PhysFrame) {
        unimplemented!();
    }
}
