# Bump Allocator in Rust

An offline learning project for implementing an arena-style bump allocator from
first principles. It uses only Rust's standard library, so there are no external
dependencies to download.

## Before the flight

Run these commands while you still have internet access:

```sh
rustc --version
cargo --version
cargo fetch --locked
cargo test --example reference_solution
cargo run --example reference_solution
cargo test --offline --example reference_solution
```

The final command proves that the toolchain and this project work without a
network connection. `cargo fetch` has nothing external to fetch today, but is a
useful safeguard if dependencies are added later.

## During the flight

1. Open [`lesson.html`](lesson.html) in a browser for the visual lesson.
2. Keep [`WORKBOOK.md`](WORKBOOK.md) next to the editor for commands and prompts.
3. Implement the `todo!()` calls in [`src/lib.rs`](src/lib.rs), in order.
4. Run one checkpoint at a time, for example `cargo test checkpoint_1`.
5. Open [`SOLUTIONS.md`](SOLUTIONS.md) only when a hint is not enough.

When all checkpoints pass, run:

```sh
cargo test
cargo run
```

The starter intentionally panics at `todo!()` until you implement it. The known
good complete version remains independently runnable with
`cargo run --example reference_solution`.
