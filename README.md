# KAMIYO Kani

[![CI](https://github.com/kamiyo-ai/kamiyo-kani/actions/workflows/ci.yml/badge.svg)](https://github.com/kamiyo-ai/kamiyo-kani/actions/workflows/ci.yml)
[![Kani](https://github.com/kamiyo-ai/kamiyo-kani/actions/workflows/kani.yml/badge.svg)](https://github.com/kamiyo-ai/kamiyo-kani/actions/workflows/kani.yml)
[![Crates.io](https://img.shields.io/crates/v/kamiyo-kani.svg)](https://crates.io/crates/kamiyo-kani)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Reusable Kani verification primitives and harnesses for Solana programs.

## Why this repo exists

Most Solana teams do not need a full formal methods stack. They need a fast path to prove a small set of high-value invariants in CI:

- value conservation (lamports and split math)
- bounds and monotonicity for risk math
- PDA seed/bump constraints
- replay and state transition safety
- AccountInfo mutation invariants

`kamiyo-kani` packages these as copyable primitives and runnable harnesses.

## Install

```toml
[dev-dependencies]
kamiyo-kani = "0.1"
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

    kani::assert(payout <= my_pnl.max(0) as u128);
}
```

Run:

```bash
cargo install --locked kani-verifier
cargo kani setup
cargo kani -p kamiyo-kani
```

## Feature flags

- `kani-full`: enable additional heavyweight proofs
- `solana-agent`: enable agent invariants
- `solana-account-info`: enable `AccountInfo` generators and proofs

## Included assets

- `crates/kamiyo-kani`: verification primitives and harnesses
- `templates/anchor-invariants`: starter template for Anchor teams
- `docs/RELEASE_CHECKLIST.md`: v0.1 launch checklist

## Related work

- Kani `AccountInfo` generator RFC: https://github.com/model-checking/kani/issues/4550
- Percolator risk primitive alignment: https://github.com/aeyakovenko/percolator/pull/19

## License

MIT
