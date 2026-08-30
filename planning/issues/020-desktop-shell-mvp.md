## Problem

`apps/augur-desktop` doesn't exist. The inherited Tauri app is a
splash+tray launcher whose dashboard is a remote webview with zero IPC
(enforced by `apps/tauri/tests/capability_security.rs`) — not a base for a
coaching UI.

## Context and repository evidence

- user-experience.md (onboarding + main window inventory);
  decision 0001 (RPC-only; dep gate from #augur-crate-skeleton).
- Reusable upstream patterns: sidecar spawn (`apps/tauri/src/daemon.rs`),
  single-instance + tray (`apps/tauri/src/{lib.rs,tray/}`), macOS permission
  FFI (`apps/tauri/src/macos/permissions.rs`), Info.plist usage strings.
- Frontend stack precedent: `web/` (React 19 + Vite + Tailwind + TS) — the
  gateway dashboard; Augur's frontend is its own app in-repo.

## Scope

New Tauri app: onboarding flow (permissions wiring the existing FFI,
provider setup incl. advanced ZeroClaw aliases, connectivity test, pack
readiness, detection test, no-control pledge); main window (detection
status, window picker, start/stop, recommendation card with evidence split +
citations, advice history, degraded-state banners, diagnostics export);
tray; RPC client (NDJSON socket) in the Tauri Rust core with typed events to
the frontend.

## Non-goals

Overlay (M2), voice UI (M3), auto-update.

## Acceptance criteria

- Dep gate green (no zeroclaw-*/augur-runtime linkage).
- Onboarding completes on a clean macOS account (screen-recording flow
  real); every degraded state in user-experience.md renderable via a dev
  state-injector.
- Manual-trigger path renders live advice end-to-end.

## Dependencies

#ratify-decision-0001, #augur-rpc-extensions; integrates
#hsbg-detection-selection, #manual-observation-trigger, #coaching-turn.

## Test plan

Frontend logic tests (upstream `web/` node-test conventions); capability
test copied from upstream pattern; scripted degraded-state walkthrough.

## Documentation impact

user-experience.md marked implemented for MVP scope.

## Security, privacy, and policy considerations

Webview gets no broad IPC — commands scoped per upstream capability
precedent; provider keys never enter the frontend.
