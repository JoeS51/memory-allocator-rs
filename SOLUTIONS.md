# Progressive Solutions

Read only the checkpoint you are currently solving. The complete, executable
answer is in `examples/reference_solution.rs`.

## Checkpoint 1: Numeric cursor

Small hint: calculate `end` with `checked_add`; return before mutating on failure.

```rust
pub fn new(capacity: usize) -> Self {
    Self { capacity, next: 0 }
}

pub fn allocate(&mut self, size: usize) -> Option<Range<usize>> {
    let start = self.next;
    let end = start.checked_add(size)?;
    if end > self.capacity {
        return None;
    }

    self.next = end;
    Some(start..end)
}
```

## Checkpoint 2: Alignment

Small hint: power-of-two alignment lets a bit mask clear the low address bits.

```rust
pub fn align_up(address: usize, alignment: usize) -> Option<usize> {
    assert!(alignment.is_power_of_two());
    let mask = alignment - 1;
    address.checked_add(mask).map(|value| value & !mask)
}
```

## Checkpoint 3: Backing storage

Small hint: slice pointers are non-null even for empty slices.

```rust
pub fn new(memory: &'a mut [MaybeUninit<u8>]) -> Self {
    let start = NonNull::new(memory.as_mut_ptr().cast::<u8>())
        .expect("slice pointers are never null");

    Self {
        start,
        capacity: memory.len(),
        next: 0,
        _memory: PhantomData,
    }
}
```

## Checkpoint 4: Allocation

Small hint: do all arithmetic and validation first. Cursor mutation is the commit.

```rust
pub fn allocate(&mut self, layout: Layout) -> Option<NonNull<u8>> {
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

    // The checked range start_offset..end lies within the borrowed buffer.
    let pointer = unsafe { self.start.as_ptr().add(start_offset) };
    self.next = end;
    NonNull::new(pointer)
}
```

## Checkpoint 5: Reset

Small hint: resetting is cheap because no per-allocation metadata is maintained.

```rust
pub unsafe fn reset(&mut self) {
    self.next = 0;
}
```

## Comparison commands

Run the independent complete answer without changing your starter:

```sh
cargo test --example reference_solution
cargo run --example reference_solution
```

Compare your implementation after it passes:

```sh
diff -u src/lib.rs examples/reference_solution.rs
```

The files have different visibility and demonstration code, so the final diff is
not expected to be empty. Compare the algorithms and safety arguments.
