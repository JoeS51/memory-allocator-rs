use std::{alloc::Layout, mem::MaybeUninit};

use memory_allocator::Bump;

fn main() {
    let mut memory = [MaybeUninit::<u8>::uninit(); 64];
    let mut bump = Bump::new(&mut memory);
    let slot = bump
        .allocate(Layout::new::<u32>())
        .expect("the arena has room for a u32")
        .cast::<u32>();

    // The allocator proved alignment and bounds; write initializes the slot
    // before read, and the backing memory remains alive for both operations.
    unsafe {
        slot.as_ptr().write(42);
        println!("stored: {}", slot.as_ptr().read());
    }

    println!("used {} of {} bytes", bump.used(), bump.capacity());
}
