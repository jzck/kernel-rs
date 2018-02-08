// #![no_std]
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

#![feature(lang_items)]
#![no_std]

#[lang = "eh_personality"]
extern fn eh_personality() {

}

#[lang = "panic_fmt"]
extern fn rust_begin_panic() -> ! {
    loop {}

}


#[no_mangle]
pub extern fn kmain() -> ! {
    unsafe {
        let vga = 0xb8000 as *mut u32;

        *vga = 0x2f592f412f;

    };

    loop {  }

}
