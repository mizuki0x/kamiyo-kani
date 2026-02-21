# add-replay-safety-proof v1.0

## Purpose

Generate replay/idempotency proofs for request/event processing paths.

## Use when

- integrating x402-like request IDs
- proving duplicate semantics in settlement paths

## Expected output

- replay helper + proof updates in agent/replay areas or examples
- clear invariant: identical duplicates accepted, conflicting duplicates rejected

## Required checks

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo kani -p kamiyo-kani --features solana-agent --tests
```
