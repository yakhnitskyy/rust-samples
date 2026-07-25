# 13 — Basic HTTP Calls

Make synchronous HTTP requests with a reusable Reqwest client and handle status,
JSON, timeout, and connection failures without panicking.

## Run

```console
cargo run
```

This project requires internet access. It calls
[httpbin](https://httpbin.org/) and
[JSONPlaceholder](https://jsonplaceholder.typicode.com/guide/). Individual
failures are printed and the remaining examples continue. JSONPlaceholder writes
are simulated and do not modify persistent data.

## Examples

1. Basic GET
2. Query parameters
3. Request headers
4. HTTP status handling
5. Dynamic JSON
6. Typed JSON response
7. POST JSON
8. Reusable client and timeout
9. Connection-error handling

## Exercises

- Add a custom user-agent header to the reusable client.
- Decode a list of posts into `Vec<Post>`.
- Retry only timeout errors with a small attempt limit.

