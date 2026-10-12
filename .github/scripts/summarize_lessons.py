#!/usr/bin/env python3
"""Build a lesson x OS markdown table from results.tsv artifacts.

Each results directory contains results.tsv with lines:
    lesson<TAB>pass|fail<TAB>runner
Directories are named like lesson-results-ubuntu-latest (the OS is the
suffix after the first 'lesson-results-').
"""

from __future__ import annotations

import argparse
import sys
from pathlib import Path


OS_ORDER = ["ubuntu-latest", "macos-latest", "windows-latest"]


def load(root: Path) -> tuple[list[str], list[str], dict[tuple[str, str], str], set[str]]:
    os_names: list[str] = []
    lessons: list[str] = []
    cells: dict[tuple[str, str], str] = {}
    runners: set[str] = set()
    files = sorted(root.rglob("results.tsv")) if root.is_dir() else []
    for path in files:
        # Artifact folder: .../lesson-results-<os>/results.tsv
        folder = path.parent.name
        prefix = "lesson-results-"
        os_name = folder[len(prefix) :] if folder.startswith(prefix) else folder
        if os_name not in os_names:
            os_names.append(os_name)
        for line in path.read_text(encoding="utf-8").splitlines():
            if not line.strip():
                continue
            parts = line.split("\t")
            if len(parts) < 2:
                continue
            lesson, status = parts[0], parts[1]
            runner = parts[2] if len(parts) > 2 else ""
            if lesson not in lessons:
                lessons.append(lesson)
            cells[(lesson, os_name)] = status
            if runner:
                runners.add(runner)
    lessons.sort()
    os_names.sort(key=lambda name: OS_ORDER.index(name) if name in OS_ORDER else len(OS_ORDER))
    return os_names, lessons, cells, runners


def markdown(
    os_names: list[str],
    lessons: list[str],
    cells: dict[tuple[str, str], str],
    *,
    selection: str,
    fail_workflow: bool,
) -> tuple[str, bool]:
    any_failed = any(status != "pass" for status in cells.values())
    if fail_workflow:
        mode = (
            f"Explicit selection (`{selection}`). "
            "This workflow fails if any selected lesson failed."
        )
    else:
        mode = (
            "Report mode. Unimplemented lessons show as failed and do not fail this workflow. "
            "Run this workflow manually with a lesson selection to fail the run when those lessons fail."
        )
    lines = [
        "## Lesson results",
        "",
        mode,
        "",
    ]
    if not lessons:
        lines.append("No lesson files were selected.")
        lines.append("")
        return "\n".join(lines), False

    header = "| Lesson | " + " | ".join(os_names) + " |"
    sep = "| --- | " + " | ".join("---" for _ in os_names) + " |"
    lines.extend([header, sep])
    passed = 0
    failed = 0
    for lesson in lessons:
        row = [f"`{lesson}`"]
        for os_name in os_names:
            status = cells.get((lesson, os_name), "missing")
            row.append(status)
            if status == "pass":
                passed += 1
            elif status == "fail":
                failed += 1
        lines.append("| " + " | ".join(row) + " |")
    lines.append("")
    lines.append(f"**{passed} passed, {failed} failed** across {len(os_names)} OS(es).")
    lines.append("")
    if any_failed and fail_workflow:
        lines.append("Failing the workflow because at least one explicitly selected lesson failed.")
        lines.append("")
    return "\n".join(lines), any_failed


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("results_root", type=Path)
    parser.add_argument("--selection", default="")
    parser.add_argument("--fail-workflow", choices=("true", "false"), default="false")
    parser.add_argument("--summary", type=Path, help="Also append the table to this file")
    parser.add_argument(
        "--expect-results",
        action="store_true",
        help="Exit with an error when no results.tsv files are present",
    )
    args = parser.parse_args()
    os_names, lessons, cells, runners = load(args.results_root)
    if args.expect_results and not lessons:
        print("Expected lesson results, but none were found.", file=sys.stderr)
        raise SystemExit(1)
    text, any_failed = markdown(
        os_names,
        lessons,
        cells,
        selection=args.selection,
        fail_workflow=args.fail_workflow == "true",
    )
    if runners:
        text += "Runner: " + ", ".join(sorted(runners)) + ".\n"
    sys.stdout.write(text)
    if args.summary:
        with args.summary.open("a", encoding="utf-8") as handle:
            handle.write(text)
            if not text.endswith("\n"):
                handle.write("\n")
    if args.fail_workflow == "true" and any_failed:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
