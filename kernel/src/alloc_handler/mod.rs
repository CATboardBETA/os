//! Anything related to allocation, including the `global_allocator` lang item.

#[cfg(debug_assertions)]
mod dbg;
#[cfg(not(debug_assertions))]
mod opt;

#[cfg(debug_assertions)]
pub use dbg::*;
#[cfg(not(debug_assertions))]
pub use opt::*;
