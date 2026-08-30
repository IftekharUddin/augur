## Problem

Governance docs describe per-game ownership, strategy-maintainer scopes,
and review rules, but nothing checks that CODEOWNERS mirrors
maintainers.yaml or that strategy PRs stay in their lane.

## Context and repository evidence

game-maintainers.md (mirror requirement, review matrix);
strategy-review.md (strategy-only path check); .github/CODEOWNERS
(founding single-owner rules with per-game sections).

## Scope

CI checks: (a) CODEOWNERS game sections ↔ maintainers.yaml consistency;
(b) PRs labeled `type:strategy` touch only `games/*/strategies/**` +
`games/*/fixtures/**`; (c) maintainers.yaml schema validation. Governance
docs get the "how to run a nomination" mechanics finalized; labels applied
by a path-labeler config adapted from upstream `.github/labeler.yml`
patterns for Augur areas.

## Non-goals

Voting/quorum machinery (upstream FND-003-scale governance is overkill at
founding; revisit at real community scale).

## Acceptance criteria

- Mirror check red/green proven; lane check red/green proven.
- Path labeler applies `area:*`/`game:*` labels on test PRs.

## Dependencies

#ci-adaptation.

## Test plan

Fixture PR simulations in CI (script-level tests over diffs).

## Documentation impact

Governance docs reference the live checks.

## Security, privacy, and policy considerations

Ownership enforcement is the review-integrity backbone for community
content.
