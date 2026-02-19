# FSM Transition Guard (Vulnerable)

This crate intentionally permits invalid state-machine transitions.

## Run failing harness

```bash
cargo kani --manifest-path examples/fsm-transition-guard-vulnerable/Cargo.toml \
  --harness proofs::vulnerable_allows_skip_to_terminal
```

Expected: Kani finds a counterexample and the harness fails.
