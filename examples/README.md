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
