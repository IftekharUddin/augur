# Game Policy Review

Per-game, recorded review of publisher terms before any release that
supports the game. Output lives in `game.yaml` (`policy_review:` block:
status, date, reviewer, sources, notes) and gates support status.

## Hearthstone Battlegrounds — review checklist (Milestone 0 issue)

**Findings:** [hearthstone-battlegrounds-review.md](hearthstone-battlegrounds-review.md)
(sources retrieved 2026-08-30). The conclusion is not recorded there:
`policy_review.status` stays `pending` until the maintainer decides, because
the decision is a risk judgment about other people's accounts rather than a
technical finding.

Checklist, for reference and for future re-runs:

- Blizzard End User License Agreement — clauses on third-party software,
  automation, and "hacks/cheats/bots"; Augur's read-only, no-input design
  must be evaluated against the exact current text.
- Blizzard's stance and enforcement history on **deck trackers and overlays**
  — relevant precedent for log-file use and overlays, to be cited concretely in
  the review.

  **Correction (2026-08-30):** an earlier version of this line described
  Hearthstone's log output as "officially togglable". The review could not
  verify that. The documented mechanism is that Hearthstone Deck Tracker writes
  a `log.config` file into the game's installation directory to enable a debug
  logging facility; no Blizzard in-game setting, patch note, or support article
  exposing log output as a supported option was found. That is a materially
  weaker claim to authorization than a publisher-provided switch, and it sits
  closer to the EULA's "Hacks" definition than the original wording implied.
  See [hearthstone-battlegrounds-review.md](hearthstone-battlegrounds-review.md).
- Hearthstone in-game options for log output, **if any exist**. The 2026-08-30
  review found none; this line previously assumed they did.
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
