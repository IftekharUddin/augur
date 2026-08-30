## Problem

No overlay primitives exist in the tree (verified: zero
always-on-top/transparency/click-through usage). M2 needs the optional
always-on-top advice overlay, excluded from capture.

## Context and repository evidence

user-experience.md overlay requirements (transparent, click-through,
movable/anchorable, 1–3 actions, stale indicator, refs on demand, hotkey
collapse, policy auto-disable); capture-and-observation.md exclusion
(macOS `NSWindow.sharingType = .none`; Windows
`SetWindowDisplayAffinity(WDA_EXCLUDEFROMCAPTURE)`).

## Scope

Second Tauri window: transparent, undecorated, always-on-top, click-through
default with an interaction mode; anchor/position persistence; renders the
standing recommendation + confidence + stale state; capture-exclusion flags
both platforms; readability pass over light/dark scenes (contrast tokens);
auto-disable wiring from game policy status.

## Non-goals

In-overlay configuration; multi-monitor anchoring polish beyond remembering
per-display position.

## Acceptance criteria

- Capture-with-overlay fixture proves exclusion on macOS and Windows (M2
  exit criterion).
- Click-through verified (game receives clicks under the overlay).
- Accessibility: text scales; contrast checked on light/dark fixture
  scenes; hotkey collapse works.

## Dependencies

#desktop-shell-mvp, #advice-invalidation (stale events),
#windows-capture-provider + #macos-capture-provider (exclusion testing).

## Test plan

Capture-exclusion fixtures; manual click-through matrix documented.

## Documentation impact

user-experience.md overlay implemented.

## Security, privacy, and policy considerations

Overlay must not contaminate observations (exclusion is a correctness AND
privacy property); auto-disable respects per-game policy status.
