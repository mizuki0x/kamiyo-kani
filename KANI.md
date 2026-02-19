# Kani Workflows

Run one Kani command at a time. Parallel invocations can race on `target/kani` artifacts.

## Install

```bash
cargo install --locked kani-verifier
cargo kani setup
```

## Default proofs (smoke set)

```bash
./scripts/kani.sh
```

Runs a fast, deterministic harness set used in CI.

## Full proofs (CI-viable full set)

```bash
KANI_FULL=1 ./scripts/kani.sh
```

Runs smoke plus additional full-mode harnesses that are expected to complete in CI.

## Stress proofs (SAT-heavy harnesses)

```bash
KANI_FULL=1 KANI_STRESS=1 ./scripts/kani.sh
```

Runs all stress harnesses. These are intentionally heavier and may take significantly longer.

## Agent + AccountInfo smoke proofs

```bash
KANI_AGENT=1 KANI_ACCOUNT_INFO=1 ./scripts/kani.sh
```

This runs default smoke proofs plus the agent end-to-end harness.

## Agent + AccountInfo full proofs

```bash
KANI_FULL=1 KANI_AGENT=1 KANI_ACCOUNT_INFO=1 ./scripts/kani.sh
```

## Agent + AccountInfo stress proofs

```bash
KANI_FULL=1 KANI_STRESS=1 KANI_AGENT=1 KANI_ACCOUNT_INFO=1 ./scripts/kani.sh
```

## CI-style output + SARIF

```bash
KANI_OUT_DIR=kani-results ./scripts/kani-ci.sh
```

Artifacts:

- `kani-results/kani.log`
- `kani-results/summary.md`
- `kani-results/kani.sarif`

## `solana-account-info` warning note

When `KANI_ACCOUNT_INFO=1` is enabled, Kani may print unsupported-construct warnings from upstream Solana internals. Current harnesses verify successfully; treat new warnings as actionable only if a harness fails.

## Agent-flow benchmark

```bash
./scripts/benchmark-agent-flow.sh
```

Artifacts:

- `bench-results/agent-flow.log`
- `bench-results/summary.md`
- `bench-results/metrics.env`
