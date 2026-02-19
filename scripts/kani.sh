#!/usr/bin/env bash
set -euo pipefail

pkg="kamiyo-kani"
features=()
extra_args=()

if [[ "${KANI_FULL:-0}" == "1" ]]; then
  features+=("kani-full")
fi
if [[ "${KANI_AGENT:-0}" == "1" ]]; then
  features+=("solana-agent")
fi
if [[ "${KANI_ACCOUNT_INFO:-0}" == "1" ]]; then
  features+=("solana-account-info")
fi

if [[ -n "${KANI_HARNESS:-}" ]]; then
  extra_args+=(--harness "${KANI_HARNESS}")
fi

if [[ "${KANI_TESTS:-0}" == "1" ]]; then
  extra_args+=(--tests)
fi

if ((${#features[@]} > 0)); then
  cargo kani -p "$pkg" --features "$(IFS=,; echo "${features[*]}")" "${extra_args[@]}"
else
  cargo kani -p "$pkg" "${extra_args[@]}"
fi
