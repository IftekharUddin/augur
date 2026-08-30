## Problem

"Adding a game should not require archaeology": the new-game process doc
exists but there's no runnable template.

## Context and repository evidence

new-game-process.md implementation path; games/hearthstone-battlegrounds/
as the reference layout; game.yaml manifest shape.

## Scope

`games/_template/` (or generator: `augur game scaffold <id>`): manifest,
maintainers.yaml, schemas dir, prompts README, empty season pack passing
validation, adapter crate skeleton implementing GameAdapter against the
replay capture provider, fixture dirs, per-game test wiring; docs walk a
contributor end-to-end.

## Non-goals

Auto-registering in the compile-time registry (deliberate one-line manual
step, reviewed).

## Acceptance criteria

- Scaffolded game compiles, validates, and passes its skeleton tests
  out of the box.
- Walkthrough doc tested by scaffolding the second-game proof from it.

## Dependencies

#adapter-api-freeze (or co-landed).

## Test plan

CI job scaffolds a throwaway game and builds it.

## Documentation impact

new-game-process.md references the real template.

## Security, privacy, and policy considerations

Template defaults `status: experimental` + `policy_review: pending`
(disabled until reviewed) — the safe-by-default path.
