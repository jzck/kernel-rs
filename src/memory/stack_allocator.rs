use x86::*;
use x86::structures::paging::*;
use memory::paging::{self, ActivePageTable};
use memory::*;
use core::ops::Range;

#[derive(Debug)]
pub struct Stack {
    top: usize,
    bottom: usize,
}

impl Stack {
    fn new (top: usize, bottom: usize) -> Stack {
        assert!(top > bottom);
        Stack {
            top,
            bottom,
        }
    }

    pub fn top(&self) -> usize {
        self.top
    }

    pub fn bottom(&self) -> usize {
        self.bottom
    }
}

#[derive(Debug)]
pub struct StackAllocator {
    range: Range<Page>
}

impl StackAllocator {
    pub fn new(range: Range<Page>) -> StackAllocator {
        StackAllocator { range }
    }

    pub fn alloc_stack<FA: FrameAllocator>(&mut self,
                                           active_table: &mut ActivePageTable,
                                           frame_allocator: &mut FA,
                                           size_in_pages: usize) -> Option<Stack> {
        if size_in_pages == 0 {
            return None; /* a zero sized stack makes no sense */
        }

        // clone the range, since we only want to change it on success
        let mut range = self.range.clone();

        // try to allocate the stack pages and a guard page
        let guard_page = range.next();
        let stack_start = range.next();
        let stack_end = if size_in_pages == 1 {
            stack_start
        } else {
            // choose the (size_in_pages-2)th element, since index
            // starts at 0 and we already allocated the start page
            range.nth(size_in_pages - 2)
        };

        match (guard_page, stack_start, stack_end) {
            (Some(_), Some(start), Some(end)) => {
                // success! write back updated range
                self.range = range.clone();

                // map stack pages to physical frames
                for page in range {
                    active_table.map(page, PageTableFlags::WRITABLE, frame_allocator);
                }

                // create a new stack
                let top_of_stack = end.start_address().as_u32() + PAGE_SIZE as u32;
                Some(Stack::new(top_of_stack as usize,
                                start.start_address().as_u32() as usize))
            }
            _ => None, /* not enough pages */
        }
    }
}
