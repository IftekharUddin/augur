# Proposing and Adding a New Game

## Proposal (issue first, code later)

Open a **New game proposal** issue (template) covering: the game and why
coaching fits; policy posture (does the publisher permit overlays, screen
analysis, real-time advice? cite terms); observation feasibility (what a
screen capture can honestly extract); who maintains it; and expected strategy
corpus shape.

**Acceptance criteria** — a proposal is accepted when:

1. The policy review (docs/policy/game-policy-review.md) records no blocker.
2. At least one named maintainer commits to the game.
3. The observation plan uses only permitted inputs (no memory reading, no
   injection).
4. A platform maintainer confirms no platform changes are required — or the
   required changes have their own issues.

## Implementation path

1. Scaffold from the new-game template: `games/<game-id>/` with `game.yaml`,
   `maintainers.yaml`, schemas, prompts, empty strategy pack, fixtures dir,
   adapter crate skeleton implementing `GameAdapter` against fixtures.
2. `game.yaml` starts `status: experimental`; the game ships disabled by
   default until `community`.
3. Fixture-driven tests pass; the architecture test proves no platform crate
   was touched (or touched changes were reviewed as platform work).
4. CODEOWNERS entry mirrors `maintainers.yaml`.

## Status ladder

`experimental` (in-tree, off by default) → `community` (on, community
support) → `maintained` (a maintainer with SLA-ish responsiveness) →
`deprecated` (frozen, warns) / `disabled-policy-review` (hard-disabled; set
immediately when a policy question arises, cleared only by a recorded
re-review).

## Retiring

A game with no maintainer for 90+ days or a standing policy blocker moves to
`deprecated`/`disabled-policy-review` by governance PR; its strategy corpus
stays in-tree as history.
