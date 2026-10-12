#!/usr/bin/env bash
# Run every cargo package that belongs to one step directory.
# Usage: test_step.sh <step-dir-name>
set -euo pipefail

step=${1:?step directory name}
root=$(cd "$(dirname "$0")/../.." && pwd)
cd "$root"

found=0
while IFS= read -r package; do
  package=${package//$'\r'/}
  if [[ -z "$package" ]]; then
    continue
  fi
  found=1
  echo "::group::${package}"
  cargo test -p "$package" --locked
  echo "::endgroup::"
done < <(python3 "$root/.github/scripts/list_steps.py" --root "$root" --packages "$step")

if [[ "$found" -ne 1 ]]; then
  echo "step ${step} has no packages" >&2
  exit 1
fi
