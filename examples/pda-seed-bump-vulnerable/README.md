# PDA Seed/Bump Validation (Vulnerable)

This crate intentionally accepts invalid PDA seed shapes.

## Run the failing harness

```bash
cargo kani --manifest-path examples/pda-seed-bump-vulnerable/Cargo.toml \
  --harness proofs::vulnerable_accepts_invalid_shape
```

Expected: Kani finds a counterexample and the harness fails.
