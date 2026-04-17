//! Implementation of the global allocator with debugging, for debug builds.

use crate::reqs::MEMMAP;
use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;
use core::mem::{ManuallyDrop, MaybeUninit};
use core::ops::{Deref, IndexMut};
use core::ptr::null_mut;
use core::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use limine::memmap::MEMMAP_USABLE;
use smallvec::SmallVec;
use spin::{Mutex, RwLock};

/// Denominator of the fraction used to determine how much to pad Assumed the numerator is 1. This
/// means you can only pad a maximum of 100% on each side, which should be plenty.
const PAD_FRAC_DENOM: usize = 2;
/// Max number of blocks in
const MAX_BLOCK_COUNT: usize = 31;
/// Max number of gaps per block
const MAX_BLOCK_GAPS: usize = 0x50;

/// Debug global allocator. See documentation for [`init_global`], and field documentation.
struct Global {
    /// An array of the [`Block`]s
    // A RwLock is really overkill here. I should change this to something else, once I get a
    // functional implementation
    blocks: RwLock<[MaybeUninit<Block>; MAX_BLOCK_COUNT]>,
    /// An array of all currently active allocations
    allocations: RwLock<SmallVec<Allocation, 5>>,
    /// Total number of active allocations
    allocation_count: AtomicUsize,
    /// Total number of blocks available for allocations
    block_count: AtomicUsize,
}

impl Global {
    /// Initializes an uninitialized global allocator.
    const fn new_uninit() -> Self {
        let mut blocks = [const { MaybeUninit::uninit() }; MAX_BLOCK_COUNT];
        Self {
            blocks: RwLock::new(blocks),
            allocations: RwLock::new(SmallVec::new()),
            allocation_count: AtomicUsize::new(0),
            block_count: AtomicUsize::new(0),
        }
    }

    /// Finds the first valid gap in the first valid block and starting address given a required
    /// length and alignment.
    fn find_free_block_addr(&self, len: usize, align: usize) -> Option<(usize, usize)> {
        let block_count = self.block_count.load(Ordering::Relaxed);
        let blocks = self.blocks.read();
        for (b_num, block) in unsafe {
            blocks
                .iter()
                .enumerate()
        } {
            let block = unsafe { block.assume_init_ref() };
            // Iterate over all the gaps in the block;
            let maybe_found = unsafe {
                block
                    .gaps
                    .lock()
                    .iter()
                    .copied()
                    .find(|&g| {
                        // If one of the blocks has enough space, we short-circuit
                        g >= len + (len / (PAD_FRAC_DENOM / 2))
                    })
            };
            if let Some(found) = maybe_found {
                // No need to check any later blocks, this is not an optimal solution
                return Some((b_num, found));
            }
        }
        None
    }
}

/// Represents one block of memory.
///
/// These are added from the [`limine::request::MemmapRequest`].
struct Block {
    /// Start of the block in physical mem
    start: usize,
    /// Length of the block
    len: usize,
    /// An array of all free space in the block. An empty block should have one gap, with
    /// `gaps[0] == len`
    gaps: Mutex<SmallVec<usize, 2>>,
}

/// An allocation, including the padding.
struct Allocation {
    /// Block number
    block: usize,
    /// Start of the actual allocation, in block virtual mem. Does not include the padding.
    start_addr: usize,
    /// Total size of the allocation in bytes
    size: usize,
    /// Required minimum alignment for this allocation,
    align: usize,
    /// The amount of padding in bytes on both the beginning and end of the allocation
    padding_amt: usize,
}

/// The global allocator itself. Only used in debug builds.
#[global_allocator]
static GLOBAL: Global = Global::new_uninit();

unsafe impl Send for Global {}
unsafe impl Sync for Global {}

unsafe impl GlobalAlloc for Global {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let alignment = layout.align();
        if let Some((block, addr)) = self.find_free_block_addr(size, alignment) {
            let allocation = Allocation {
                block,
                start_addr: addr,
                size,
                align: alignment,
                padding_amt: size / PAD_FRAC_DENOM,
            };
            unsafe {
                *self.allocations.write().index_mut(self.allocation_count.fetch_add(1, Ordering::Relaxed) + 1) = allocation;
            }
            addr as _
        } else {
            null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        todo!()
    }
}

/// This takes all memory that [`limine`] says are usable right off the bat and makes it
/// allocable. Note that allocations are significantly larger when calling this version of the
/// function, as opposed to [`crate::alloc_handler::opt::init_global`], because each allocation
/// is padded to catch memory errors.
///
/// It is critical that no allocations be performed prior to calling this function, for we call
/// [`MaybeUninit::assume_init`] and related functions.
pub fn init_global() {
    let memmap = MEMMAP.response().unwrap().entries();

    for entry in memmap {
        if entry.type_ == MEMMAP_USABLE {
            let idx = GLOBAL.block_count.fetch_add(1, Ordering::Relaxed);

            #[allow(unused_results)]
            unsafe {
                GLOBAL.blocks.write().index_mut(idx).write(Block {
                    start: entry.base as _,
                    len: entry.length as _,
                    gaps: Mutex::new(SmallVec::new()),
                });
            }
        }
    }
}
