#!/usr/bin/env python3
"""Resolve lesson test-target names.

Specs: all | 7 | 1-5 | 3,8,12
Changed paths: starter/harness/src/lNN_*.rs and tests/lNN_*.rs
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

LESSON_FILE = re.compile(r"(?:^|/)(?:src|tests)/(l(\d+)_[A-Za-z0-9_]+)\.rs$")


def lesson_map(root: Path) -> dict[int, str]:
    tests = root / "starter" / "harness" / "tests"
    if not tests.is_dir():
        raise SystemExit(f"missing test directory: {tests}")
    mapping: dict[int, str] = {}
    for path in sorted(tests.glob("l*.rs")):
        match = re.fullmatch(r"l(\d+)_[A-Za-z0-9_]+", path.stem)
        if not match:
            continue
        number = int(match.group(1))
        if number in mapping:
            raise SystemExit(f"two tests share lesson number {number}: {mapping[number]} and {path.stem}")
        mapping[number] = path.stem
    if not mapping:
        raise SystemExit(f"no lesson tests found in {tests}")
    return mapping


def parse_spec(spec: str, mapping: dict[int, str]) -> list[str]:
    text = spec.strip()
    if not text:
        raise SystemExit("empty lesson spec")
    if text.lower() == "all":
        return [mapping[number] for number in sorted(mapping)]

    selected: list[str] = []
    seen: set[int] = set()
    known = ", ".join(str(number) for number in sorted(mapping))
    for part in text.split(","):
        part = part.strip()
        if not part:
            continue
        range_match = re.fullmatch(r"(\d+)\s*-\s*(\d+)", part)
        if range_match:
            start, end = int(range_match.group(1)), int(range_match.group(2))
            if start > end:
                raise SystemExit(f"invalid range '{part}': start is greater than end")
            numbers = range(start, end + 1)
        elif re.fullmatch(r"\d+", part):
            numbers = [int(part)]
        else:
            raise SystemExit(
                f"invalid lesson spec '{part}'. Use 7, 1-5, 3,8,12, or all."
            )
        for number in numbers:
            if number not in mapping:
                raise SystemExit(f"no lesson {number} (known lessons: {known})")
            if number not in seen:
                seen.add(number)
                selected.append(mapping[number])
    if not selected:
        raise SystemExit("no lessons selected")
    return selected


def from_paths(lines: list[str], mapping: dict[int, str]) -> list[str]:
    by_name = {name: number for number, name in mapping.items()}
    found: list[tuple[int, str]] = []
    seen: set[str] = set()
    for line in lines:
        path = line.strip().replace("\\", "/")
        match = LESSON_FILE.search(path)
        if not match:
            continue
        name = match.group(1)
        if name not in by_name or name in seen:
            continue
        seen.add(name)
        found.append((by_name[name], name))
    found.sort()
    return [name for _, name in found]


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path("."))
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("--spec", help="Lesson spec: all, 7, 1-5, or 3,8,12")
    group.add_argument(
        "--changed-from",
        type=argparse.FileType("r"),
        help="Newline-separated changed paths ('-' for stdin)",
    )
    args = parser.parse_args()
    root = args.root.resolve()
    mapping = lesson_map(root)
    if args.spec is not None:
        names = parse_spec(args.spec, mapping)
    else:
        names = from_paths(args.changed_from.readlines(), mapping)
    for name in names:
        print(name)


if __name__ == "__main__":
    try:
        main()
    except BrokenPipeError:
        sys.exit(0)
