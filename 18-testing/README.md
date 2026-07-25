# 18 — Unit and Integration Testing

This project is both a runnable demonstration and a real testable library. It
intentionally uses `src/lib.rs` and `tests/` because Cargo integration tests must
be separate crates.

## Run

```console
cargo run
cargo test
cargo test addition
cargo test -- --ignored
cargo test --doc
```

## Examples

1. Testable public API
2. Basic assertions
3. Helpful assertion messages
4. `Result`-returning tests
5. `should_panic`
6. Unit and private-function tests
7. Integration tests and shared setup
8. Filtering, ignored tests, and documentation tests

## Expected behavior

`cargo run` explains each category. `cargo test` runs unit, integration, and
documentation tests; the deliberately slow ignored test runs only when requested.

## Exercises

- Add table-driven cases for `slugify`.
- Add an integration test for overflow-aware arithmetic.
- Temporarily break an assertion and study Cargo's failure output.

