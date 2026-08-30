## Problem

M1's spine: a user-triggered single observation (button + global hotkey)
flowing capture → envelope → coaching turn. No global-hotkey machinery
exists in the tree (verified absence).

## Context and repository evidence

mvp.md step 3; runtime-integration.md `augur/observe/once`;
`tauri-plugin-global-shortcut` absent from `Cargo.lock` (to be added).

## Scope

`augur/observe/once` end-to-end: desktop button + configurable global hotkey
(tauri-plugin-global-shortcut) → capture via bound window → normalized frame
→ envelope stub (schema-valid, extraction filled by #bg-state-schema) →
coaching turn → streamed advice rendered.

## Non-goals

Cadence/change detection; overlay.

## Acceptance criteria

- Button and hotkey both produce advice in the dashboard against a live
  window, and against the replay provider in CI.
- Envelope binding: the shown advice references the triggering
  observation_id.
- Failure paths (no window bound, capture failed, provider down) render the
  documented states.

## Dependencies

#macos-capture-provider, #hsbg-detection-selection, #augur-rpc-extensions,
#coaching-turn (integrates), #desktop-shell-mvp (renders).

## Test plan

System test over the real socket with replay capture + trace-replay
provider (zeroclaw-eval pattern).

## Documentation impact

mvp.md path marked demonstrable.

## Security, privacy, and policy considerations

Hotkey is capture-trigger only; no input is ever sent to the game.
