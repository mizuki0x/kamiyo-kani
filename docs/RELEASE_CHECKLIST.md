# v0.1 Release Checklist

## Product bar

- [ ] default proof set finishes in under 2 minutes on GitHub Actions
- [ ] no false-positive failures in default profile
- [ ] one minimal integration example for Anchor users

## Technical bar

- [ ] `cargo check`, `cargo test`, `cargo clippy` pass on stable
- [ ] `./scripts/kani.sh` passes
- [ ] `KANI_FULL=1 ./scripts/kani.sh` passes
- [ ] CI and Kani workflows green on `main`

## Trust bar

- [ ] publish at least 3 real bug classes with fail/pass proof diffs
- [ ] document all assumptions and unsupported Solana runtime semantics
- [ ] pin reproducible toolchain versions for releases

## Adoption bar

- [ ] crates.io publish (`kamiyo-kani`)
- [ ] template repo example linked from README
- [ ] short migration guide for teams currently using custom harnesses
