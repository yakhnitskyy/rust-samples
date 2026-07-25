# 10 — Smart Pointers

Use pointer-like types to express heap ownership, shared ownership, interior
mutability, non-owning links, thread-safe sharing, copy-on-write, and pinning.

## Run

```console
cargo run
```

Examples 01–09 execute in order.

## Examples

1. `Box` and recursive data
2. `Deref`
3. `Drop`
4. `Rc`
5. `RefCell`
6. `Weak` links
7. `Arc`
8. `Cow`
9. Introductory `Pin`

## Exercises

- Add a method that calculates the recursive list length.
- Build a tree with weak parent and strong child links.
- Use `Cow` to normalize text only when necessary.

