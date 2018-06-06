use x86::devices::pit;
use x86::devices::pic;
pub mod cpu;
use x86::instructions::interrupts;

static CHAN0_DIVISOR: u16 = 2685;

pub unsafe fn init() {
    pic::init_cascade();
    pic::disable_irqs();
    pic::enable_irq(0);
    pic::enable_irq(1);
    pit::CHAN0.set_divisor(CHAN0_DIVISOR);
    interrupts::enable();
}
