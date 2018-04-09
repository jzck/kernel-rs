use x86::structures::paging::*;

pub trait RecTable {
    fn next_table_address(&self, index: usize) -> Option<u32>;
    fn next_table(&self, index: usize) -> Option<&PageTable>;
    fn next_table_mut(&mut self, index: usize) -> Option<&mut PageTable>;
    fn next_table_create(&mut self, index: usize) -> &mut PageTable;
}

impl RecTable for PageTable {
    fn next_table_address(&self, index: usize) -> Option<u32> {
        let entry_flags = self[index].flags();
        if entry_flags.contains(PageTableFlags::PRESENT)
            && !entry_flags.contains(PageTableFlags::HUGE_PAGE)
        {
            let table_address = self as *const _ as usize;
            Some((table_address << 10 | index << 12) as u32)
        } else {
            None
        }
    }

    fn next_table(&self, index: usize) -> Option<&PageTable> {
        self.next_table_address(index)
            .map(|address| unsafe { &*(address as *const _) })
    }

    fn next_table_mut(&mut self, index: usize) -> Option<&mut PageTable> {
        self.next_table_address(index)
            .map(|address| unsafe { &mut *(address as *mut _) })
    }

    fn next_table_create(&mut self, index: usize) -> &mut PageTable {
        if self.next_table(index).is_none() {
            assert!(
                !self[index].flags().contains(PageTableFlags::HUGE_PAGE),
                "mapping code does not support huge pages"
            );
            let frame = ::memory::allocate_frames(1).expect("out of memory");
            self[index].set(frame, PageTableFlags::PRESENT | PageTableFlags::WRITABLE);
            self.next_table_mut(index)
                .expect("next_table_mut gave None")
                .zero()
        }
        self.next_table_mut(index).expect("no next table 2")
    }
}
