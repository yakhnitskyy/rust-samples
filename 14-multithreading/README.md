# 14 — Multithreading

Coordinate operating-system threads through ownership, messages, locks, atomics,
barriers, condition variables, and a small worker queue.

## Run

```console
cargo run
```

Examples 01–10 terminate deterministically. The final example explains a
deadlock pattern without deliberately creating one.

## Examples

1. Spawn and join
2. `move` capture
3. Scoped threads
4. Channels
5. Multiple producers
6. `Arc<Mutex<_>>` and `RwLock`
7. Atomics
8. Barrier and condition variable
9. Worker queue and panic handling
10. Deadlock avoidance

## Exercises

- Return structured results from several workers.
- Add graceful shutdown messages to the worker queue.
- Compare an atomic counter with a mutex-protected counter.

