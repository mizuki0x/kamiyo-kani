#!/usr/bin/env bash
set -euo pipefail

pkg="kamiyo-kani"
features=()

if [[ "${KANI_FULL:-0}" == "1" ]]; then
  features+=("kani-full")
fi
if [[ "${KANI_AGENT:-0}" == "1" ]]; then
  features+=("solana-agent")
fi
if [[ "${KANI_ACCOUNT_INFO:-0}" == "1" ]]; then
  features+=("solana-account-info")
fi

if ((${#features[@]} > 0)); then
  cargo kani -p "$pkg" --features "$(IFS=,; echo "${features[*]}")"
else
  cargo kani -p "$pkg"
fi
