# 19 — Rust Knowledge Tester

A native desktop quiz built with Dioxus. Each attempt randomly selects 15
distinct questions from the 70-question JSON bank. Questions may have one or
multiple correct answers. A response is correct only when its selected answers
exactly match the complete answer set.

The passing score is 80%, which means at least 12 of 15 questions.

## Run on Windows 11 x64

Requirements:

- 64-bit Rust MSVC toolchain
- Microsoft WebView2 Runtime (included with normal Windows 11 installations)

Run directly with Cargo:

```console
cargo run
```

For hot reload, install the Dioxus CLI and use:

```console
dx serve --desktop
```

## Build and bundle

Build a release executable:

```console
cargo build --release
```

To hide the terminal window in a packaged Windows build and create a desktop
bundle:

```console
dx bundle --desktop --release --features bundle
```

The `bundle` feature enables the Windows GUI subsystem only for packaged builds,
so normal development runs retain terminal diagnostics.

## Question bank

Questions live in `assets/questions.json`. During development, the program reads
that file at startup, so edits appear on the next launch. The same data is also
embedded at compile time, allowing packaged executables to work if the standalone
JSON file is unavailable.

Each question contains:

- A stable numeric ID
- The prompt
- Four answer choices
- Zero-based indexes of all correct choices
- An explanation shown after submission

## Source layout

- `src/main.rs` only declares the application modules and starts the app.
- `src/app.rs` configures the desktop window and root component.
- `src/session.rs` owns quiz and application state.
- `src/ui/` contains separate question, results, and error screens.
- `src/lib.rs` contains reusable question loading, validation, selection, and
  scoring logic.

## Behavior

- A new attempt shuffles the bank and takes 15 unique questions.
- Single-answer questions use radio controls.
- Multiple-answer questions use checkboxes.
- Previous/next navigation preserves selections.
- The final screen shows the score, pass/fail message, and answer review.
- “Try another set” creates a fresh random attempt.

## Tests

```console
cargo test
```

Tests validate the JSON bank, unique IDs, answer indexes, single/multiple-answer
coverage, deterministic random selection, exact-match scoring, and the 80% pass
boundary.

## Reusable generator skill

`skills/build-knowledge-tester` contains a Codex skill for generating this type
of application for any subject language, implementation language, framework,
and target platform. Invoke it by name and provide its repository path, for
example:

```text
Use $build-knowledge-tester from ./skills/build-knowledge-tester to create
a Python knowledge tester written in C# with Avalonia for Windows.
```
