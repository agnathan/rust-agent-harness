#!/usr/bin/env python3
"""Resolve my-harness milestone test targets.

The plan is chapters 1-53 in .github/chapters.json. A spec may name any of
those chapters; a chapter with no tests/chNN.rs yet is an error.
Specs: all | 1 | 1-48 | 3,6,12
Changed paths: my-harness/tests/chNN.rs, and any my-harness/src change selects every chapter that has a test.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

CHAPTER_FILE = re.compile(r"(?:^|/)my-harness/tests/(ch(\d+)(?:_\d+)?)\.rs$")
STEM = re.compile(r"ch(\d+)(?:_\d+)?")
HARNESS_SRC = re.compile(r"(?:^|/)my-harness/(?:src/|Cargo\.toml$|README\.md$)")


def planned_chapters(root: Path) -> dict[int, str]:
    path = root / ".github" / "chapters.json"
    if not path.is_file():
        raise SystemExit(f"missing chapter plan: {path}")
    data = json.loads(path.read_text(encoding="utf-8"))
    planned: dict[int, str] = {}
    for item in data:
        number = int(item["n"])
        if number == 0:
            continue
        planned[number] = str(item["title"])
    return planned


def chapter_map(root: Path) -> dict[int, list[str]]:
    tests = root / "my-harness" / "tests"
    if not tests.is_dir():
        raise SystemExit(f"missing test directory: {tests}")
    mapping: dict[int, list[str]] = {}
    for path in sorted(tests.glob("ch*.rs")):
        match = STEM.fullmatch(path.stem)
        if not match:
            continue
        number = int(match.group(1))
        mapping.setdefault(number, []).append(path.stem)
    if not mapping:
        raise SystemExit(f"no chapter tests found in {tests}")
    return mapping


def parse_spec(spec: str, mapping: dict[int, list[str]], planned: dict[int, str]) -> list[str]:
    text = spec.strip()
    if not text:
        raise SystemExit("empty chapter spec")
    if text.lower() == "all":
        return [name for number in sorted(mapping) for name in mapping[number]]

    selected: list[str] = []
    seen: set[int] = set()
    known = ", ".join(str(number) for number in sorted(mapping))
    last = max(planned) if planned else 0
    span = f"1-{last}" if last else "1-48"
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
                f"invalid chapter spec '{part}'. Use 1, {span}, 3,6,12, or all."
            )
        for number in numbers:
            if number not in mapping:
                title = planned.get(number)
                if title:
                    raise SystemExit(
                        f"chapter {number} ({title}) has no milestone test yet "
                        f"(tests exist for: {known})"
                    )
                raise SystemExit(
                    f"no chapter {number} in the plan (chapters are {span}; tests exist for: {known})"
                )
            if number not in seen:
                seen.add(number)
                selected.extend(mapping[number])
    if not selected:
        raise SystemExit("no chapters selected")
    return selected


def from_paths(lines: list[str], mapping: dict[int, list[str]]) -> list[str]:
    by_name = {name: number for number, names in mapping.items() for name in names}
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
                for name in mapping[number]:
                    add(name)
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
    group.add_argument("--spec", help="Chapter spec: all, 1, 1-48, or 3,6,12")
    group.add_argument("--changed-from", type=argparse.FileType("r"))
    args = parser.parse_args()
    root = args.root.resolve()
    mapping = chapter_map(root)
    planned = planned_chapters(root)
    if args.spec is not None:
        names = parse_spec(args.spec, mapping, planned)
    else:
        names = from_paths(args.changed_from.readlines(), mapping)
    for name in names:
        print(name)


if __name__ == "__main__":
    try:
        main()
    except BrokenPipeError:
        sys.exit(0)
