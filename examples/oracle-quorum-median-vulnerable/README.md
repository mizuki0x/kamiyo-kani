# Oracle Quorum/Median (Vulnerable)

This crate intentionally accepts invalid oracle consensus states.

## Run failing harnesses

```bash
cargo kani --manifest-path examples/oracle-quorum-median-vulnerable/Cargo.toml \
  --harness proofs::vulnerable_accepts_insufficient_reveals

cargo kani --manifest-path examples/oracle-quorum-median-vulnerable/Cargo.toml \
  --harness proofs::vulnerable_accepts_median_over_cap
```

Expected: both harnesses fail.
