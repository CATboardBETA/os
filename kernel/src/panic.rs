//! Panic handler lang item and any code it depends on.

use crate::{hcf, println};
use core::panic::PanicInfo;
use core::sync::atomic::{AtomicBool, Ordering};


/// Tracks whether a panic has already occured. Used in order to prevent double faults
/// from the formatting macro.
static PANICKED: AtomicBool = AtomicBool::new(false);

/// The panic handler itself. Try not to panic in this function, if you can.
///
/// This is a lang item—so normally it would be provided by default, but because we're in a 
/// `no_std` environment, we need to provide a few ourselves (notably, `panic_handler` and 
/// `global_allocator`)
/// 
/// This function should probably not be called by hand.
#[panic_handler]
#[cfg(not(test))]
fn rust_panic(info: &PanicInfo) -> ! {
    // Is this the right ordering? Someone who knows atomics better lemme know.
    if !PANICKED.swap(true, Ordering::AcqRel) {
        println!("{info}");
    }
    // otherwise, we already panicked once, probably from the println invocation. So we just don't
    // do anything to avoid a double/triple fault
    hcf()
}
