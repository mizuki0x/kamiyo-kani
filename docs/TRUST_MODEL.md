# Trust Model

## Scope

This repo is intended to reduce verification risk, so the trust boundary must stay small and explicit.

## Policy

- Keep proof-critical math, invariants, and harness logic in this repository.
- Do not move trusted spec logic into external crates.
- Do not use git or path dependencies for workspace crates.
- Keep optional integrations (`solana-account-info`) feature-gated and out of the default proof surface.

## CI enforcement

- `scripts/check-trust-boundary.sh` enforces:
  - no git dependencies in workspace manifests
  - no path dependencies in workspace manifests
  - no git-sourced crates in `Cargo.lock`
- `.github/workflows/audit.yml` runs the trust-boundary check before `cargo audit`.

## Change rules

- If a new dependency is required, add a rationale in the PR with:
  - why in-repo code is insufficient
  - security review notes
  - rollback plan
- If a dependency affects proof-critical semantics, prefer copying a reviewed implementation into this repo over linking external spec crates.
