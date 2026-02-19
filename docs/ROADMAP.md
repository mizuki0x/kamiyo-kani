# Roadmap

## Phase 1 (complete)

- Solana invariant primitives (`risk`, `math`, `bounds`, `token`, `staking`)
- Agent invariants (`lamport`, `reentrancy`, `PDA`, replay, FSM)
- `AccountInfo` symbolic helpers aligned with Kani issue #4550

## Phase 2 (complete)

- `cpi_contract!` macro for requires/body/ensures style CPI modeling
- Explicit auto-assert policy helpers:
  - `assert_timelock_release_policy`
  - `assert_oracle_consensus`
  - `assert_fsm_transition_guard`

## Phase 3 (next)

- More real-world fail -> fix proof examples
- Expanded benchmark matrix per harness category
- Artifact-level comparators to detect proof regressions in CI
