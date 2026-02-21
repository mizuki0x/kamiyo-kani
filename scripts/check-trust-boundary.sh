#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "${script_dir}/.." && pwd)"
cd "${repo_root}"

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo is required" >&2
  exit 1
fi

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required" >&2
  exit 1
fi

metadata_file="$(mktemp)"
trap 'rm -f "${metadata_file}"' EXIT

cargo metadata --format-version 1 --no-deps > "${metadata_file}"

git_dep_violations="$(
  jq -r '
    . as $m
    | [
        .packages[]
        | select(.id as $id | $m.workspace_members | index($id))
        | .name as $pkg
        | .dependencies[]
        | select((.source // "") | startswith("git+"))
        | "workspace package \($pkg) depends on git source \(.name): \(.source)"
      ]
    | .[]
  ' "${metadata_file}"
)"

path_dep_violations="$(
  jq -r '
    . as $m
    | [
        .packages[]
        | select(.id as $id | $m.workspace_members | index($id))
        | .name as $pkg
        | .dependencies[]
        | select(.path != null)
        | "workspace package \($pkg) has path dependency \(.name): \(.path)"
      ]
    | .[]
  ' "${metadata_file}"
)"

lockfile_git_sources="$(grep -nE 'source = "git\+' Cargo.lock || true)"

if [[ -n "${git_dep_violations}" || -n "${path_dep_violations}" || -n "${lockfile_git_sources}" ]]; then
  echo "trust-boundary check failed" >&2
  if [[ -n "${git_dep_violations}" ]]; then
    echo "" >&2
    echo "Git dependency violations:" >&2
    echo "${git_dep_violations}" >&2
  fi
  if [[ -n "${path_dep_violations}" ]]; then
    echo "" >&2
    echo "Path dependency violations:" >&2
    echo "${path_dep_violations}" >&2
  fi
  if [[ -n "${lockfile_git_sources}" ]]; then
    echo "" >&2
    echo "Cargo.lock git sources:" >&2
    echo "${lockfile_git_sources}" >&2
  fi
  exit 1
fi

echo "trust-boundary check passed"
