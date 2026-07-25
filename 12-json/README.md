# 12 — Working with JSON

Move between untyped JSON trees, strongly typed Rust data, formatted text, and
files using Serde.

## Run

```console
cargo run
```

Examples 01–09 execute offline. The file example uses and removes a file in the
operating system's temporary directory.

## Examples

1. Construct JSON
2. Parse into `Value`
3. Traverse safely
4. Deserialize typed data
5. Field attributes and defaults
6. Nested structures
7. Pretty serialization
8. Read and write a JSON file
9. Validate decoded data

## Exercises

- Add an enum represented as tagged JSON.
- Accept an unknown extra field and observe Serde's default behavior.
- Validate a list and report every invalid item.

