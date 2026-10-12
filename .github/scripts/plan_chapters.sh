#!/usr/bin/env bash
# Resolve the chapter list, OS matrix, and whether test failures fail the workflow.
set -eu

root=$(cd "$(dirname "$0")/../.." && pwd)
cd "$root"

if [[ -n "${GITHUB_OUTPUT:-}" ]]; then
  output_file=$GITHUB_OUTPUT
else
  output_file=$(mktemp)
  echo "Writing plan outputs to $output_file" >&2
fi

event_name=${EVENT_NAME:-}
chapters_input=${CHAPTERS_INPUT:-all}
os_input=${OS_INPUT:-all}
before=${BEFORE:-}
base_sha=${BASE_SHA:-}

case "$event_name" in
  workflow_dispatch) fail_workflow=true ;;
  *) fail_workflow=false ;;
esac

if [[ -z "$os_input" ]]; then
  os_input=all
fi
case "$os_input" in
  all) oses='["ubuntu-latest","macos-latest","windows-latest"]' ;;
  ubuntu) oses='["ubuntu-latest"]' ;;
  macos) oses='["macos-latest"]' ;;
  windows) oses='["windows-latest"]' ;;
  *)
    echo "unknown os '$os_input' (expected all, ubuntu, macos, or windows)" >&2
    exit 1
    ;;
esac

tmp=$(mktemp)
if [[ "$event_name" == "workflow_dispatch" || -z "$event_name" ]]; then
  spec=${chapters_input:-all}
  selection=$spec
  python3 "$root/.github/scripts/select_chapters.py" --root "$root" --spec "$spec" >"$tmp"
else
  changed=$(mktemp)
  if [[ "$event_name" == "pull_request" ]]; then
    if [[ -z "$base_sha" ]]; then
      echo "pull_request is missing a base sha" >&2
      exit 1
    fi
    git diff --name-only "$base_sha" HEAD >"$changed"
    selection="changed files"
  else
    if [[ -z "$before" || "$before" =~ ^0+$ ]]; then
      git diff-tree --no-commit-id --name-only -r HEAD >"$changed" || true
    else
      git diff --name-only "$before" "${GITHUB_SHA:-HEAD}" >"$changed"
    fi
    selection="changed files"
  fi
  python3 "$root/.github/scripts/select_chapters.py" --root "$root" --changed-from "$changed" >"$tmp"
  rm -f "$changed"
fi

{
  echo "fail_workflow=$fail_workflow"
  echo "oses=$oses"
  echo "selection<<SEL"
  echo "$selection"
  echo "SEL"
  if [[ -s "$tmp" ]]; then
    echo "has_chapters=true"
    echo "chapters<<EOF"
    cat "$tmp"
    echo "EOF"
  else
    echo "has_chapters=false"
    echo "chapters<<EOF"
    echo "EOF"
  fi
} >>"$output_file"

echo "Selected chapters:"
if [[ -s "$tmp" ]]; then
  cat "$tmp"
else
  echo "(none)"
fi
echo "fail_workflow=$fail_workflow"
echo "oses=$oses"
rm -f "$tmp"
