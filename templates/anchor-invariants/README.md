# Anchor Invariant Template

Use this template to start proving invariants in an Anchor program with `kamiyo-kani`.

## Invariants to add first

1. fund conservation across settle paths
2. signer/authority gate correctness
3. PDA seed + bump stability
4. replay/idempotency semantics on unique IDs

## Example harness skeleton

```rust
#![cfg(kani)]

#[kani::proof]
fn funds_are_conserved() {
    let total_before: u64 = kani::any();
    let debit: u64 = kani::any();
    kani::assume(debit <= total_before);

    let credit = debit;
    let total_after = (total_before - debit) + credit;

    kani::assert(total_before == total_after);
}
```
