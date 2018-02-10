#[macro_use]
pub mod buffer;
pub mod color;
pub mod terminal;

pub use self::color::{Color, ColorCode};
pub use self::buffer::{WRITER, Screen, BufferAction};

// use self::buffer::Writer;
// use core::fmt;
// use core::ptr::Unique;

// macro_rules! println {
//     ($fmt:expr) => (print!(concat!($fmt, "\n")));
//     ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
// }

// macro_rules! print {
//     ($($arg:tt)*) => ({
//         $crate::vga::print(format_args!($($arg)*));
//     });
// }

// pub fn print(args: fmt::Arguments) {
//     use core::fmt::Write;
//     self::WRITER.write_fmt(args).unwrap();
// }
