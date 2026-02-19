#!/usr/bin/env bash
set -euo pipefail

pkg="kamiyo-kani"
features=()
extra_args=()

if [[ "${KANI_FULL:-0}" == "1" ]]; then
  features+=("kani-full")
fi
if [[ "${KANI_STRESS:-0}" == "1" ]]; then
  features+=("kani-stress")
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

# Keep default CI fast and stable. Deeper checks are available via KANI_FULL=1,
# with the heaviest harnesses behind KANI_STRESS=1.
if [[ "${KANI_FULL:-0}" == "0" && "${KANI_TESTS:-0}" == "0" ]]; then
  default_harnesses=(
    risk::proofs::proof_principal_protection_across_accounts
    risk::proofs::proof_haircut_ratio_basic_properties
    risk::proofs::proof_haircut_ratio_is_one_when_residual_covers_profit
    risk::proofs::proof_fee_sweep_conservation
    risk::proofs::proof_fee_sweep_clears_when_sufficient
    risk::proofs::proof_funding_zero_rate_no_payment
    risk::proofs::proof_funding_zero_denominator_safe
    risk::proofs::proof_funding_zero_position_no_payment
    risk::proofs::proof_writeoff_conservation
    risk::proofs::proof_writeoff_insurance_monotonic_decrease
  )

  if [[ "${KANI_AGENT:-0}" == "1" ]]; then
    default_harnesses+=(agent::bench::verify_agent_flow_end_to_end)
  fi
  if [[ "${KANI_ACCOUNT_INFO:-0}" == "1" ]]; then
    default_harnesses+=(
      account_info::proofs::proof_account_config_flags_are_applied
      account_info::proofs::proof_account_lamports_range_is_enforced
      account_info::proofs::proof_lamport_snapshot_tracks_mutations
    )
  fi

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
