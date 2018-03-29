use x86::structures::idt::*;

interrupt!(keyboard, {
    println!("key pressed!");
    flush!();
});
