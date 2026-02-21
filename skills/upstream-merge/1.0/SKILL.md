# upstream-merge v1.0

## Purpose

Help forks ingest upstream fixes without losing local proof extensions.

## Use when

- upstream changed core primitives
- local fork carries custom harnesses/skills

## Expected output

- conflict-resolved patch set
- explicit note for each conflict: upstream kept / local kept / merged

## Required checks

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
./scripts/kani.sh
```
