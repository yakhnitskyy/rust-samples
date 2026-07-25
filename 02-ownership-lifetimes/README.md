# 02 — Ownership and Lifetimes

Follow values through moves, copies, clones, borrows, slices, and increasingly
explicit lifetime relationships.

## Run

```console
cargo run
```

Examples 01–09 execute in order. Comments also point out operations that the
borrow checker intentionally prevents.

## Examples

1. Stack and heap values
2. Moves
3. `Copy` and `Clone`
4. Shared borrowing
5. Mutable borrowing
6. Slices
7. Explicit and elided lifetimes
8. Struct and multiple lifetimes
9. `'static` data

## Exercises

- Write `last_word(&str) -> &str`.
- Add a function that selects the shorter of two string slices.
- Try to return a reference to a local `String` and study the compiler message.

