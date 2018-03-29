//! Recycle allocator
//! Uses freed frames if possible, then uses inner allocator

use alloc::Vec;
use x86::*;
use x86::structures::paging::*;
use super::*;

pub struct RecycleAllocator<T: FrameAllocator> {
    inner: T,
    core: bool,
    free: Vec<(usize, usize)>,
}

impl<T: FrameAllocator> RecycleAllocator<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: inner,
            core: true,
            free: Vec::new(),
        }
    }

    pub fn set_core(&mut self, core: bool) {
        self.core = core;
    }

    fn merge(&mut self, address: usize, count: usize) -> bool {
        for i in 0 .. self.free.len() {
            let changed = {
                let free = &mut self.free[i];
                if address + count * 4096 == free.0 {
                    free.0 = address;
                    free.1 += count;
                    true
                } else if free.0 + free.1 * 4096 == address {
                    free.1 += count;
                    true
                } else {
                    false
                }
            };

            if changed {
                //TODO: Use do not use recursion
                let (address, count) = self.free[i];
                if self.merge(address, count) {
                    self.free.remove(i);
                }
                return true;
            }
        }

        false
    }
}

impl<T: FrameAllocator> FrameAllocator for RecycleAllocator<T> {
    fn allocate_frames(&mut self, count: usize) -> Option<PhysFrame> {
        let mut small_i = None;
        {
            let mut small = (0, 0);
            for i in 0..self.free.len() {
                let free = self.free[i];
                // Later entries can be removed faster
                if free.1 >= count {
                    if free.1 <= small.1 || small_i.is_none() {
                        small_i = Some(i);
                        small = free;
                    }
                }
            }
        }

        if let Some(i) = small_i {
            let (address, remove) = {
                let free = &mut self.free[i];
                free.1 -= count;
                (free.0 + free.1 * 4096, free.1 == 0)
            };

            if remove {
                self.free.remove(i);
            }

            //println!("Restoring frame {:?}, {}", frame, count);
            Some(PhysFrame::containing_address(PhysAddr::new(address as u32)))
        } else {
            //println!("No saved frames {}", count);
            self.inner.allocate_frames(count)
        }
    }

    fn deallocate_frames(&mut self, frame: PhysFrame, count: usize) {
        // we cant use vec! before the heap has been initialized
        if self.core {
            self.inner.deallocate_frames(frame, count);
        } else {
            let address = frame.start_address().as_u32() as usize;
            if ! self.merge(address, count) {
                self.free.push((address, count));
            }
        }
    }
}
