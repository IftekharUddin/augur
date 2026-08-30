## Problem

Augur must not ship Battlegrounds support without a recorded review of
Blizzard's terms. `games/hearthstone-battlegrounds/game.yaml` carries
`policy_review.status: pending`, which hard-blocks release.

## Context and repository evidence

docs/policy/game-policy-review.md — checklist: Blizzard EULA third-party
software/automation clauses; deck-tracker/overlay precedent (incl. official
log-output toggles); log-file posture; API terms if used.

## Scope

Perform the review against current published terms; record findings, sources
(with retrieval dates), and a go/conditional/no-go recommendation in the
`policy_review` block + a docs/policy/ report; define what (if anything)
changes scope (e.g., Power.log usage).

## Non-goals

Legal advice; reviewing other games.

## Proposed approach

Research issue (timeboxed 3 days); findings PR updates game.yaml + policy
doc; disputes escalate per governance.

## Acceptance criteria

- `policy_review.status` is a recorded outcome with dated sources, not
  `pending`.
- Any scope changes are filed as issues on affected milestones.

## Dependencies

None. Blocks M1 release exit, not M1 development.

## Test plan

n/a (research); CI validates game.yaml shape.

## Documentation impact

game-policy-review.md gains the HSBG record.

## Security, privacy, and policy considerations

This IS the policy gate; the standing rules in the policy doc (no memory
reading etc.) apply regardless of findings.
