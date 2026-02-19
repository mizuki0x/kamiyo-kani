# Replay/Idempotency (Vulnerable)

This crate intentionally accepts duplicate `event_id` values with conflicting payload.

## Run the failing harness

```bash
cargo kani --manifest-path examples/replay-idempotency-vulnerable/Cargo.toml \
  --harness proofs::vulnerable_accepts_conflicting_duplicate_event_id
```

Expected: Kani finds a counterexample and the harness fails.
