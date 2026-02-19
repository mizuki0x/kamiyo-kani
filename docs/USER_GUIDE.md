# User Guide

## 1. Add the crate

```toml
[dev-dependencies]
kamiyo-kani = "0.1.1"
```

## 2. Install Kani

```bash
cargo install --locked kani-verifier
cargo kani setup
```

## 3. Start with one high-value invariant

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

## 4. Run smoke proofs in CI

```bash
./scripts/kani.sh
```

## 5. Expand to full and stress profiles

```bash
KANI_FULL=1 ./scripts/kani.sh
KANI_FULL=1 KANI_STRESS=1 ./scripts/kani.sh
```

## 6. Model CPI safely

Use `cpi_contract!` to keep preconditions explicit and avoid unconstrained branching.

```rust
cpi_contract! {
    name: invoke_allowlisted_cpi,
    program: ALLOWLISTED_PROGRAM,
    args: |amount: u64| {},
    requires: {
        kani::assume(amount <= 10_000);
    },
    body: {},
    ensures: {},
    record: {
        lamports_transferred: amount,
        accounts_touched: 2,
    },
    auto_asserts: {
        oracle_monotonic: (0u64, 1u64);
    },
}
```

## 7. Publish SARIF to GitHub Security

```yaml
- name: Run Kani
  run: KANI_OUT_DIR=kani-results ./scripts/kani-ci.sh

- name: Upload SARIF
  if: always()
  uses: github/codeql-action/upload-sarif@v3
  with:
    sarif_file: kani-results/kani.sarif
```

## 8. Add one fail->fix example before rollout

Use any pair from `examples/` and include both commands in your PR description.
