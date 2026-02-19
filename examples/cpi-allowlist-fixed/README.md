# CPI Allowlist (Fixed)

This crate fixes CPI target authorization and models allowlisted calls with `cpi_contract!`.

## Run the passing harnesses

```bash
cargo kani --manifest-path examples/cpi-allowlist-fixed/Cargo.toml \
  --harness proofs::fixed_rejects_unauthorized_program

cargo kani --manifest-path examples/cpi-allowlist-fixed/Cargo.toml \
  --harness proofs::fixed_allows_allowlisted_contract
```
