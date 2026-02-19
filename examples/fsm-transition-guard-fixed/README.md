# FSM Transition Guard (Fixed)

This crate enforces explicit state-machine transition guards.

## Run passing harnesses

```bash
cargo kani --manifest-path examples/fsm-transition-guard-fixed/Cargo.toml \
  --harness proofs::fixed_rejects_skip_to_terminal

cargo kani --manifest-path examples/fsm-transition-guard-fixed/Cargo.toml \
  --harness proofs::fixed_rejects_terminal_exit

cargo kani --manifest-path examples/fsm-transition-guard-fixed/Cargo.toml \
  --harness proofs::fixed_accepts_valid_progression
```
