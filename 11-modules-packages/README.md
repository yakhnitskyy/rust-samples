# 11 — Modules, Packages, and Visibility

See how Rust organizes names inside a crate and across filesystem-backed modules.
This project intentionally uses extra source files because file modules are part
of the lesson.

## Run

```console
cargo run
```

Examples 01–07 execute in order.

## Examples

1. Inline modules
2. Nested modules
3. Public and private items
4. `use` and aliases
5. Re-exports
6. `super` and `crate`
7. Filesystem modules

## Exercises

- Add a second vegetable module.
- Re-export a useful garden type from the crate root.
- Make an item private and observe where access stops compiling.

