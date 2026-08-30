## Problem

No window-scoped capture exists anywhere in the inherited tree (verified: no
ScreenCaptureKit/CGWindowList usage; `zeroclaw-tools/src/screenshot.rs` shells
out to `screencapture` and can't target a window programmatically or run on
Windows). M1 is macOS-first.

## Context and repository evidence

- docs/architecture/capture-and-observation.md — API choice
  (ScreenCaptureKit `SCContentFilter` per-window), normalization rules.
- Permission FFI already exists unwired:
  `apps/tauri/src/macos/permissions.rs` (`CGPreflightScreenCaptureAccess` /
  `CGRequestScreenCaptureAccess`); usage string already in
  `apps/tauri/Info.plist`.

## Scope

`augur-capture` macOS implementation of `CaptureProvider`:
`enumerate_windows`, `request_permission` (reusing the FFI pattern),
single-frame `capture(target)` returning a normalized `CapturedFrame`
(native pixels + scale factor + downscale to the adapter band, PNG encode,
SHA-256). Plus the fixture-backed `ReplayCaptureProvider` for tests.

## Non-goals

Continuous streams/cadence (M2), Windows (M2), Linux (deferred), overlay
exclusion (M2 — but `CapturedFrame` carries the fields it needs).

## Proposed approach

`objc2`-based ScreenCaptureKit binding (upstream precedent for objc2 use in
`apps/tauri`); single-shot `SCScreenshotManager` path first, `SCStream`
deferred to M2.

## Acceptance criteria

- Enumerates on-screen windows with title/app/pid/frame on macOS 13+.
- Captures exactly one chosen window (other windows/notifications provably
  absent — fixture assertion with a staged second window).
- Denied-permission path returns the typed state the UX doc expects.
- Retina + non-Retina normalization verified (DPI test matrix).

## Dependencies

#augur-crate-skeleton.

## Test plan

Live-marked integration tests (upstream `tests/live` convention) + replay
provider unit tests; DPI matrix fixtures.

## Documentation impact

capture-and-observation.md platform table marked implemented (macOS).

## Security, privacy, and policy considerations

Window-scoped capture is the privacy baseline; frames transient; TCC
permission flow honest (no silent retries).
