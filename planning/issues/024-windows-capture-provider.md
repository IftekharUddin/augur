## Problem

No Windows capture exists (inherited screenshot tool returns "not supported
on this platform"). M2 makes Augur cross-platform.

## Context and repository evidence

capture-and-observation.md platform table (Windows.Graphics.Capture,
GraphicsCaptureItem per-window; BitBlt/PrintWindow fallback); upstream
Windows precedents: named-pipe RPC (`rpc/local.rs`), `apps/tauri/windows/app.manifest`
(PerMonitorV2 DPI already declared).

## Scope

`augur-capture` Windows `CaptureProvider`: enumeration, per-window capture,
DPI-aware normalization, permission model (none beyond OS prompts; yellow
capture border accepted and documented), parity with the macOS provider's
typed failure states.

## Non-goals

Overlay exclusion implementation detail beyond `WDA_EXCLUDEFROMCAPTURE`
wiring (lands with #overlay); Linux.

## Acceptance criteria

- Same fixture/behavior suite as macOS provider passes on Windows CI
  (hosted runner) for logic; live-marked tests for real capture.
- Windowed/borderless/fullscreen-exclusive behaviors documented with
  observed results (exclusive fullscreen may require documented fallback).

## Dependencies

#macos-capture-provider (shared trait + suite), #ci-adaptation (Windows
runner lane).

## Test plan

Shared provider conformance suite; DPI matrix; mode matrix.

## Documentation impact

Platform table row implemented.

## Security, privacy, and policy considerations

Window-scoped capture parity; no display capture.
