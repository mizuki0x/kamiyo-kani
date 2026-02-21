# Skills

`kamiyo-kani` skills are prompt templates for AI-assisted, fork-first customization.

Goal: keep the core small and auditable while letting teams generate project-specific proof harnesses without adding everything to upstream.

## Layout

- `skills/index.json`: skill registry
- `skills/<skill-id>/<version>/SKILL.md`: behavior and acceptance contract
- `skills/<skill-id>/<version>/PROMPT.md.tmpl`: renderable prompt template

## Apply a skill

Render a skill prompt:

```bash
./scripts/skills-apply.sh \
  --skill solana-account-generator \
  --version 1.0 \
  --files-modified crates/kamiyo-kani/src/account_info.rs,crates/kamiyo-kani/tests/account_info_verify.rs
```

List available skills:

```bash
./scripts/skills-apply.sh --list
```

Apply an AI-generated patch after review:

```bash
./scripts/skills-apply.sh \
  --skill add-solana-token-proof \
  --version 1.0 \
  --patch-file /tmp/llm.patch
```

The script writes rendered prompts to `.skills/runs/` for auditability.

## Security model

- skill execution does not auto-call remote models
- generated patches are local artifacts reviewed in git
- patch application is explicit (`--patch-file`)
- default flow keeps forks small and reviewable
