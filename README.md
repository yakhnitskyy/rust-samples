# Rust by Example Workspace

This workspace is a numbered path from Rust fundamentals to advanced topics. Each
folder is an independent Cargo project. Open a folder in VS Code, read its
`README.md`, and run:

```console
cargo run
```

Every binary prints numbered headings and executes its examples from simple to
advanced. Project 13 requires network access; all other examples run offline.

## Learning path

1. `01-basics`
2. `02-ownership-lifetimes`
3. `03-strings`
4. `04-arrays-slices-vectors`
5. `05-structs-enums-patterns`
6. `06-collections`
7. `07-generics-traits`
8. `08-closures-iterators`
9. `09-error-handling`
10. `10-smart-pointers`
11. `11-modules-packages`
12. `12-json`
13. `13-http-calls`
14. `14-multithreading`
15. `15-tokio-async`
16. `16-macros`
17. `17-unsafe-rust`
18. `18-testing`

## Workspace commands

```console
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets
cargo fmt --all --check
```

The source deliberately favors clarity and explanatory comments over abstraction.
Try each README exercise by changing or extending the numbered functions.
