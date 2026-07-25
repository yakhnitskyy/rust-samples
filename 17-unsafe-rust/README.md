# 17 — Unsafe Rust

Understand the small set of additional operations enabled by `unsafe`, and keep
their proof obligations behind narrow, documented, safe interfaces.

## Run

```console
cargo run
```

Examples 01–08 execute in order. Every unsafe operation has a nearby `SAFETY`
comment explaining its invariant.

## Examples

1. Unsafe blocks
2. Unsafe functions
3. Raw-pointer mutation
4. Safe wrapper around unsafe code
5. `NonNull`
6. Safe global state
7. `Send` and `Sync`
8. C-compatible interface

## Exercises

- Add bounds checks to another raw-pointer wrapper.
- Explain why two mutable slices returned by `split_at_mut` cannot overlap.
- Find a standard-library safe abstraction for each raw-pointer example.

