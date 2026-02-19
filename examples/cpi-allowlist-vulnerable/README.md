# CPI Allowlist (Vulnerable)

This crate intentionally contains a CPI allowlist bug.

## Run the failing harness

```bash
cargo kani --manifest-path examples/cpi-allowlist-vulnerable/Cargo.toml \
  --harness proofs::vulnerable_allows_unauthorized_program
```

Expected: Kani finds a counterexample and the harness fails.
