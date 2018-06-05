use x86::devices::pit;
use x86::devices::pic;
pub mod cpu;

pub unsafe fn init() {
    pic::init();
    pit::init();
}
