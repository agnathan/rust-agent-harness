#!/usr/bin/env bash
# Run each my-harness chapter named on stdin (one test target per line).
# Always exits 0. Writes results under $RESULTS_DIR.
set -u

root=$(cd "$(dirname "$0")/../.." && pwd)
results_dir=${RESULTS_DIR:-"$root/chapter-results"}
mkdir -p "$results_dir/junit"
results_dir=$(cd "$results_dir" && pwd)
cd "$root"

use_nextest=0
if command -v cargo-nextest >/dev/null 2>&1; then
  use_nextest=1
fi

any_failed=false
: >"$results_dir/results.tsv"

write_fallback_junit() {
  chapter=$1
  status=$2
  log_file=$3
  out="$results_dir/junit/${chapter}.xml"
  body=$(sed 's/]]>/]]]]><![CDATA[>/g' "$log_file" | tail -c 20000)
  if [[ "$status" == "pass" ]]; then
    failures=0
    failure_xml=""
  else
    failures=1
    failure_xml="<failure message=\"chapter tests failed\"><![CDATA[${body}]]></failure>"
  fi
  cat >"$out" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<testsuite name="${chapter}" tests="1" failures="${failures}">
  <testcase classname="${chapter}" name="${chapter}">${failure_xml}</testcase>
</testsuite>
EOF
}

while IFS= read -r chapter || [[ -n "${chapter:-}" ]]; do
  chapter=$(printf '%s' "$chapter" | tr -d '\r' | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')
  if [[ -z "$chapter" ]]; then
    continue
  fi
  log=$(mktemp)
  runner=cargo-test
  rc=1
  if [[ "$use_nextest" -eq 1 ]]; then
    runner=nextest
    rm -f target/nextest/ci/junit.xml
    cargo nextest run --locked --profile ci -p my-harness --test "$chapter" >"$log" 2>&1
    rc=$?
    junit_src="target/nextest/ci/junit.xml"
    if [[ -f "$junit_src" ]]; then
      cp "$junit_src" "$results_dir/junit/${chapter}.xml"
    else
      write_fallback_junit "$chapter" "$([ "$rc" -eq 0 ] && echo pass || echo fail)" "$log"
    fi
  else
    echo "cargo-nextest is not installed; falling back to cargo test for ${chapter}" | tee "$log"
    cargo test --locked -p my-harness --test "$chapter" -- --color=never >>"$log" 2>&1
    rc=$?
    if [[ "$rc" -eq 0 ]]; then
      write_fallback_junit "$chapter" pass "$log"
    else
      write_fallback_junit "$chapter" fail "$log"
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
  printf '%s\t%s\t%s\n' "$chapter" "$status" "$runner" >>"$results_dir/results.tsv"
  echo "chapter ${chapter}: ${status} (${runner})"
done

printf '%s\n' "$any_failed" >"$results_dir/any_failed"
echo "any_failed=${any_failed}"
if [[ -n "${GITHUB_OUTPUT:-}" ]]; then
  echo "any_failed=${any_failed}" >>"$GITHUB_OUTPUT"
fi
exit 0
