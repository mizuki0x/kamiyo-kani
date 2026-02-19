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

Runs a fast, deterministic harness set.

## Full proofs (all harnesses for selected features)

```bash
KANI_FULL=1 ./scripts/kani.sh
```

## Agent + AccountInfo smoke proofs

```bash
KANI_AGENT=1 KANI_ACCOUNT_INFO=1 ./scripts/kani.sh
```

This runs default smoke proofs plus the agent end-to-end harness.

## Agent + AccountInfo full proofs

```bash
KANI_FULL=1 KANI_AGENT=1 KANI_ACCOUNT_INFO=1 ./scripts/kani.sh
```

## CI-style output + SARIF

```bash
KANI_OUT_DIR=kani-results ./scripts/kani-ci.sh
```

Artifacts:

- `kani-results/kani.log`
- `kani-results/summary.md`
- `kani-results/kani.sarif`

## Agent-flow benchmark

```bash
./scripts/benchmark-agent-flow.sh
```

Artifacts:

- `bench-results/agent-flow.log`
- `bench-results/summary.md`
- `bench-results/metrics.env`
