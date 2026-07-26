# Knowledge Tester Product Contract

## Contents

1. Configuration
2. Question data
3. Quiz lifecycle
4. Scoring
5. User interface
6. Architecture and reliability
7. Testing and acceptance

## 1. Configuration

Keep these values centralized and easy to change:

| Setting | Default |
|---|---:|
| Question-bank size | 70 |
| Questions per attempt | 15 |
| Passing percentage | 80 |
| Answer indexing | Zero-based |

Derive displayed requirements from configuration. For 15 questions at 80%, the
minimum passing score is 12; do not hardcode 12 in logic when quiz size or pass
percentage can change.

## 2. Question data

Store questions in JSON independently of UI code. Each entry has:

- stable unique integer `id`
- non-empty `prompt`
- at least two unique string `options`
- one or more unique `correct_answers` indexes
- non-empty `explanation`

Infer answer mode from `correct_answers.length`: one index means single-answer;
more than one means multiple-answer.

Validate the entire bank before beginning an attempt. Show a recoverable,
human-readable loading error instead of crashing.

## 3. Quiz lifecycle

Use this state progression:

```text
load and validate bank
        ↓
shuffle/select unique questions
        ↓
answer ↔ navigate while preserving selections
        ↓
submit final question
        ↓
score and review
        ↓
restart with a newly randomized set
```

Select exactly the configured quiz size without replacement. A restart must
reshuffle the full bank; it may coincidentally include prior questions.

Preserve selections when navigating backward and forward. Disable forward or
submit actions until the current question has at least one selected choice,
unless the user requests unanswered submissions.

## 4. Scoring

Score each question as one point only when the selected indexes exactly equal
the correct indexes:

```text
selected set == correct set
```

For multiple-answer questions, a subset or superset is incorrect. Do not award
partial points unless explicitly requested.

Use integer-safe pass logic:

```text
score * 100 >= pass_percent * total_questions
```

On success, display exactly:

```text
Congratulations! You passed!
```

On failure, clearly state that the user failed and show the score and required
threshold.

## 5. User interface

The question screen must show:

- application title
- current position and total
- progress indicator
- single/multiple selection guidance
- one answer choice per full-width row
- Previous and Next/Submit controls
- answered-question count

Use native radio semantics for single-answer questions and checkboxes/toggles
for multiple-answer questions. Provide keyboard focus, visible selected states,
readable contrast, and labels that activate their controls.

Choose an initial desktop window size that fits a normal four-option question
and navigation without vertical scrolling on the requested baseline display.
Keep the window resizable and responsive for smaller screens. Result review may
scroll because it contains all attempted questions.

The result screen must show:

- required pass or fail message
- numeric score and percentage
- per-question correct/incorrect state
- user's selected answer
- correct answer when the response was wrong
- explanation
- restart action

## 6. Architecture and reliability

Separate entry point, platform configuration, state, UI screens, and domain
logic. Keep loading, validation, selection, scoring, and pass calculation
independent of the UI framework so they can be unit-tested.

During development, load the editable JSON question file. For packaged desktop
or mobile applications, bundle or embed the bank as a fallback. Web applications
may fetch a bundled static asset but must handle fetch and parse failures.

Use a seeded random generator in tests and a normal entropy-backed generator in
the application.

## 7. Testing and acceptance

Test at minimum:

- expected bank size and unique IDs
- invalid/malformed JSON
- invalid answer indexes and duplicate indexes
- single- and multiple-answer coverage
- selection count and uniqueness
- deterministic seeded selection
- exact matching for single and multiple answers
- missing, subset, and superset answers
- pass threshold immediately below, at, and above the boundary
- zero-question percentage behavior

Acceptance requires:

- project builds with the documented command
- tests and linters pass
- a complete quiz can be taken
- exactly the configured number of unique questions appears
- choices persist across navigation
- scoring and messaging match the contract
- restart creates a fresh randomized attempt

