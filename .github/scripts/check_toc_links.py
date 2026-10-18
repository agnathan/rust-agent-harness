#!/usr/bin/env python3
"""Fail if TABLE_OF_CONTENTS.md links to a path that is not in the repo.

Written chapters must use the same files as book/SUMMARY.md. A summary
entry titled "Chapter N" must link to a file named chapter-NN-*.md.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

LINK = re.compile(r"\[[^\]]*\]\(([^)]+)\)")
CHAPTER_TITLE = re.compile(r"^Chapter (\d+):")


def markdown_targets(text: str) -> list[str]:
    found: list[str] = []
    for raw in LINK.findall(text):
        target = raw.strip()
        if not target or target.startswith(("#", "http://", "https://", "mailto:")):
            continue
        path = target.split("#", 1)[0].strip()
        if path:
            found.append(path)
    return found


def check(root: Path) -> list[str]:
    """Return human-readable problems. An empty list means the links are sound."""
    toc_path = root / "TABLE_OF_CONTENTS.md"
    summary_path = root / "book" / "SUMMARY.md"
    errors: list[str] = []
    if not toc_path.is_file():
        return ["TABLE_OF_CONTENTS.md is missing"]
    if not summary_path.is_file():
        return ["book/SUMMARY.md is missing"]

    toc = toc_path.read_text(encoding="utf-8")
    for target in markdown_targets(toc):
        rel = Path(target)
        if rel.is_absolute() or ".." in rel.parts:
            errors.append(f"{target} is not a relative path inside the repo")
            continue
        if not (root / rel).exists():
            errors.append(f"missing: {rel.as_posix()}")

    summary = summary_path.read_text(encoding="utf-8")
    for line in summary.splitlines():
        match = re.match(r"- \[(.+?)\]\(([^)]*)\)", line)
        if not match:
            continue
        title, href = match.group(1), match.group(2).strip()
        if not href or href.startswith(("#", "http://", "https://")):
            continue
        href = href.split("#", 1)[0].strip()
        if not href:
            continue
        numbered = CHAPTER_TITLE.match(title)
        if numbered:
            prefix = f"chapter-{int(numbered.group(1)):02d}-"
            if not Path(href).name.startswith(prefix):
                errors.append(
                    f"SUMMARY.md links {title} to {href}; expected {prefix}….md"
                )
        book_path = f"book/{href}"
        if book_path not in toc:
            errors.append(f"TABLE_OF_CONTENTS.md does not link {book_path} ({title})")
        if not (root / "book" / href).exists():
            errors.append(f"missing: {book_path}")
    return errors


def main() -> None:
    root = Path(__file__).resolve().parents[2]
    errors = check(root)
    if errors:
        print("TABLE_OF_CONTENTS.md link check failed:", file=sys.stderr)
        for error in errors:
            print(f"  {error}", file=sys.stderr)
        raise SystemExit(1)
    count = len(markdown_targets((root / "TABLE_OF_CONTENTS.md").read_text(encoding="utf-8")))
    print(f"checked {count} links in TABLE_OF_CONTENTS.md")


if __name__ == "__main__":
    main()
