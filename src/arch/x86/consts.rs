// p2 layout looks like this:
// [kernel 0->4MiB]
// [kernel 4->8MiB]
// [start of no man's land]
// .
// .
// .
// [end of no man's land]
// [user stack]
// [kernel heap]
// [recursive map] points to first entry
//
// no man's land should be used in the future for mmap() I guess
// the kernel right now takes up 2 pages with debug symbols,
// if it gets to 3 pages everything will crash and i'll have
// to hardcode the third page here and in `boot.asm` also!
//
// TODO
// kernel is mapped identically (start of memory) but in
// the future it should be higher-half mapped so that
// user memory starts at 0

use x86::structures::paging::*;
use x86::*;
use core::ops::Range;

// macro_rules! prange { ($i:expr, $s:expr) => {$i..$i+$s+1}}

// all of this is fucking contrived
// the rust compiler should be able to evaluate const expressions
// by using the impl for my structs, what i'm doing here is obvious
// to me but not to the compiler

pub const RECURSIVE_PAGE_OFFSET: VirtAddr = VirtAddr(0xffc0_000); // first 10 bits
pub const RECURSIVE_PAGE: Page = Page::containing_address(RECURSIVE_PAGE_OFFSET);
pub const RECURSIVE_PAGE_SIZE: u32 = 0x0040_0000; // the whole p2 entry

pub const KERNEL_HEAP_OFFSET: VirtAddr = VirtAddr(RECURSIVE_PAGE_OFFSET.0 - RECURSIVE_PAGE_SIZE);
// should be
// pub const KERNEL_HEAP_OFFSET: VirtAddr = RECURSIVE_PAGE_OFFSET - RECURSIVE_PAGE_SIZE);
pub const KERNEL_HEAP_SIZE: u32 = 0x0040_0000; //4MiB (1 huge page)
pub const KERNEL_HEAP_START: Page = Page::containing_address(KERNEL_HEAP_OFFSET);
pub const KERNEL_HEAP_END: Page =
    Page::containing_address(VirtAddr(KERNEL_HEAP_OFFSET.0 + KERNEL_HEAP_SIZE));
pub const KERNEL_HEAP_RANGE: Range<Page> = KERNEL_HEAP_START..KERNEL_HEAP_END;

pub const USER_STACK_OFFSET: VirtAddr = VirtAddr(KERNEL_HEAP_OFFSET.0 - KERNEL_HEAP_SIZE);
pub const USER_STACK_START: Page = Page::containing_address(USER_STACK_OFFSET);
pub const USER_STACK_SIZE: u32 = 0x0040_0000; //4MiB (1 huge page)
pub const USER_STACK_END: Page =
    Page::containing_address(VirtAddr(USER_STACK_OFFSET.0 + USER_STACK_SIZE));
pub const USER_STACK_RANGE: Range<Page> = USER_STACK_START..USER_STACK_END;
