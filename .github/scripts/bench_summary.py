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
    print("Criterion for the written chapters. Informational only: no fail threshold.")
    print("Each run also writes `target/criterion-summary.json` for the trend chart.")
    print()
    custom: list[dict[str, object]] = []
    recorded: list[dict[str, object]] = []
    if not estimates:
        print("No Criterion estimates were written.")
    else:
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
            custom.append({"name": name, "unit": "ns", "value": mean_ns})
            recorded.append({"name": name, "mean_ns": mean_ns})
    if custom:
        target = Path("target")
        target.mkdir(exist_ok=True)
        (target / "criterion-summary.json").write_text(
            json.dumps(custom, indent=2) + "\n", encoding="utf-8"
        )
        commit = __import__("os").environ.get("GITHUB_SHA", "")
        (target / "criterion-run.json").write_text(
            json.dumps({"commit": commit, "benches": recorded}, indent=2) + "\n",
            encoding="utf-8",
        )


if __name__ == "__main__":
    try:
        main()
    except BrokenPipeError:
        sys.exit(0)
