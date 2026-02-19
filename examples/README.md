# Examples

## Fail -> Fix: Escrow Release Policy

- Vulnerable: `escrow-release-policy-vulnerable`
- Fixed: `escrow-release-policy-fixed`

Run sequence:

```bash
# 1) expect FAIL
cargo kani --manifest-path examples/escrow-release-policy-vulnerable/Cargo.toml \
  --harness proofs::vulnerable_allows_oracle_before_expiry

# 2) expect PASS
cargo kani --manifest-path examples/escrow-release-policy-fixed/Cargo.toml \
  --harness proofs::fixed_blocks_oracle_before_expiry

cargo kani --manifest-path examples/escrow-release-policy-fixed/Cargo.toml \
  --harness proofs::fixed_allows_oracle_after_expiry
```

## Fail -> Fix: CPI Allowlist

- Vulnerable: `cpi-allowlist-vulnerable`
- Fixed: `cpi-allowlist-fixed`

Run sequence:

```bash
# 1) expect FAIL
cargo kani --manifest-path examples/cpi-allowlist-vulnerable/Cargo.toml \
  --harness proofs::vulnerable_allows_unauthorized_program

# 2) expect PASS
cargo kani --manifest-path examples/cpi-allowlist-fixed/Cargo.toml \
  --harness proofs::fixed_rejects_unauthorized_program

cargo kani --manifest-path examples/cpi-allowlist-fixed/Cargo.toml \
  --harness proofs::fixed_allows_allowlisted_contract
```

## Fail -> Fix: PDA Seed/Bump Validation

- Vulnerable: `pda-seed-bump-vulnerable`
- Fixed: `pda-seed-bump-fixed`

Run sequence:

```bash
# 1) expect FAIL
cargo kani --manifest-path examples/pda-seed-bump-vulnerable/Cargo.toml \
  --harness proofs::vulnerable_accepts_invalid_shape

# 2) expect PASS
cargo kani --manifest-path examples/pda-seed-bump-fixed/Cargo.toml \
  --harness proofs::fixed_rejects_seed_count_overflow

cargo kani --manifest-path examples/pda-seed-bump-fixed/Cargo.toml \
  --harness proofs::fixed_rejects_seed_len_overflow

cargo kani --manifest-path examples/pda-seed-bump-fixed/Cargo.toml \
  --harness proofs::fixed_accepts_valid_shape_and_bump
```

## Fail -> Fix: Replay/Idempotency

- Vulnerable: `replay-idempotency-vulnerable`
- Fixed: `replay-idempotency-fixed`

Run sequence:

```bash
# 1) expect FAIL
cargo kani --manifest-path examples/replay-idempotency-vulnerable/Cargo.toml \
  --harness proofs::vulnerable_accepts_conflicting_duplicate_event_id

# 2) expect PASS
cargo kani --manifest-path examples/replay-idempotency-fixed/Cargo.toml \
  --harness proofs::fixed_rejects_conflicting_duplicate_event_id

cargo kani --manifest-path examples/replay-idempotency-fixed/Cargo.toml \
  --harness proofs::fixed_accepts_identical_duplicate_event_id

cargo kani --manifest-path examples/replay-idempotency-fixed/Cargo.toml \
  --harness proofs::fixed_accepts_new_event_id
```

## Fail -> Fix: Oracle Quorum/Median

- Vulnerable: `oracle-quorum-median-vulnerable`
- Fixed: `oracle-quorum-median-fixed`

Run sequence:

```bash
# 1) expect FAIL
cargo kani --manifest-path examples/oracle-quorum-median-vulnerable/Cargo.toml \
  --harness proofs::vulnerable_accepts_insufficient_reveals

cargo kani --manifest-path examples/oracle-quorum-median-vulnerable/Cargo.toml \
  --harness proofs::vulnerable_accepts_median_over_cap

# 2) expect PASS
cargo kani --manifest-path examples/oracle-quorum-median-fixed/Cargo.toml \
  --harness proofs::fixed_rejects_insufficient_reveals

cargo kani --manifest-path examples/oracle-quorum-median-fixed/Cargo.toml \
  --harness proofs::fixed_rejects_median_over_cap

cargo kani --manifest-path examples/oracle-quorum-median-fixed/Cargo.toml \
  --harness proofs::fixed_accepts_valid_consensus
```

## Fail -> Fix: Signer/Owner Authority

- Vulnerable: `signer-owner-authority-vulnerable`
- Fixed: `signer-owner-authority-fixed`

Run sequence:

```bash
# 1) expect FAIL
cargo kani --manifest-path examples/signer-owner-authority-vulnerable/Cargo.toml \
  --harness proofs::vulnerable_allows_unsigned_wrong_owner_authority

# 2) expect PASS
cargo kani --manifest-path examples/signer-owner-authority-fixed/Cargo.toml \
  --harness proofs::fixed_rejects_unsigned_authority

cargo kani --manifest-path examples/signer-owner-authority-fixed/Cargo.toml \
  --harness proofs::fixed_rejects_wrong_owner

cargo kani --manifest-path examples/signer-owner-authority-fixed/Cargo.toml \
  --harness proofs::fixed_accepts_signed_expected_owner
```

## Fail -> Fix: FSM Transition Guard

- Vulnerable: `fsm-transition-guard-vulnerable`
- Fixed: `fsm-transition-guard-fixed`

Run sequence:

```bash
# 1) expect FAIL
cargo kani --manifest-path examples/fsm-transition-guard-vulnerable/Cargo.toml \
  --harness proofs::vulnerable_allows_skip_to_terminal

# 2) expect PASS
cargo kani --manifest-path examples/fsm-transition-guard-fixed/Cargo.toml \
  --harness proofs::fixed_rejects_skip_to_terminal

cargo kani --manifest-path examples/fsm-transition-guard-fixed/Cargo.toml \
  --harness proofs::fixed_rejects_terminal_exit

cargo kani --manifest-path examples/fsm-transition-guard-fixed/Cargo.toml \
  --harness proofs::fixed_accepts_valid_progression
```

## End-to-End: Autonomous Payment Oracle (x402-style)

- Fixed: `autonomous-payment-oracle-fixed`

Run sequence:

```bash
cargo kani --manifest-path examples/autonomous-payment-oracle-fixed/Cargo.toml \
  --harness proofs::proof_autonomous_payment_oracle_flow
```

## End-to-End: x402 SVM Agentic Payments

- Fixed: `x402-svm-agent-payments-fixed`

Run sequence:

```bash
cargo kani --manifest-path examples/x402-svm-agent-payments-fixed/Cargo.toml \
  --harness proofs::proof_svm_x402_agentic_payment
```
