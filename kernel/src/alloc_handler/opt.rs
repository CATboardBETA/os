//! Implementation of the global allocator with no debugging, for release builds. Implemented
//! using the [`talc`] crate.

use crate::reqs::{HHDM, MEMMAP};
use core::num::NonZero;
use core::ptr::NonNull;
use limine::memmap::MEMMAP_USABLE;
use spin::Mutex;
use talc::source::Manual;
use talc::TalcLock;


/// This is global allocator. Implemented using the [`Talc`](talc) crate.
#[global_allocator]
static GLOBAL: TalcLock<Mutex<()>, Manual> = TalcLock::new(Manual);

/// This takes all memory that [`limine`] says are usable right off the bat and makes it
/// allocable.
///
/// Adjacent memory chunks provided by limine are guarranteed to be combined.
pub fn init_global() {
    let offset = HHDM.response().unwrap().offset;
    let memmap = MEMMAP.response().unwrap().entries();
    let mut last_end: Option<NonNull<u8>> = None;
    // There is no need to sort the memmap, the limine protocol already guarrantees that it is
    // sorted by base address
    for entry in memmap {
        if entry.type_ == MEMMAP_USABLE {
            // SAFETY: So long as this function is called only once on a block of memory, this is
            // fine. Otherwise, we're claiming the same regions multiple times which is a no-no.
            if last_end.is_none() || last_end.unwrap().as_ptr().addr() != entry.base as usize {
                unsafe {
                    last_end = GLOBAL
                        .lock()
                        .claim((entry.base + offset) as _, entry.length as _);
                }
            } else {
                let mem = (entry.base + offset);
                unsafe {
                    last_end = Some(GLOBAL.lock().extend(
                        NonNull::without_provenance(NonZero::new(mem as usize).unwrap()),
                        (mem + entry.length) as *mut _,
                    ));
                }
            }
        }
    }
}
