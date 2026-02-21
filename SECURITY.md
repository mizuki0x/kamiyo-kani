# Security Policy

## Scope

This repository contains formal-verification utilities and proof harnesses. Vulnerabilities in this repo can impact verification confidence and downstream protocol safety.

## Reporting

Report vulnerabilities privately to:

- security@kamiyo.ai

Do not open public issues for security disclosures.

## Response targets

- acknowledgment: 2 business days
- triage: 7 business days
- remediation timeline: severity-based

## Dependency posture

- CI runs `cargo audit --deny warnings` on push, PR, and weekly schedule.
- Advisory exceptions are tracked in `.cargo/audit.toml` and must include a concrete rationale.
