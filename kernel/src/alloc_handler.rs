//! Anything related to allocation, including the `global_allocator` lang item.
use crate::reqs::{HHDM, MEMMAP};
use limine::memmap::MEMMAP_USABLE;
use spin::Mutex;
use talc::source::Manual;
use talc::TalcLock;


/// This is gloal allocator. For now, until we roll our own allocator, we are using [`Talc`](talc).
#[global_allocator]
static GLOBAL: TalcLock<Mutex<()>, Manual> = TalcLock::new(Manual);

/// This takes all memory that [`limine`] says are usable right off the bat and makes it
/// allocable.
pub fn init_global() {
    let offset = HHDM.response().unwrap().offset;
    let memmap = MEMMAP.response().unwrap().entries();
    for entry in memmap {
        if entry.type_ == MEMMAP_USABLE {
            // SAFETY: So long as this function is called only once, this is fine. Otherwise, we're
            // claiming the same regions multiple times which is a no-no.
            unsafe {
                GLOBAL
                    .lock()
                    .claim((entry.base + offset) as _, entry.length as _);
            }
        }
    }
}
