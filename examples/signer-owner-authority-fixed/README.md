# Signer/Owner Authority (Fixed)

This crate enforces authority checks for signer and expected owner.

## Run the passing harnesses

```bash
cargo kani --manifest-path examples/signer-owner-authority-fixed/Cargo.toml \
  --harness proofs::fixed_rejects_unsigned_authority

cargo kani --manifest-path examples/signer-owner-authority-fixed/Cargo.toml \
  --harness proofs::fixed_rejects_wrong_owner

cargo kani --manifest-path examples/signer-owner-authority-fixed/Cargo.toml \
  --harness proofs::fixed_accepts_signed_expected_owner
```
