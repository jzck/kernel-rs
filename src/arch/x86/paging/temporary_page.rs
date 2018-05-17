use super::ActivePageTable;
use x86::*;
use x86::structures::paging::*;

pub struct TemporaryPage {
    pub page: Page,
}

impl TemporaryPage {
    pub fn new(page: Page) -> TemporaryPage {
        TemporaryPage { page: page }
    }

    /// Maps the temporary page to the given frame in the active table.
    /// Returns the start address of the temporary page.
    pub fn map(&mut self, frame: PhysFrame, active_table: &mut ActivePageTable) -> VirtAddr {
        assert!(
            active_table.translate_page(self.page).is_none(),
            "temporary page is already mapped"
        );
        active_table.map_to(self.page, frame, PageTableFlags::WRITABLE);
        // this kind of check should be done in a test routine
        assert!(
            active_table.translate_page(self.page).is_some(),
            "temporary page was not mapped"
        );
        self.page.start_address()
    }

    /// Unmaps the temporary page in the active table.
    pub fn unmap(&mut self, active_table: &mut ActivePageTable) {
        active_table.unmap(self.page)
    }

    /// Maps the temporary page to the given page table frame in the active
    /// table. Returns a reference to the now mapped table.
    pub fn map_table_frame(
        &mut self,
        frame: PhysFrame,
        active_table: &mut ActivePageTable,
    ) -> &mut PageTable {
        unsafe { &mut *(self.map(frame, active_table).as_u32() as *mut PageTable) }
    }
}
