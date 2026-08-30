## Summary

Founding planning PR for **Augur** — a real-time AI game-coaching platform on
the ZeroClaw runtime (fork of zeroclaw-labs/zeroclaw, full history
preserved). This PR contains planning and scaffolding only; no coaching
product implementation.

- **Scope boundary**: docs, governance, game-module skeleton, planning
  artifacts, root identity files (README/NOTICE/CODEOWNERS/SECURITY/
  CONTRIBUTING), three new issue templates. No Rust code changes; no
  workflow changes (Actions are disabled repo-wide pending the CI-adaptation
  issue).
- **Blast radius**: none at runtime — nothing here executes.

What's inside:

| Area | Files |
|---|---|
| Product | `docs/product/{vision,mvp,user-experience}.md` |
| Architecture | `docs/architecture/*` (11 docs incl. the ZeroClaw reuse audit with per-path evidence) |
| Decisions | `docs/decisions/000{1,2,3}-*.md` (**proposed** — ratification issues in M0) + README |
| Governance & policy | `docs/governance/*` (4), `docs/policy/game-policy-review.md`, `docs/roadmap.md` |
| Game module | `games/hearthstone-battlegrounds/` — manifest, maintainers, schema/prompt READMEs, placeholder season skeleton (all `draft`), fixture/test dirs |
| Planning | `planning/github-plan.yaml` (repo metadata, 40 labels, 5 milestones, 40 issues w/ dependencies), `planning/issues/*.md` (exact bodies), `planning/apply-github-plan.sh` (idempotent) |
| Identity | README (upstream README preserved at `docs/ZEROCLAW-README.md`), NOTICE (ZeroClaw attribution kept, marks removed), CODEOWNERS, SECURITY, CONTRIBUTING preamble, issue templates (research/strategy/new-game) |

Key recorded decisions (each with repository evidence):

1. **Runtime integration** — bundled sidecar daemon + local JSON-RPC socket,
   desktop is RPC-only (zerocode pattern, CI-gated). Decision 0001.
2. **Game adapters** — compile-time registry; strategy packs are the
   data-only community path; WASM plugins rejected for now. Decision 0002.
3. **Strategy retrieval** — metadata filters + lexical FTS5 with caps and
   citation post-check; embeddings later, evidence-gated. Decision 0003.

## Testing (required)

Docs-only PR. Validation performed: YAML front matter of every strategy
skeleton parses; `planning/apply-github-plan.sh` passes `bash -n`; internal
doc links checked by inspection (link CI arrives with the CI-adaptation
issue). No `cargo` surface touched.

## Security & Privacy Impact (required)

No code paths changed. The PR *documents* the security model (threat model,
trust boundaries, injection defenses, privacy defaults) and the "coach, not
bot" enforcement plan. No secrets, PII, or real user data in any file;
fixture directories are empty placeholders.

## Compatibility (required)

No runtime behavior changes. Upstream files modified: README.md (moved to
docs/ZEROCLAW-README.md), NOTICE, CODEOWNERS, SECURITY.md, CONTRIBUTING.md,
.github/ISSUE_TEMPLATE/{config.yml,+3 new}. All inventoried for
upstream-merge hygiene in docs/architecture/upstream-sync.md.

## Rollback

Revert the merge commit; delete labels/milestones/issues via GitHub if
desired (the plan YAML documents exactly what was created).
