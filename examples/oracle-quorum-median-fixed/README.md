# Oracle Quorum/Median (Fixed)

This crate enforces oracle quorum and median constraints.

## Run passing harnesses

```bash
cargo kani --manifest-path examples/oracle-quorum-median-fixed/Cargo.toml \
  --harness proofs::fixed_rejects_insufficient_reveals

cargo kani --manifest-path examples/oracle-quorum-median-fixed/Cargo.toml \
  --harness proofs::fixed_rejects_median_over_cap

cargo kani --manifest-path examples/oracle-quorum-median-fixed/Cargo.toml \
  --harness proofs::fixed_accepts_valid_consensus
```
