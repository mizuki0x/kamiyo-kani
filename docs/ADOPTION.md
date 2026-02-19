# Adoption Plan

## Technical positioning

- `kamiyo-kani` is a verification toolkit for Solana teams shipping in production.
- Goal: reduce onboarding time for proof-driven checks from days to minutes.

## Channels

- Solana Discord (developer channels)
- Rust Zulip `#kani`
- r/solana
- Anchor and Sealevel builder communities

## Launch sequence

1. Publish crate and docs.
2. Post one concrete fail -> fix case with reproducible commands.
3. Publish benchmark artifact showing agent-flow harness runtime.
4. Open one integration PR against a real Solana program.

## Success criteria

- 3+ external repos adopt template harnesses
- at least 1 upstream contribution merged to Kani ecosystem
- repeatable CI runtime for default proof profile under 2 minutes
