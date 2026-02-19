# Signer/Owner Authority (Vulnerable)

This crate intentionally allows state mutation without proper signer/owner checks.

## Run the failing harness

```bash
cargo kani --manifest-path examples/signer-owner-authority-vulnerable/Cargo.toml \
  --harness proofs::vulnerable_allows_unsigned_wrong_owner_authority
```

Expected: Kani finds a counterexample and the harness fails.
