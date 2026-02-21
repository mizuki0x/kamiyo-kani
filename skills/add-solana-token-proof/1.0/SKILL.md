# add-solana-token-proof v1.0

## Purpose

Add token-specific proofs for common Solana safety invariants (authority checks, value conservation, mint/burn bounds).

## Use when

- a fork needs token invariants beyond generic split math
- adding SPL-style authority and accounting constraints

## Expected output

- new or updated proof helpers in `token`-related modules
- at least one runnable example harness for the target invariant class

## Required checks

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
./scripts/kani.sh
```
