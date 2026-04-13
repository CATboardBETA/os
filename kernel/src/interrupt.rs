#![allow(unused_imports)]

#[cfg(target_arch = "x86_64")]
mod interrupt_x64;
#[cfg(target_arch = "x86_64")]
pub use interrupt_x64::*;
