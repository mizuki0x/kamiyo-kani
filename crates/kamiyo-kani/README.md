# kamiyo-kani

Reusable Kani harnesses for Solana protocol math and agent invariants.

Everything is gated behind `cfg(kani)`, so normal builds are unaffected.

## Usage

Add as a dev dependency:

```toml
[dev-dependencies]
kamiyo-kani = { git = "https://github.com/kamiyo-ai/kamiyo-kani", package = "kamiyo-kani" }
```

Use in your own proofs:

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

    kani::assert(payout <= my_pnl.max(0) as u128);
}
```

Run:

```bash
cargo install --locked kani-verifier
cargo kani setup
cargo kani -p kamiyo-kani
```

## CPI contract stubs

- `cpi_stub!`: pre/post CPI modeling
- `cpi_contract!`: requires/body/ensures contract-style CPI modeling for lower path branching

## Policy helpers

- `assert_timelock_release_policy`
- `assert_oracle_consensus`
- `assert_fsm_transition_guard`

## Solana AccountInfo generators

`kamiyo-kani` exposes a Kani-only `AccountInfo` helper module aligned with [Kani issue #4550](https://github.com/model-checking/kani/issues/4550).

## License

MIT
