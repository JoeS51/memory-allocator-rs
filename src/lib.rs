use std::{alloc::Layout, marker::PhantomData, mem::MaybeUninit, ops::Range, ptr::NonNull};

/// A safe numeric model of bump allocation.
///
/// Implement this before working with addresses or raw pointers.
#[derive(Debug)]
pub struct Cursor {
    capacity: usize,
    next: usize,
}

impl Cursor {
    pub fn new(capacity: usize) -> Self {
        todo!("checkpoint 1: initialize the numeric cursor")
    }

    pub fn allocate(&mut self, size: usize) -> Option<Range<usize>> {
        todo!("checkpoint 1: reserve a range without overflowing")
    }

    pub fn used(&self) -> usize {
        self.next
    }
}

/// Rounds `address` up to the next address divisible by `alignment`.
///
/// `alignment` must be a non-zero power of two. Return `None` on overflow.
pub fn align_up(address: usize, alignment: usize) -> Option<usize> {
    todo!("checkpoint 2: implement checked power-of-two alignment")
}

/// A bump allocator over a caller-provided byte buffer.
pub struct Bump<'a> {
    start: NonNull<u8>,
    capacity: usize,
    next: usize,
    _memory: PhantomData<&'a mut [MaybeUninit<u8>]>,
}

impl<'a> Bump<'a> {
    pub fn new(memory: &'a mut [MaybeUninit<u8>]) -> Self {
        todo!("checkpoint 3: borrow and record the backing memory")
    }

    /// Returns suitably aligned storage, or `None` if the request cannot fit.
    /// Zero-sized requests are deliberately unsupported in this exercise.
    pub fn allocate(&mut self, layout: Layout) -> Option<NonNull<u8>> {
        todo!("checkpoint 4: align, bounds-check, then commit the cursor")
    }

    pub fn used(&self) -> usize {
        self.next
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Reclaims the entire arena at once.
    ///
    /// # Safety
    ///
    /// Every pointer returned by this allocator must be dead before this call.
    /// Resetting does not run destructors for values placed in the arena.
    pub unsafe fn reset(&mut self) {
        todo!("checkpoint 5: reset the cursor")
    }
}
