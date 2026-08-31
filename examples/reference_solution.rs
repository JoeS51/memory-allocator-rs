use std::{alloc::Layout, marker::PhantomData, mem::MaybeUninit, ops::Range, ptr::NonNull};

#[derive(Debug)]
struct Cursor {
    capacity: usize,
    next: usize,
}

impl Cursor {
    fn new(capacity: usize) -> Self {
        Self { capacity, next: 0 }
    }

    fn allocate(&mut self, size: usize) -> Option<Range<usize>> {
        let start = self.next;
        let end = start.checked_add(size)?;
        if end > self.capacity {
            return None;
        }

        self.next = end;
        Some(start..end)
    }
}

fn align_up(address: usize, alignment: usize) -> Option<usize> {
    assert!(alignment.is_power_of_two());
    let mask = alignment - 1;
    address.checked_add(mask).map(|value| value & !mask)
}

struct Bump<'a> {
    start: NonNull<u8>,
    capacity: usize,
    next: usize,
    _memory: PhantomData<&'a mut [MaybeUninit<u8>]>,
}

impl<'a> Bump<'a> {
    fn new(memory: &'a mut [MaybeUninit<u8>]) -> Self {
        let start =
            NonNull::new(memory.as_mut_ptr().cast::<u8>()).expect("slice pointers are never null");

        Self {
            start,
            capacity: memory.len(),
            next: 0,
            _memory: PhantomData,
        }
    }

    fn allocate(&mut self, layout: Layout) -> Option<NonNull<u8>> {
        if layout.size() == 0 {
            return None;
        }

        let base = self.start.as_ptr() as usize;
        let current = base.checked_add(self.next)?;
        let aligned = align_up(current, layout.align())?;
        let start_offset = aligned.checked_sub(base)?;
        let end = start_offset.checked_add(layout.size())?;
        if end > self.capacity {
            return None;
        }

        // start_offset..end has been proven to be within the borrowed buffer.
        let pointer = unsafe { self.start.as_ptr().add(start_offset) };
        self.next = end;
        NonNull::new(pointer)
    }

    fn used(&self) -> usize {
        self.next
    }

    fn capacity(&self) -> usize {
        self.capacity
    }

    /// Every previously returned pointer must be dead before this call.
    unsafe fn reset(&mut self) {
        self.next = 0;
    }
}

fn main() {
    let mut cursor = Cursor::new(8);
    assert_eq!(cursor.allocate(3), Some(0..3));

    let mut memory = [MaybeUninit::<u8>::uninit(); 64];
    let mut bump = Bump::new(&mut memory);
    let slot = bump
        .allocate(Layout::new::<u32>())
        .expect("enough room")
        .cast::<u32>();

    // The allocation is live, aligned, large enough, and initialized by write.
    unsafe {
        slot.as_ptr().write(42);
        println!("stored: {}", slot.as_ptr().read());
    }

    println!("used {} of {} bytes", bump.used(), bump.capacity());
    // `slot` is not used again, so all returned pointers are now dead.
    unsafe { bump.reset() };
    assert_eq!(bump.used(), 0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_solution_handles_boundaries() {
        assert_eq!(align_up(5, 4), Some(8));
        assert_eq!(align_up(usize::MAX, 8), None);

        let mut memory = [MaybeUninit::<u8>::uninit(); 8];
        let mut bump = Bump::new(&mut memory);
        assert!(
            bump.allocate(Layout::from_size_align(8, 1).unwrap())
                .is_some()
        );
        assert!(bump.allocate(Layout::new::<u8>()).is_none());
        assert_eq!(bump.used(), 8);
    }
}
