# solana-account-generator v1.0

## Purpose

Generate or refine symbolic Solana `AccountInfo` generators and matching Kani harnesses.

## Use when

- adding new account-shape constraints
- proving mutability/signer/owner safety properties
- extending RFC #4550-style account modeling

## Expected output

- code changes in generator + harness files
- deterministic assumptions (`kani::assume`) that avoid unconstrained blow-ups
- at least one proof harness that fails before fix and passes after fix (or a clear explanation when no bug exists)

## Required checks

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo kani -p kamiyo-kani --features solana-account-info --tests
```
