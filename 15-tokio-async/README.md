# 15 — Tokio and Async Rust

Run cooperative asynchronous tasks with Tokio and coordinate them through time,
channels, locks, semaphores, cancellation, and bounded concurrency.

## Run

```console
cargo run
```

Examples 01–10 run offline and terminate deterministically.

## Examples

1. Async functions and `.await`
2. Spawned tasks
3. `join!`
4. `select!` and timeout
5. Async channels
6. Shared state
7. Semaphores
8. Cancellation
9. Blocking work and task errors
10. Bounded concurrent work

## Exercises

- Build a three-stage channel pipeline.
- Add a timeout around each bounded work item.
- Compare a Tokio mutex with an owned-state actor task.

