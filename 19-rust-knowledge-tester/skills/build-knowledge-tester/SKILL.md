---
name: build-knowledge-tester
description: Create or adapt programming-language knowledge-testing applications in any implementation language and framework. Use when Codex must build a desktop, web, or mobile quiz that loads a question bank from JSON, randomly selects questions, supports single- and multiple-correct answers, scores exact answer sets, reports pass/fail results, or generalizes the Rust/Dioxus knowledge tester by replacing the subject language, implementation language, framework, or target platform.
---

# Build Knowledge Tester

Build a complete, runnable knowledge-testing application from a subject and a
chosen implementation stack. Preserve the product behavior while adapting code,
project structure, UI patterns, build tools, and packaging to the selected
framework.

## Establish the specification

Determine these values from the request and repository before writing code:

- `subject`: programming language or technical topic being tested
- `implementation_language`: language used to build the application
- `framework`: UI/application framework
- `target_platforms`: desktop OS, browser, mobile platform, or combination
- `bank_size`: default 70
- `quiz_size`: default 15
- `pass_percent`: default 80
- `application_name`: default `<Subject> Knowledge Tester`

Ask only when a missing value materially changes the implementation and cannot
be inferred. Treat the subject and implementation language as independent:
for example, a Rust quiz may be implemented in C# with Avalonia.

Use `assets/tester-spec.template.json` as a working specification when the task
is large or will be handed between agents. Do not copy it into the output unless
it benefits the project.

## Select and verify the stack

Inspect an existing repository before selecting versions or structure. For a new
project, choose the current stable toolchain and framework version compatible
with the requested targets. Consult current official framework documentation
because initialization, state, asset, and packaging APIs change frequently.

Follow the chosen framework's conventions instead of translating syntax
mechanically from Dioxus. Keep these responsibilities separate:

- executable entry point and platform initialization
- application/root component
- quiz session state and transitions
- question, result, and error/loading screens
- question loading, validation, random selection, and scoring
- static assets and question data

Keep the executable entry point minimal: instantiate or launch the application;
put application logic in modules, components, services, or packages.

## Implement the product contract

Read `references/product-contract.md` before implementation. Apply its behavior,
data, scoring, UI, reliability, and testing requirements unless the user
explicitly overrides them.

Create the question bank as a standalone JSON file matching
`assets/question-bank.schema.json`. During development, read the editable file
at startup. For packaged native applications, also embed or bundle a fallback so
the installed application does not depend on the source tree.

Generate factually precise questions. Include both single-answer and
multiple-answer entries. Avoid trick wording and ensure every listed correct
answer is defensible. For topics that can change with language or framework
versions, verify against official documentation and state the targeted version
in project documentation.

## Validate the question bank

Run:

```console
python scripts/validate_question_bank.py <path-to-questions.json> \
  --minimum <bank-size> --quiz-size <quiz-size>
```

Resolve every reported issue. If Python is unavailable, reproduce the same
checks in the project's test framework:

- minimum question count and unique IDs
- non-empty prompts, options, answers, and explanations
- unique answer choices per question
- unique, zero-based, in-range correct-answer indexes
- presence of both single- and multiple-answer questions
- bank large enough to select the requested unique quiz size

## Verify the application

Add unit tests for loading, bank validation, deterministic seeded selection,
unique random selection, exact-set scoring, empty totals, and the pass boundary.
Add UI/component tests when supported without fragile setup.

Before delivery:

1. Format and lint with the implementation stack's standard tools.
2. Build and run all automated tests.
3. Build for each explicitly requested target when the local toolchain supports it.
4. Launch or serve the application and smoke-test a complete attempt when safe.
5. Confirm single-answer controls, multiple-answer controls, navigation,
   submission, pass/fail text, result review, and restart behavior.
6. Document development commands, required runtimes, packaging commands,
   question schema, source layout, and unsupported/unverified targets.

Do not claim a platform or installer was verified when only source compilation
was performed.

