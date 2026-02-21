# Skills Guide

`kamiyo-kani` ships a fork-first skills system for AI-assisted proof customization.

Use it when you want bespoke invariants without bloating upstream.

## What a skill is

A skill is a versioned prompt template plus an execution contract:

- `skills/<skill-id>/<version>/SKILL.md`
- `skills/<skill-id>/<version>/PROMPT.md.tmpl`

The template is rendered by `scripts/skills-apply.sh` into `.skills/runs/`.

## Built-in skills

- `solana-account-generator@1.0`
- `add-solana-token-proof@1.0`
- `add-replay-safety-proof@1.0`
- `upstream-merge@1.0`

List them:

```bash
./scripts/skills-apply.sh --list
```

## Render a prompt

```bash
./scripts/skills-apply.sh \
  --skill solana-account-generator \
  --version 1.0 \
  --files-modified crates/kamiyo-kani/src/account_info.rs,crates/kamiyo-kani/tests/account_info_verify.rs \
  --print
```

## Apply an AI-generated patch

```bash
./scripts/skills-apply.sh \
  --skill add-solana-token-proof \
  --version 1.0 \
  --patch-file /tmp/llm.patch
```

The script uses `git apply` and writes run metadata next to the rendered prompt.

## Security posture

- no automatic remote model execution
- no implicit writes outside the git checkout
- patch application is explicit and reviewable
- all runs are logged in `.skills/runs/`
