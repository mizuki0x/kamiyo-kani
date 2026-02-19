# PDA Seed/Bump Validation (Fixed)

This crate enforces Solana PDA limits: max 16 seeds and max 32 bytes per seed.

## Run the passing harnesses

```bash
cargo kani --manifest-path examples/pda-seed-bump-fixed/Cargo.toml \
  --harness proofs::fixed_rejects_seed_count_overflow

cargo kani --manifest-path examples/pda-seed-bump-fixed/Cargo.toml \
  --harness proofs::fixed_rejects_seed_len_overflow

cargo kani --manifest-path examples/pda-seed-bump-fixed/Cargo.toml \
  --harness proofs::fixed_accepts_valid_shape_and_bump
```
