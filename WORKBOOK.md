# Bump Allocator Workbook

## Contract

The allocator borrows one fixed byte buffer. Each successful allocation returns
the next suitably aligned range and advances a cursor. Failed allocations leave
the cursor unchanged. Individual allocations cannot be freed; `reset` reclaims
everything at once.

Central invariant:

```text
0 <= next <= capacity
bytes before next: reserved or alignment padding
bytes at/after next: available
```

## Working rhythm

For each checkpoint: predict on paper, implement, run its focused tests, then
explain why the invariant still holds. Give yourself ten minutes before opening
the matching section of `SOLUTIONS.md`.

### 1. Numeric cursor

Implement `Cursor::new` and `Cursor::allocate` in `src/lib.rs`.

Questions:

- What range is returned when `next == 3` and `size == 6`?
- Why must you calculate `end` before changing `next`?
- What happens when `next + size` exceeds `usize::MAX`?

```sh
cargo test checkpoint_1
```

### 2. Alignment arithmetic

Implement `align_up`. Alignment is a non-zero power of two. Derive the formula
with an address of 5 and alignment of 4 before coding.

```text
mask = alignment - 1
aligned = (address + mask) & !mask
```

```sh
cargo test checkpoint_2
```

### 3. Borrow the arena

Implement `Bump::new`. Record the buffer's address and length without claiming
that any typed value already exists in it. Explain what each of these contributes:
`MaybeUninit<u8>`, `NonNull<u8>`, and `PhantomData<&mut [...]>`.

```sh
cargo test checkpoint_3
```

### 4. Allocate raw storage

Implement `Bump::allocate` in this order:

1. Reject a zero-sized layout.
2. Compute the actual address at the cursor.
3. Align that address, not just the cursor offset.
4. Convert the aligned address back to an offset.
5. Check the full allocation, including padding, against capacity.
6. Create the pointer and only then commit `next`.

Before writing the `unsafe` block, state why pointer addition remains within the
borrowed allocation.

```sh
cargo test checkpoint_4
```

### 5. Reset and use values

Implement `Bump::reset`, then make `cargo run` store and print `42`. Explain why
every old pointer must be dead before reset and why reset cannot run destructors.

```sh
cargo test checkpoint_5
cargo test
cargo run
```

## Debugging map

| Symptom | Check |
| --- | --- |
| Alignment test fails | Align `base + next`, not `next` alone. |
| Used bytes change after failure | Assign `self.next` only after every check. |
| Allocation crosses the buffer | Include alignment padding in the end offset. |
| Arithmetic panics or wraps | Use `checked_add` and `checked_sub`. |
| Raw pointer code feels unjustified | Calculate and bounds-check integer offsets first. |
| Buffer cannot be accessed directly | `Bump` still holds its exclusive borrow. |

## Final self-check

You understand the exercise when you can answer these without opening the code:

1. Why does the backing storage use `MaybeUninit`?
2. Why does alignment apply to an address rather than an offset?
3. Why is committing the cursor the final step?
4. Which fact makes the pointer `add` valid?
5. What is invalidated by `reset`?
6. What simplicity does a bump allocator gain by not freeing individual blocks?
