# Contributing

## Local setup

```bash
git clone https://github.com/kamiyo-ai/kamiyo-kani.git
cd kamiyo-kani
cargo check --workspace
```

## Validation requirements

Before opening a PR, run:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
./scripts/kani.sh
```

If you touched heavy harnesses or `cover!` assumptions, also run:

```bash
KANI_FULL=1 ./scripts/kani.sh
```

## PR requirements

Each PR should include:

- invariant or behavior change summary
- exact commands executed
- before/after behavior if a proof was fixed
- docs updates for any new harness or feature flag
