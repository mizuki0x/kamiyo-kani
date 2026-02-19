# Kani Workflows

## Install

```bash
cargo install --locked kani-verifier
cargo kani setup
```

## Default proofs

```bash
./scripts/kani.sh
```

## Full proofs

```bash
KANI_FULL=1 ./scripts/kani.sh
```

## Agent + AccountInfo proofs

```bash
KANI_AGENT=1 KANI_ACCOUNT_INFO=1 ./scripts/kani.sh
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
