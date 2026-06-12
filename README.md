# kamiyo-kani

[![CI](https://github.com/mizuki0x/kamiyo-kani/actions/workflows/ci.yml/badge.svg?branch=main&event=push)](https://github.com/mizuki0x/kamiyo-kani/actions/workflows/ci.yml)
[![Kani](https://github.com/mizuki0x/kamiyo-kani/actions/workflows/kani.yml/badge.svg?branch=main&event=push)](https://github.com/mizuki0x/kamiyo-kani/actions/workflows/kani.yml)
[![Docs](https://github.com/mizuki0x/kamiyo-kani/actions/workflows/docs.yml/badge.svg?branch=main&event=push)](https://github.com/mizuki0x/kamiyo-kani/actions/workflows/docs.yml)
[![Benchmarks](https://github.com/mizuki0x/kamiyo-kani/actions/workflows/benchmark.yml/badge.svg?branch=main&event=push)](https://github.com/mizuki0x/kamiyo-kani/actions/workflows/benchmark.yml)
[![Audit](https://github.com/mizuki0x/kamiyo-kani/actions/workflows/audit.yml/badge.svg?branch=main&event=push)](https://github.com/mizuki0x/kamiyo-kani/actions/workflows/audit.yml)
[![Crates.io](https://img.shields.io/crates/v/kamiyo-kani.svg)](https://crates.io/crates/kamiyo-kani)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

![kani](https://github.com/user-attachments/assets/e7b69412-9d22-4384-80d8-90daa9cf2a6d)

Reusable [Kani](https://model-checking.github.io/kani/) verification primitives and proof harnesses for Solana protocol math. The crate packages checked arithmetic for haircut ratios, effective PnL, warmup schedules, fee sweeps, funding payments and loss writeoffs, plus assertion helpers for value-conserving splits, staking multipliers, bounded outputs, CPI authorization and state machine guards. Proofs run in CI through small profile scripts, so a team gets machine-checked invariants without standing up a formal methods stack.

## Features

- **Checked protocol math.** The `risk` module implements Percolator-style primitives: `haircut_ratio`, `effective_pnl`, `warmup_slope`, `fee_debt_sweep`, `funding_payment` and `loss_writeoff`. Each function ships with proofs of conservation, bounds and monotonicity.
- **Reusable proof helpers.** `token`, `staking`, `bounds` and `math` expose assert-style helpers that wrap your own closures: split conservation, monotonic staking multipliers, bounded outputs, spec equivalence.
- **Constrained generators.** `generators` produces bounded symbolic values for common Solana domains: scores, weights, basis points, lamports, ordered timestamps.
- **Agent invariants.** The `solana-agent` feature adds symbolic agent accounts, lamport conservation, reentrancy checks, CPI allowlists via `cpi_stub!` and `cpi_contract!`, PDA seed limits, oracle consensus checks and FSM transition guards.
- **Symbolic `AccountInfo`.** The `solana-account-info` feature generates real `solana_program::AccountInfo` values from configurable constraints, with `LamportSnapshot` for conservation checks.
- **Zero runtime cost.** Everything is gated behind `cfg(kani)`. Normal builds compile the crate to nothing.
- **Fail to fix examples.** Seven vulnerable and fixed example pairs plus two end-to-end flows, each with runnable Kani commands.

Design notes:

- generic verification gaps go upstream to Kani
- Solana-focused layers live here
- proof-critical spec logic stays in-repo, no external spec-crate trust expansion

## Install

The crate is published on [crates.io](https://crates.io/crates/kamiyo-kani):

```toml
[dev-dependencies]
kamiyo-kani = "0.1.1"
```

Install Kani once:

```bash
cargo install --locked kani-verifier
cargo kani setup
```

## Quick start

```rust
#![cfg(kani)]

use kamiyo_kani::risk::{effective_pnl, haircut_ratio};

#[kani::proof]
fn payout_is_bounded_by_profit() {
    let vault: u128 = kani::any();
    let principal_total: u128 = kani::any();
    let insurance: u128 = kani::any();
    let pnl_pos_total: u128 = kani::any();
    let my_pnl: i128 = kani::any();

    let (h_num, h_den) = haircut_ratio(vault, principal_total, insurance, pnl_pos_total);
    let payout = effective_pnl(my_pnl, h_num, h_den);

    kani::assert(payout <= my_pnl.max(0) as u128, "payout bounded by positive pnl");
}
```

Run it:

```bash
cargo kani -p kamiyo-kani
```

## Proof coverage

All harnesses below are `#[kani::proof]` functions in this repository. Status says where each proof runs: the `Kani` workflow verifies the smoke and full profiles on every push and weekly, the `Benchmarks` workflow verifies the agent flow harness, and opt-in rows run via the documented commands.

### Core risk proofs (`src/risk.rs`, default features)

| Harness | Property proved | Status |
| --- | --- | --- |
| `proof_haircut_ratio_basic_properties` | Haircut ratio is well formed: nonzero denominator, `num <= den`, `h = 1` when no profitable accounts, numerator capped by vault residual | CI (smoke, full) |
| `proof_haircut_ratio_is_one_when_residual_covers_profit` | `h = 1` exactly when the residual covers total positive PnL | CI (smoke, full) |
| `proof_principal_protection_across_accounts` | A loss capped to one account's principal keeps that account and the total consistent | CI (smoke, full) |
| `proof_fee_sweep_conservation` | `swept + remaining == debt`, with `swept` bounded by balance and debt | CI (smoke, full) |
| `proof_fee_sweep_clears_when_sufficient` | When the balance covers the debt, the sweep clears it fully | CI (smoke, full) |
| `proof_funding_zero_rate_no_payment` | Zero funding rate pays nothing | CI (smoke, full) |
| `proof_funding_zero_denominator_safe` | Zero rate denominator returns 0 instead of dividing by zero | CI (smoke, full) |
| `proof_funding_zero_position_no_payment` | Zero position pays nothing | CI (smoke, full) |
| `proof_writeoff_conservation` | `writeoff + new_insurance == insurance`, writeoff capped by loss and insurance | CI (smoke, full) |
| `proof_writeoff_insurance_monotonic_decrease` | Larger losses never leave more insurance behind | CI (smoke, full) |

### Extended risk proofs (`src/risk.rs`, `kani-full` and `kani-stress`)

| Harness | Property proved | Status |
| --- | --- | --- |
| `proof_funding_long_short_symmetry_overflow_branch` | Long and short payments stay symmetric on the overflow saturation branch | CI (full) |
| `proof_profit_conversion_payout_formula` | Payout never exceeds raw positive PnL | stress profile |
| `proof_effective_pnl_matches_reference_wide_domain` | `effective_pnl` equals the reference formula `pos * num / den` over the u32 domain | stress profile |
| `proof_effective_pnl_matches_reference_smoke_domain` | Same reference equivalence over the u16 domain | stress profile |
| `proof_rounding_slack_bound_when_haircut_active` | Floored per-account payouts sum to at most the residual, rounding slack at most `N - 1` | stress profile |
| `proof_effective_pnl_bounded` | Effective PnL never exceeds `max(pnl, 0)` | stress profile |
| `proof_effective_equity_with_haircut` | Equity with haircut PnL never drops below capital | stress profile |
| `proof_warmup_monotonic_in_elapsed` | Warmed profit is monotonic in elapsed time | stress profile |
| `proof_warmup_bounded_by_gross` | Warmed profit never exceeds gross profit | stress profile |
| `proof_warmup_full_after_period` | The full amount unlocks once the warmup period elapses | stress profile |
| `proof_funding_long_short_symmetry` | Long pays exactly what short receives (u8 domain) | stress profile |
| `proof_funding_long_short_symmetry_wide_domain` | Same symmetry over the u16 domain without overflow | stress profile |

### Agent proofs (`solana-agent` feature)

| Harness | Property proved | Status |
| --- | --- | --- |
| `agent::bench::verify_agent_flow_end_to_end` | A payer to escrow to payee settle flow conserves lamports | CI (benchmarks) |
| `verify_payer_is_signer_and_writable` | Payer generator yields a signing, writable, rent-exempt account | opt-in (agent tests) |
| `verify_program_account_executable` | Program generator yields an executable program account | opt-in (agent tests) |
| `verify_data_len_aligned` | Generated account data length is 8 byte aligned | opt-in (agent tests) |
| `verify_initial_lamports_snapshot` | `initial_lamports` records the creation balance | opt-in (agent tests) |
| `verify_lamport_conservation_after_transfer` | A two account transfer conserves total lamports | opt-in (agent tests) |
| `verify_lamport_conservation_three_account_chain` | A chained three account transfer conserves total lamports | opt-in (agent tests) |
| `verify_no_reentrancy_clean` | Fresh accounts carry zero reentry depth and pass the reentrancy check | opt-in (agent tests) |
| `verify_cpi_authorized_subset` | A CPI log of allowlisted programs passes authorization | opt-in (agent tests) |
| `verify_state_machine_valid_transition` | Declared edges and self loops pass the transition check | opt-in (agent tests) |
| `verify_terminal_state_holds` | Terminal states cannot transition further | opt-in (agent tests) |
| `verify_pda_seed_limits` | Seed count and length validators accept Solana-valid shapes (16 seeds, 32 bytes) | opt-in (agent tests) |
| `verify_cpi_stub_records_call` | `cpi_stub!` records exactly one call with the declared program | opt-in (agent tests) |
| `verify_cpi_contract_records_call` | `cpi_contract!` records the call and enforces its body assertions | opt-in (agent tests) |
| `verify_cpi_contract_records_metadata` | `cpi_contract!` records `lamports_transferred` and `accounts_touched` metadata | opt-in (agent tests) |
| `verify_cpi_contract_auto_asserts` | `auto_asserts` blocks (timelock, oracle, FSM) hold inside a contract | opt-in (agent tests) |
| `verify_oracle_consensus_helper` | Consensus helper accepts quorum and cap valid inputs | opt-in (agent tests) |
| `verify_fsm_transition_guard` | FSM guard accepts valid progressions and terminal self loops | opt-in (agent tests) |

### `AccountInfo` proofs (`solana-account-info` feature)

| Harness | Property proved | Status |
| --- | --- | --- |
| `proof_account_config_flags_are_applied` | Generated `AccountInfo` carries the configured signer, writable, executable and rent epoch values | opt-in (account-info) |
| `proof_account_lamports_range_is_enforced` | Generated lamports stay inside the configured range | opt-in (account-info) |
| `proof_lamport_snapshot_tracks_mutations` | `LamportSnapshot` reports balanced transfers as unchanged and detects single-account mutation | opt-in (account-info) |
| `timelock_policy_matches_release_rule` | Before expiry only the agent signer can release, after expiry agent or api can | opt-in (account-info tests) |
| `release_funds_conserves_lamports` | A successful release moves exactly `amount` and conserves the pair total, a failed release mutates nothing | opt-in (account-info tests) |

### Self-verification proofs (`tests/self_verify.rs`, `kani-full`)

| Harness | Property proved | Status |
| --- | --- | --- |
| `any_score_is_bounded` | `any_score()` stays in `[0, 100]` | opt-in (self-verify) |
| `any_weight_is_positive_and_bounded` | `any_weight()` stays in `[1, 10_000]` | opt-in (self-verify) |
| `any_bps_is_bounded` | `any_bps()` stays in `[0, 10_000]` | opt-in (self-verify) |
| `any_ordered_timestamps_are_ordered` | Generated timestamp pairs are ordered | opt-in (self-verify) |
| `any_lamports_bounded_is_bounded` | Bounded lamports generator respects its cap | opt-in (self-verify) |
| `identity_split_conserves` | The identity split conserves the total | opt-in (self-verify) |
| `floor_bps_split_conserves` | A floor based bps split conserves the total | opt-in (self-verify) |
| `ceiling_avg_single_element_returns_score` | Single element weighted average returns the score itself | opt-in (self-verify) |
| `ceiling_avg_bounded_at_cap` | Weighted average never exceeds the cap | opt-in (self-verify) |
| `ceiling_avg_empty_returns_zero` | Empty input yields zero | opt-in (self-verify) |
| `identity_u64_is_monotonic` | The monotonicity helper accepts the identity function | opt-in (self-verify) |

The seven vulnerable and fixed example pairs under `examples/` carry 29 more harnesses. Each example README lists its exact commands.

## Running the proofs

`scripts/kani.sh` selects features and harnesses from environment variables. CI runs the same script through `scripts/kani-ci.sh`, which also writes a SARIF report.

```bash
# smoke profile, the kani.yml smoke matrix leg
./scripts/kani.sh

# full profile, the kani.yml full matrix leg
KANI_FULL=1 ./scripts/kani.sh

# stress profile, heaviest SAT load, run locally
KANI_FULL=1 KANI_STRESS=1 ./scripts/kani.sh

# agent invariants (lib harness) and the benchmark.yml harness
KANI_AGENT=1 ./scripts/kani.sh
./scripts/benchmark-agent-flow.sh

# agent test-target proofs
KANI_AGENT=1 KANI_TESTS=1 ./scripts/kani.sh

# AccountInfo proofs, lib then test target
KANI_ACCOUNT_INFO=1 ./scripts/kani.sh
KANI_ACCOUNT_INFO=1 KANI_TESTS=1 ./scripts/kani.sh

# self-verification of the crate's own helpers
KANI_FULL=1 KANI_TESTS=1 ./scripts/kani.sh

# single harness
KANI_HARNESS=risk::proofs::proof_fee_sweep_conservation ./scripts/kani.sh
```

The benchmark target is to prove `agent::bench::verify_agent_flow_end_to_end` in under 5 seconds on `ubuntu-latest`.

## Fail to fix examples

Each example pair contains a vulnerable crate where a Kani harness finds the bug and a fixed crate where the same property proves.

| Example pair | Bug class |
| --- | --- |
| `escrow-release-policy-*` | Timelock release policy bypass |
| `cpi-allowlist-*` | CPI target not checked against the allowlist |
| `pda-seed-bump-*` | PDA seed count and length limits ignored |
| `replay-idempotency-*` | Conflicting duplicate event accepted |
| `oracle-quorum-median-*` | Quorum and median cap not enforced |
| `signer-owner-authority-*` | Signer and owner checks bypassed |
| `fsm-transition-guard-*` | State machine skips to a terminal state |

The escrow pair as a walkthrough:

```rust
// before, fails verification
let release_allowed = oracle_signed && now >= expires_at;

// after, proves
let release_allowed = agent_signed || (oracle_signed && now >= expires_at);
assert_timelock_release_policy(now, expires_at, agent_signed, oracle_signed, release_allowed);
```

```bash
# expected FAIL
cargo kani --manifest-path examples/escrow-release-policy-vulnerable/Cargo.toml \
  --harness proofs::vulnerable_allows_oracle_before_expiry

# expected PASS
cargo kani --manifest-path examples/escrow-release-policy-fixed/Cargo.toml \
  --harness proofs::fixed_blocks_oracle_before_expiry
```

<img src="docs/assets/gifs/escrow-before.gif" width="45%" alt="Escrow policy failing run" />
<img src="docs/assets/gifs/escrow-after.gif" width="45%" alt="Escrow policy passing run" />

Two end-to-end flows combine the pieces:

```bash
# oracle quorum, replay semantics, timelock, CPI allowlist, conservation, FSM
cargo kani --manifest-path examples/autonomous-payment-oracle-fixed/Cargo.toml \
  --harness proofs::proof_autonomous_payment_oracle_flow

# x402-style SVM payment settlement with conservation and CPI metadata checks
cargo kani --manifest-path examples/x402-svm-agent-payments-fixed/Cargo.toml \
  --harness proofs::proof_svm_x402_agentic_payment
```

## Feature flags

- `kani-full`: CI-viable full proof set
- `kani-stress`: SAT-heavy proofs, depends on `kani-full`
- `solana-agent`: agent invariants, CPI contracts, FSM guards
- `solana-account-info`: symbolic `AccountInfo` helpers and proofs

## Documentation

- Hosted API docs: https://mizuki0x.github.io/kamiyo-kani/kamiyo_kani/
- Local: `cargo doc --no-deps --open`
- Guides in `docs/`: `USER_GUIDE.md`, `BUG_CLASSES.md`, `TRUST_MODEL.md`, `ADOPTION.md`, `FORK_GUIDE.md`, `SKILLS.md`, `ROADMAP.md`, `RELEASE_CHECKLIST.md`
- Fork-local proof generation skills: `./scripts/skills-apply.sh --list`
- `templates/anchor-invariants`: starter template for Anchor teams
- `packages/kamiyo-kani-js`: experimental TypeScript shim for parsing proof artifacts

## Related work

- Kani `AccountInfo` generator RFC: https://github.com/model-checking/kani/issues/4550
- Percolator risk primitive alignment: https://github.com/aeyakovenko/percolator/pull/19

## License

MIT, see [LICENSE](LICENSE).
