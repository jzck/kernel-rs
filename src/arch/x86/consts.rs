//layout looks like this:
// [kernel 0->4MiB]
// [kernel 4->8MiB]
// [user stack]
// [start of userheap]
// .
// .
// .
// [end of userheap]
// [kernel heap]
// [recursive map] points to first entry
//
// which makes userheap = 4GiB - 4*4MiB (~= 4GiB)
macro_rules! p2 { ($i:expr) => {($i & P2_MASK) / P1_SIZE}}
pub const P2_MASK: u32 = 0xFFC0_0000; //top 10 bits
pub const P1_SIZE: u32 = 0x0040_0000; //4MiB (2**22)

pub const RECURSIVE_PAGE_OFFSET: u32 = (-(P1_SIZE as isize)) as u32;
///1023
pub const RECURSIVE_PAGE_P2: u32 = p2!(RECURSIVE_PAGE_OFFSET);

pub const KERNEL_HEAP_OFFSET: u32 = RECURSIVE_PAGE_OFFSET - P1_SIZE;
///1022
pub const KERNEL_HEAP_P2: u32 = p2!(KERNEL_HEAP_OFFSET);
pub const KERNEL_HEAP_SIZE: u32 = 4 * 1024 * 1024; //4MiB (1 huge page)

pub const USER_OFFSET: u32 = 0x00a0_0000; //3rd page (kernel takes 2 first pages)
pub const USER_P2: u32 = p2!(USER_OFFSET);

pub const USER_STACK_OFFSET: u32 = USER_OFFSET + P1_SIZE;
pub const USER_STACK_2: u32 = p2!(USER_STACK_OFFSET);
