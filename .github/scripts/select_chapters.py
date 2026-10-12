#!/usr/bin/env python3
"""Resolve my-harness milestone test targets.

Specs: all | 1 | 1-2 | 1,2
Changed paths: my-harness/tests/chNN.rs, and any my-harness/src change selects every chapter.
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

CHAPTER_FILE = re.compile(r"(?:^|/)my-harness/tests/(ch(\d+))\.rs$")
HARNESS_SRC = re.compile(r"(?:^|/)my-harness/(?:src/|Cargo\.toml$|README\.md$)")


def chapter_map(root: Path) -> dict[int, str]:
    tests = root / "my-harness" / "tests"
    if not tests.is_dir():
        raise SystemExit(f"missing test directory: {tests}")
    mapping: dict[int, str] = {}
    for path in sorted(tests.glob("ch*.rs")):
        match = re.fullmatch(r"ch(\d+)", path.stem)
        if not match:
            continue
        number = int(match.group(1))
        if number in mapping:
            raise SystemExit(f"two tests share chapter {number}")
        mapping[number] = path.stem
    if not mapping:
        raise SystemExit(f"no chapter tests found in {tests}")
    return mapping


def parse_spec(spec: str, mapping: dict[int, str]) -> list[str]:
    text = spec.strip()
    if not text:
        raise SystemExit("empty chapter spec")
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
            raise SystemExit(f"invalid chapter spec '{part}'. Use 1, 1-2, 1,2, or all.")
        for number in numbers:
            if number not in mapping:
                raise SystemExit(f"no chapter {number} (known chapters: {known})")
            if number not in seen:
                seen.add(number)
                selected.append(mapping[number])
    if not selected:
        raise SystemExit("no chapters selected")
    return selected


def from_paths(lines: list[str], mapping: dict[int, str]) -> list[str]:
    by_name = {name: number for number, name in mapping.items()}
    found: list[tuple[int, str]] = []
    seen: set[str] = set()

    def add(name: str) -> None:
        if name not in by_name or name in seen:
            return
        seen.add(name)
        found.append((by_name[name], name))

    for line in lines:
        path = line.strip().replace("\\", "/")
        if HARNESS_SRC.search(path):
            for number in sorted(mapping):
                add(mapping[number])
            continue
        match = CHAPTER_FILE.search(path)
        if match:
            add(match.group(1))
    found.sort()
    return [name for _, name in found]


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path("."))
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("--spec", help="Chapter spec: all, 1, 1-2, or 1,2")
    group.add_argument("--changed-from", type=argparse.FileType("r"))
    args = parser.parse_args()
    mapping = chapter_map(args.root.resolve())
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
