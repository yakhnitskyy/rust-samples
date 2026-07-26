#!/usr/bin/env python3
"""Validate a language-neutral knowledge-tester JSON question bank."""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import Any


REQUIRED_FIELDS = {
    "id",
    "prompt",
    "options",
    "correct_answers",
    "explanation",
}


def non_empty_text(value: Any) -> bool:
    return isinstance(value, str) and bool(value.strip())


def validate_bank(bank: Any, minimum: int, quiz_size: int) -> list[str]:
    errors: list[str] = []
    if not isinstance(bank, list):
        return ["top-level JSON value must be an array"]
    if len(bank) < minimum:
        errors.append(f"bank has {len(bank)} questions; minimum is {minimum}")
    if len(bank) < quiz_size:
        errors.append(
            f"bank has {len(bank)} questions but quiz size is {quiz_size}"
        )

    seen_ids: set[int] = set()
    has_single = False
    has_multiple = False

    for position, question in enumerate(bank, start=1):
        prefix = f"question at position {position}"
        if not isinstance(question, dict):
            errors.append(f"{prefix} must be an object")
            continue

        missing = REQUIRED_FIELDS - question.keys()
        extra = question.keys() - REQUIRED_FIELDS
        if missing:
            errors.append(f"{prefix} is missing fields: {sorted(missing)}")
        if extra:
            errors.append(f"{prefix} has unsupported fields: {sorted(extra)}")

        question_id = question.get("id")
        if not isinstance(question_id, int) or isinstance(question_id, bool):
            errors.append(f"{prefix} id must be an integer")
        elif question_id < 1:
            errors.append(f"{prefix} id must be positive")
        elif question_id in seen_ids:
            errors.append(f"{prefix} repeats id {question_id}")
        else:
            seen_ids.add(question_id)
            prefix = f"question {question_id}"

        if not non_empty_text(question.get("prompt")):
            errors.append(f"{prefix} prompt must be non-empty text")
        if not non_empty_text(question.get("explanation")):
            errors.append(f"{prefix} explanation must be non-empty text")

        options = question.get("options")
        if not isinstance(options, list) or len(options) < 2:
            errors.append(f"{prefix} options must contain at least two values")
            options = []
        else:
            if any(not non_empty_text(option) for option in options):
                errors.append(f"{prefix} options must all be non-empty strings")
            normalized = [
                option.strip() if isinstance(option, str) else option
                for option in options
            ]
            if len({json.dumps(value, sort_keys=True) for value in normalized}) != len(
                normalized
            ):
                errors.append(f"{prefix} contains duplicate answer choices")

        answers = question.get("correct_answers")
        if not isinstance(answers, list) or not answers:
            errors.append(f"{prefix} correct_answers must be a non-empty array")
            continue
        if any(not isinstance(answer, int) or isinstance(answer, bool) for answer in answers):
            errors.append(f"{prefix} correct answer indexes must be integers")
            continue
        if len(set(answers)) != len(answers):
            errors.append(f"{prefix} repeats a correct answer index")
        if any(answer < 0 or answer >= len(options) for answer in answers):
            errors.append(f"{prefix} has an out-of-range correct answer index")

        has_single |= len(answers) == 1
        has_multiple |= len(answers) > 1

    if bank and not has_single:
        errors.append("bank has no single-answer questions")
    if bank and not has_multiple:
        errors.append("bank has no multiple-answer questions")
    return errors


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("question_file", type=Path)
    parser.add_argument("--minimum", type=int, default=70)
    parser.add_argument("--quiz-size", type=int, default=15)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    if args.minimum < 1 or args.quiz_size < 1:
        print("minimum and quiz size must be positive", file=sys.stderr)
        return 2

    try:
        bank = json.loads(args.question_file.read_text(encoding="utf-8"))
    except OSError as error:
        print(f"could not read {args.question_file}: {error}", file=sys.stderr)
        return 2
    except json.JSONDecodeError as error:
        print(f"invalid JSON in {args.question_file}: {error}", file=sys.stderr)
        return 1

    errors = validate_bank(bank, args.minimum, args.quiz_size)
    if errors:
        for error in errors:
            print(f"ERROR: {error}", file=sys.stderr)
        return 1

    single = sum(len(question["correct_answers"]) == 1 for question in bank)
    multiple = len(bank) - single
    print(
        f"OK: {len(bank)} questions "
        f"({single} single-answer, {multiple} multiple-answer); "
        f"quiz size {args.quiz_size}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

