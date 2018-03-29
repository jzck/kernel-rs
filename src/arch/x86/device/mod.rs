pub mod pic;

pub fn init() {
    unsafe { pic::init(); }
}
