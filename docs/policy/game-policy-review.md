# Game Policy Review

Per-game, recorded review of publisher terms before any release that
supports the game. Output lives in `game.yaml` (`policy_review:` block:
status, date, reviewer, sources, notes) and gates support status.

## Hearthstone Battlegrounds — review checklist (Milestone 0 issue)

To review before M1 ships (the issue tracks the actual findings; nothing here
asserts a conclusion):

- Blizzard End User License Agreement — clauses on third-party software,
  automation, and "hacks/cheats/bots"; Augur's read-only, no-input design
  must be evaluated against the exact current text.
- Blizzard's stance and enforcement history on **deck trackers and overlays**
  (e.g. the long-standing tolerance of Hearthstone Deck Tracker reading
  `Power.log` with log output officially togglable) — relevant precedent for
  log-file use and overlays, to be cited concretely in the review.
- Hearthstone in-game options for log output (the supported observation
  side-channel, if used).
- Any API terms if official APIs are used.
- Streaming/derivative-content guidelines only insofar as they affect
  screenshots in diagnostics/fixtures.

## Standing rules

- A game with an unresolved policy question is `disabled-policy-review` —
  hard-disabled in the registry, not just warned.
- Future games known to prohibit overlays, screen analysis, or real-time
  advice are recorded as such in their proposal issue and rejected or
  scoped-down accordingly.
- Policy reviews are re-run when the publisher's terms change materially, on
  a major Augur release, or on maintainer request; each run appends to the
  `policy_review` history.
- Nothing in a policy review authorizes prohibited inputs: no memory
  reading, no injection, no traffic manipulation, no anti-cheat
  circumvention, regardless of terms interpretation.
