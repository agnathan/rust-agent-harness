#!/usr/bin/env bash
# Per-lesson cargo test report for the starter workspace.
# Unimplemented lessons fail their tests; this script still exits 0.
set -u

root=$(cd "$(dirname "$0")/../.." && pwd)
cd "$root/starter"

summary=${GITHUB_STEP_SUMMARY:-}
pass=0
fail=0
rows=""

for path in harness/tests/l*.rs; do
  name=$(basename "$path" .rs)
  echo "::group::${name}"
  if cargo test --locked --test "$name" -- --color=never; then
    status=pass
    pass=$((pass + 1))
  else
    status=fail
    fail=$((fail + 1))
  fi
  echo "::endgroup::"
  rows="${rows}| \`${name}\` | ${status} |"$'\n'
done

report=$(
  cat <<EOF
## Starter lesson tests

Unimplemented lessons are expected to fail. This report does not fail the workflow.

| Lesson | Result |
| --- | --- |
${rows}
**${pass} passed, ${fail} failed.**
EOF
)

printf '%s\n' "$report"
if [[ -n "$summary" ]]; then
  printf '%s\n' "$report" >>"$summary"
fi
exit 0
