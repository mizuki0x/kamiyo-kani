# Escrow Release Policy (Fixed)

This example applies the policy used by `assert_timelock_release_policy`.

## Run the passing harnesses

```bash
cargo kani --manifest-path examples/escrow-release-policy-fixed/Cargo.toml \
  --harness proofs::fixed_blocks_oracle_before_expiry

cargo kani --manifest-path examples/escrow-release-policy-fixed/Cargo.toml \
  --harness proofs::fixed_allows_oracle_after_expiry
```
