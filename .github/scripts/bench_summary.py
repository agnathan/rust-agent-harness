#!/usr/bin/env python3
"""Render Criterion estimates into a GitHub job summary. Never fails the job."""

from __future__ import annotations

import json
import sys
from pathlib import Path


def main() -> None:
    root = Path("target/criterion")
    estimates = sorted(root.glob("*/new/estimates.json")) if root.is_dir() else []
    print("## Benchmarks")
    print()
    print("Criterion for chapter 1 sections 1.1–1.3. Informational only: no fail threshold.")
    print()
    if not estimates:
        print("No Criterion estimates were written.")
        return
    print("| Benchmark | Mean |")
    print("|---|---|")
    for path in estimates:
        name = path.parts[-3]
        try:
            data = json.loads(path.read_text(encoding="utf-8"))
            mean_ns = float(data["mean"]["point_estimate"])
        except (OSError, KeyError, TypeError, ValueError, json.JSONDecodeError) as err:
            print(f"| `{name}` | unreadable ({err}) |")
            continue
        print(f"| `{name}` | {mean_ns / 1_000:.1f} µs |")


if __name__ == "__main__":
    try:
        main()
    except BrokenPipeError:
        sys.exit(0)
