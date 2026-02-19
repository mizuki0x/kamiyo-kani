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
