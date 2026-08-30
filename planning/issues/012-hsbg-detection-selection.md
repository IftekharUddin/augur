## Problem

Augur must find the Hearthstone window (detect the process, match the
window) and let the user confirm or override — the manifest data exists
(`games/hearthstone-battlegrounds/game.yaml` detection block) but nothing
consumes it.

## Context and repository evidence

game-adapter.md (detect responsibility, DetectionContext);
user-experience.md (manual selector fallback, detection test in onboarding);
`game.yaml` executables/window_titles.

## Scope

Detection engine in `augur-observation` (process list + window-title match
against manifest rules, confidence-ranked candidates); `augur/capture/windows`
+ `augur/capture/select` RPC wiring; desktop picker UI consumes it.

## Non-goals

Auto-start of coaching; multi-game arbitration polish (M4).

## Acceptance criteria

- With Hearthstone running: auto-detected, correct window preselected.
- Without: explicit "no supported game" state; manual selection works on any
  window (capture still window-scoped).
- Detection test in onboarding passes/fails honestly.

## Dependencies

#macos-capture-provider (enumeration), #augur-rpc-extensions.

## Test plan

Fixture enumerations (fake window lists) for matching logic; one live test.

## Documentation impact

user-experience.md detection states implemented.

## Security, privacy, and policy considerations

Enumeration metadata stays local; titles of unrelated windows are never sent
to providers.
