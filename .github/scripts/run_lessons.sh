#!/usr/bin/env bash
# Run each lesson named on stdin (one test target per line).
# Always exits 0. Writes:
#   $RESULTS_DIR/results.tsv    lesson<TAB>pass|fail<TAB>nextest|cargo-test
#   $RESULTS_DIR/junit/<lesson>.xml
#   $RESULTS_DIR/any_failed     true|false
# Set RESULTS_DIR (default: lesson-results). Run from anywhere; the repo root
# is the parent of .github/.
set -u

root=$(cd "$(dirname "$0")/../.." && pwd)
results_dir=${RESULTS_DIR:-"$root/lesson-results"}
mkdir -p "$results_dir/junit"
results_dir=$(cd "$results_dir" && pwd)
cd "$root/starter"

use_nextest=0
if command -v cargo-nextest >/dev/null 2>&1; then
  use_nextest=1
fi

any_failed=false
: >"$results_dir/results.tsv"

write_fallback_junit() {
  lesson=$1
  status=$2
  log_file=$3
  out="$results_dir/junit/${lesson}.xml"
  # Keep the report well-formed if the log contains the CDATA terminator.
  body=$(sed 's/]]>/]]]]><![CDATA[>/g' "$log_file" | tail -c 20000)
  if [[ "$status" == "pass" ]]; then
    failures=0
    failure_xml=""
  else
    failures=1
    failure_xml="<failure message=\"lesson tests failed\"><![CDATA[${body}]]></failure>"
  fi
  cat >"$out" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<testsuite name="${lesson}" tests="1" failures="${failures}">
  <testcase classname="${lesson}" name="${lesson}">${failure_xml}</testcase>
</testsuite>
EOF
}

while IFS= read -r lesson || [[ -n "${lesson:-}" ]]; do
  lesson=$(printf '%s' "$lesson" | tr -d '\r' | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')
  if [[ -z "$lesson" ]]; then
    continue
  fi
  log=$(mktemp)
  runner=cargo-test
  rc=1
  if [[ "$use_nextest" -eq 1 ]]; then
    runner=nextest
    rm -f target/nextest/ci/junit.xml
    cargo nextest run --locked --profile ci --test "$lesson" >"$log" 2>&1
    rc=$?
    junit_src="target/nextest/ci/junit.xml"
    if [[ -f "$junit_src" ]]; then
      cp "$junit_src" "$results_dir/junit/${lesson}.xml"
    else
      write_fallback_junit "$lesson" "$([ "$rc" -eq 0 ] && echo pass || echo fail)" "$log"
    fi
  else
    echo "cargo-nextest is not installed; falling back to cargo test for ${lesson}" | tee "$log"
    cargo test --locked --test "$lesson" -- --color=never >>"$log" 2>&1
    rc=$?
    if [[ "$rc" -eq 0 ]]; then
      write_fallback_junit "$lesson" pass "$log"
    else
      write_fallback_junit "$lesson" fail "$log"
    fi
  fi
  cat "$log"
  rm -f "$log"
  if [[ "$rc" -eq 0 ]]; then
    status=pass
  else
    status=fail
    any_failed=true
  fi
  printf '%s\t%s\t%s\n' "$lesson" "$status" "$runner" >>"$results_dir/results.tsv"
  echo "lesson ${lesson}: ${status} (${runner})"
done

printf '%s\n' "$any_failed" >"$results_dir/any_failed"
echo "any_failed=${any_failed}"
if [[ -n "${GITHUB_OUTPUT:-}" ]]; then
  echo "any_failed=${any_failed}" >>"$GITHUB_OUTPUT"
fi
exit 0
