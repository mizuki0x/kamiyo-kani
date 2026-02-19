# Replay/Idempotency (Fixed)

This crate enforces idempotency semantics:

- duplicate `event_id` + identical payload => accepted
- duplicate `event_id` + conflicting payload => rejected

## Run the passing harnesses

```bash
cargo kani --manifest-path examples/replay-idempotency-fixed/Cargo.toml \
  --harness proofs::fixed_rejects_conflicting_duplicate_event_id

cargo kani --manifest-path examples/replay-idempotency-fixed/Cargo.toml \
  --harness proofs::fixed_accepts_identical_duplicate_event_id

cargo kani --manifest-path examples/replay-idempotency-fixed/Cargo.toml \
  --harness proofs::fixed_accepts_new_event_id
```
