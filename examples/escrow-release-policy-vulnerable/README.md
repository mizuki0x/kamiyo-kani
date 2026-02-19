# Escrow Release Policy (Vulnerable)

This example intentionally encodes a release-policy bug.

## Run the failing harness

```bash
cargo kani --manifest-path examples/escrow-release-policy-vulnerable/Cargo.toml \
  --harness proofs::vulnerable_allows_oracle_before_expiry
```

Expected: Kani finds a counterexample and the harness fails.
