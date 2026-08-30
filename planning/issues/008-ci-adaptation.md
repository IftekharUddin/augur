## Problem

Inherited workflows target upstream's self-hosted runners
(`blacksmith-8vcpu-ubuntu-2404` in `.github/actionlint.yaml`) and encode
upstream's required-check set; GitHub Actions is disabled repo-wide at fork
time to prevent 27 workflows failing/queueing. Augur has no CI.

## Context and repository evidence

- `.github/workflows/ci.yml` (25-job `CI Required Gate`), `pr-title.yml`
  (Conventional Commits with required scope — keep), `desktop-check.yml`,
  `platform-tests.yml` split (heavy matrices scheduled, not required).
- Augur CI needs: docs/architecture/testing-and-evaluation.md "CI" section.

## Scope

Replace/disable upstream workflows deliberately: an Augur `ci.yml` on hosted
runners (fmt, clippy, workspace check, unit/component tests, architecture
tests, strategy validation once it exists, docs links) collapsing into one
`Augur Required Gate`; keep `pr-title.yml`; archive the rest under
`.github/workflows-upstream/` (or `if: false` guards) with a README; then
re-enable Actions.

## Non-goals

Release workflows (M4 issue); reproducing upstream's full matrix.

## Proposed approach

Start minimal and green; grow with the crates. Branch protection: require
the single gate check (upstream's aggregate pattern).

## Acceptance criteria

- Actions re-enabled; gate green on the planning branch.
- PR title check active.
- No workflow references unavailable runners.
- Branch protection on `master` requires the gate.

## Dependencies

None hard; #augur-crate-skeleton lands its tests into this gate.

## Test plan

A deliberately-failing PR proves the gate blocks; a green PR proves it
passes.

## Documentation impact

testing-and-evaluation.md CI section; CONTRIBUTING validation-evidence rule
retained.

## Security, privacy, and policy considerations

Re-enable Dependabot/audit workflows (inherited configs) once the gate is
stable; secret-scanning and dependency review on.
