//! Arch-independent interrupts.
//!
//! Currently, you have to go to each architecture's module to run its initializer. The plan,
//! however, is to change this so that the correct function is just selected by `cfg`.

#[cfg(target_arch = "x86_64")]
mod interrupt_x64;
#[cfg(target_arch = "x86_64")]
pub use interrupt_x64::*;
