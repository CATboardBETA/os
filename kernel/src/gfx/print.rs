//! Printing functions. By default, these will print to a scrolling framebuffer.

use core::fmt::Arguments;

/// Effectively the same as a `println!` call in the standard library.
/// Makes use of the internal [`print`] function
#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/// Effectively the same as a `print!` call in the standard library.
/// Makes use of the internal [`print`] function
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::gfx::print::print(format_args!($($arg)*)));
}

/// Internal function for the [`crate::print!`] & [`crate::println!`] macros.
/// TODO: Make this not todo
pub fn print(_args: Arguments<'_>) {
    todo!()
}
