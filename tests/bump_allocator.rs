use std::{alloc::Layout, mem::MaybeUninit};

use memory_allocator::{Bump, Cursor, align_up};

#[test]
fn checkpoint_1_cursor_reserves_ranges() {
    let mut cursor = Cursor::new(9);

    assert_eq!(cursor.allocate(3), Some(0..3));
    assert_eq!(cursor.allocate(6), Some(3..9));
    assert_eq!(cursor.allocate(1), None);
    assert_eq!(cursor.used(), 9, "failure must not move the cursor");
}

#[test]
fn checkpoint_1_cursor_rejects_overflow() {
    let mut cursor = Cursor::new(8);

    assert_eq!(cursor.allocate(1), Some(0..1));
    assert_eq!(cursor.allocate(usize::MAX), None);
    assert_eq!(cursor.used(), 1);
}

#[test]
fn checkpoint_2_aligns_addresses() {
    assert_eq!(align_up(5, 4), Some(8));
    assert_eq!(align_up(8, 4), Some(8));
    assert_eq!(align_up(17, 16), Some(32));
    assert_eq!(align_up(usize::MAX, 8), None);
}

#[test]
#[should_panic]
fn checkpoint_2_rejects_invalid_alignment() {
    let _ = align_up(5, 3);
}

#[test]
fn checkpoint_3_constructs_from_borrowed_memory() {
    let mut memory = [MaybeUninit::<u8>::uninit(); 24];
    let bump = Bump::new(&mut memory);

    assert_eq!(bump.used(), 0);
    assert_eq!(bump.capacity(), 24);
}

#[test]
fn checkpoint_4_reports_exhaustion_without_moving() {
    let mut memory = [MaybeUninit::<u8>::uninit(); 8];
    let mut bump = Bump::new(&mut memory);
    let all_bytes = Layout::from_size_align(8, 1).unwrap();

    assert!(bump.allocate(all_bytes).is_some());
    assert_eq!(bump.used(), 8);
    assert!(bump.allocate(Layout::new::<u8>()).is_none());
    assert_eq!(bump.used(), 8, "failure must not move the cursor");
}

#[test]
fn checkpoint_4_honors_alignment_and_does_not_overlap() {
    let mut memory = [MaybeUninit::<u8>::uninit(); 32];
    let mut bump = Bump::new(&mut memory);

    let first = bump.allocate(Layout::new::<u8>()).unwrap();
    let second = bump.allocate(Layout::new::<u64>()).unwrap();
    let first_address = first.as_ptr() as usize;
    let second_address = second.as_ptr() as usize;

    assert_eq!(second_address % align_of::<u64>(), 0);
    assert!(first_address + size_of::<u8>() <= second_address);
}

#[test]
fn checkpoint_4_rejects_zero_sized_requests() {
    let mut memory = [MaybeUninit::<u8>::uninit(); 8];
    let mut bump = Bump::new(&mut memory);

    assert!(bump.allocate(Layout::new::<()>()).is_none());
    assert_eq!(bump.used(), 0);
}

#[test]
fn checkpoint_5_reset_reclaims_the_arena() {
    let mut memory = [MaybeUninit::<u8>::uninit(); 16];
    let mut bump = Bump::new(&mut memory);

    bump.allocate(Layout::new::<u32>()).unwrap();
    assert!(bump.used() > 0);

    // No pointer returned above is retained or used after reset.
    unsafe { bump.reset() };
    assert_eq!(bump.used(), 0);
}
