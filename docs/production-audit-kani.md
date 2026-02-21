# Production Audit: Kani Surface

## Executive Summary
The crate is technically solid and proof coverage is broad, but there were still release-quality gaps in security governance and CI policy. The code could ship for early adopters, but not with a strict production bar until dependency-audit enforcement and workflow hardening were explicit and automated.

## Critical Issues (P0 - Block Release)
- [x] None found.

## High Priority (P1 - Fix Before Launch)
- [x] No enforced dependency-audit gate in CI | New advisories could land silently and ship without review | Added `Dependency Audit` workflow and repo-local `.cargo/audit.toml` policy with explicit exception tracking.
- [x] CI test job did not execute feature-gated paths | Regressions in `solana-agent` / `solana-account-info` could pass CI unnoticed | Updated CI tests to run `cargo test --workspace --all-features`.
- [x] Trust-boundary validation only ran in audit workflow | Dependency-policy regressions could fail later than necessary | Added trust-boundary check to main CI and kept it in dependency audit workflow.

## Medium Priority (P2 - Fix Soon After Launch)
- [x] Skills patch application lacked preflight validation | Bad patches could fail midway and leave unclear operator feedback | Added `git apply --check` preflight and strict target-repo validation in `scripts/skills-apply.sh`.
- [x] Skills CLI argument parsing did not guard missing flag values | User/operator mistakes produced brittle failure modes | Added explicit missing-value checks for all value-taking flags.
- [x] Trust-boundary script relied on non-portable tooling assumptions | Runner drift could cause false failures | Reduced metadata scope (`--no-deps`) and switched lockfile scan to POSIX-available `grep`.

## Low Priority (P3 - Technical Debt)
- [x] Proof test file had heavy narrative comments and one `expect` path | Reviewability and style drift from production code standards | Cleaned `crates/kamiyo-kani/tests/agent_verify.rs` to be concise and deterministic while preserving behavior.
- [x] FSM helper loops did not short-circuit on match | Minor path bloat in proofs | Added early breaks in `assert_valid_transition` and `assert_terminal_state`.

## Security Assessment
- CI now enforces dependency auditing (`cargo audit --deny warnings`) on push/PR/schedule.
- Advisory exception policy is now local to repo and documented in `SECURITY.md`.
- Current accepted exception: `RUSTSEC-2025-0141` (`bincode`), transitive through `solana-program 2.3.x`. This is tracked with rationale and should be re-evaluated on every Solana upgrade.

## Performance Assessment
- Local benchmark harness (`agent::bench::verify_agent_flow_end_to_end`) remains green and under the existing profile constraints.
- No new runtime overhead added in library code paths; changes are workflow/script hardening and proof-helper cleanup.

## Observability Assessment
- Kani + benchmark artifact publication remains intact.
- Workflow hardening adds timeouts and explicit concurrency controls to reduce hung/stale CI executions.

## Recommended Architecture Changes
- Keep Kani harness selection split across smoke/full/stress tiers; this is the right tradeoff for CI time vs. depth.
- Keep advisory exceptions minimal and tied to upstream dependency constraints, never to convenience.

## Test Coverage Gaps
- Feature-gated test execution gap is now closed in CI.
- Remaining gap: no property-level mutation tests for scripts. This is acceptable for now but should be added if skills CLI complexity grows.

## Action Plan
1. Enforce dependency auditing in CI and codify advisory exceptions.
2. Harden CI workflow permissions, concurrency, and runtime bounds.
3. Harden skills patch tooling for deterministic operator behavior.
4. Cleanup proof tests/helpers to keep review quality high.
5. Re-run full validation suite (`fmt`, `clippy`, `test`, `kani`, `benchmark`, `doc`, `audit`).
