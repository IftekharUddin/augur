# Game Maintainership

## Roles

| Role | Owns | Review power |
|---|---|---|
| Core maintainers | Whole repo; final escalation | Everything |
| Platform-area maintainers | `crates/augur-*`, `apps/augur-desktop` | Their area |
| Security maintainers | `augur-capture`, security docs, capture-method changes, SECURITY.md | Required on security-sensitive paths |
| Release maintainers | Release workflows, packaging, signing | Release-gating changes |
| Game maintainers | `games/<game-id>/**` (adapter + data) | Their game |
| Strategy maintainers | `games/<game-id>/strategies/**` (scoped by season and/or category) | Strategy-only PRs for their scope |
| Contributors | PRs anywhere | — |

At founding, all roles are held by the founding maintainer; the structure
exists so authority can be delegated as the community grows, not to pretend a
community already exists.

## Per-game ownership

`games/<game-id>/maintainers.yaml` is the source of truth:

```yaml
game: hearthstone-battlegrounds
status: experimental
maintainers:
  - github: IftekharUddin
    scopes: [adapter, strategy]
strategy_maintainers: []
```

CODEOWNERS mirrors it: `/games/<game-id>/** @<maintainers>`, with a narrower
`/games/<game-id>/strategies/**` rule adding strategy maintainers
(last-match-wins ordering, following upstream's documented CODEOWNERS style).
A CI check (Milestone 4) validates the mirror.

## Becoming a game maintainer

Sustained, quality contributions to the game's adapter or strategy corpus →
nomination by an existing maintainer (or self-nomination) via a
`type:governance` issue → core-maintainer approval → `maintainers.yaml` +
CODEOWNERS PR. Inactive maintainers (no review/commit activity for 90 days
with pending review load) are pinged, then rotated to emeritus by the same
process.

## Review requirements

- **Executable changes** (adapter code, capture, runtime): one platform/game
  maintainer; **two** reviews when `risk:high` or touching `augur-capture`
  or permission surfaces (mirrors upstream's two-approval rule for
  `risk:high`/`domain:security`).
- **Strategy-only changes**: one strategy maintainer for the scope;
  validation CI must pass; `stable` status requires named reviewer in front
  matter.
- **Shared-runtime changes**: escalate to platform maintainers; if generic,
  redirect upstream (see docs/architecture/upstream-sync.md).
- Malicious or plagiarized strategy: removed on sight by any maintainer,
  recorded in a `type:governance` issue; repeat offenders lose contributor
  standing per the Code of Conduct process.
