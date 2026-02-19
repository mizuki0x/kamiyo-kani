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

run_kani() {
  if ((${#features[@]} > 0)); then
    cargo kani -p "$pkg" --features "$(IFS=,; echo "${features[*]}")" "$@"
  else
    cargo kani -p "$pkg" "$@"
  fi
}

if [[ -n "${KANI_HARNESS:-}" ]]; then
  if ((${#extra_args[@]} > 0)); then
    run_kani "${extra_args[@]}"
  else
    run_kani
  fi
  exit 0
fi

# Keep default CI fast and stable. Deeper checks are available via KANI_FULL=1.
if [[ "${KANI_FULL:-0}" == "0" && "${KANI_TESTS:-0}" == "0" && "${KANI_AGENT:-0}" == "0" && "${KANI_ACCOUNT_INFO:-0}" == "0" ]]; then
  default_harnesses=(
    risk::proofs::proof_haircut_ratio_basic_properties
    risk::proofs::proof_effective_pnl_matches_reference_u64_domain
    risk::proofs::proof_warmup_monotonic_in_elapsed
    risk::proofs::proof_fee_sweep_conservation
    risk::proofs::proof_writeoff_conservation
  )

  for harness in "${default_harnesses[@]}"; do
    run_kani --harness "$harness"
  done
  exit 0
fi

if ((${#extra_args[@]} > 0)); then
  run_kani "${extra_args[@]}"
else
  run_kani
fi
