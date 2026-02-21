# Fork Guide

This project is designed for small, auditable forks.

Use upstream for reusable primitives. Use skills for local program-specific invariants.

## Recommended fork workflow

1. Fork and clone `kamiyo-kani`.
2. Keep core updates synced from upstream.
3. Add custom proof logic through skills, not by expanding upstream core.
4. Validate every skill run with local checks.
5. Keep each forked delta small enough for manual audit.

## Example: add custom Solana account proofs

```bash
./scripts/skills-apply.sh \
  --skill solana-account-generator \
  --version 1.0 \
  --files-modified crates/kamiyo-kani/src/account_info.rs,crates/kamiyo-kani/tests/account_info_verify.rs
```

Feed the rendered prompt to your model, review the patch, then apply it:

```bash
./scripts/skills-apply.sh \
  --skill solana-account-generator \
  --version 1.0 \
  --patch-file /tmp/llm.patch
```

## Keep forks maintainable

- avoid unrelated refactors in the same patch
- prefer new example crates over invasive framework changes
- document each local invariant class in the fork README
- use `upstream-merge` skill when rebasing onto upstream
