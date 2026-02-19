# Adoption Guide

This document is for teams integrating `kamiyo-kani` into production CI.

## Integration tracks

- Track A: add one proof to an existing program and run smoke checks.
- Track B: add fail->fix examples for your top bug class and gate merges on proof pass.
- Track C: include full profile, SARIF upload, and benchmark budget checks.

## Recommended rollout

1. Add `kamiyo-kani` as a dev dependency.
2. Add one invariant in each class you care about:
- value conservation
- CPI authorization
- replay/idempotency
- FSM transition guards
3. Run smoke profile on every push.
4. Run full profile on pull requests and main.
5. Keep stress profile scheduled/nightly.

## CI blueprint (copy-paste)

```yaml
name: Kani

on:
  push:
    branches: [main]
  pull_request:

permissions:
  contents: read
  security-events: write

jobs:
  kani:
    runs-on: ubuntu-latest
    strategy:
      fail-fast: false
      matrix:
        profile: [smoke, full]
    steps:
      - uses: actions/checkout@v4
      - uses: model-checking/kani-github-action@v1
        with:
          kani-version: latest
          command: ${{ matrix.profile == 'full' && 'KANI_FULL=1 ./scripts/kani-ci.sh' || './scripts/kani-ci.sh' }}
      - uses: actions/upload-artifact@v4
        with:
          name: kani-results-${{ matrix.profile }}
          path: kani-results
      - uses: github/codeql-action/upload-sarif@v3
        if: always()
        with:
          sarif_file: kani-results/kani.sarif
```

## Benchmark gate blueprint

```yaml
name: Benchmarks

on:
  push:
    branches: [main]
  pull_request:

jobs:
  agent-flow:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: model-checking/kani-github-action@v1
        with:
          kani-version: latest
          command: ./scripts/benchmark-agent-flow.sh
      - uses: actions/upload-artifact@v4
        if: always()
        with:
          name: benchmark-agent-flow
          path: bench-results
```

## Docs deployment

The repository uses GitHub Pages with workflow deployment.

```yaml
- name: Build rustdoc
  run: cargo doc --workspace --no-deps

- uses: actions/upload-pages-artifact@v3
  with:
    path: public

- uses: actions/deploy-pages@v4
```

## Suggested PR policy

Require all of:
- `CI` workflow green
- `Kani` workflow green
- `Docs` workflow green
- benchmark job green

## Publish checklist

- Crate release: `cargo publish -p kamiyo-kani`
- JS shim dry-run: `npm publish --dry-run --access public` in `packages/kamiyo-kani-js`
- Attach benchmark summary and one fail->fix command pair in release notes

## Community channels

- Rust Zulip: `#kani`
- Solana Discord: security and smart-contract development channels
- r/solana: post one reproducible fail->fix sample
