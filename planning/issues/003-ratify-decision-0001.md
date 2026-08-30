## Problem

Decision 0001 (sidecar daemon over local JSON-RPC; desktop is RPC-only) is
`proposed`. Every desktop and runtime issue builds on it; it must be ratified
or amended with evidence before implementation starts.

## Context and repository evidence

- docs/decisions/0001-runtime-integration.md — full evidence: RPC surface
  (`crates/zeroclaw-runtime/src/rpc/`, 88 methods, `SessionUpdateEvent`
  streaming), zerocode's CI-enforced RPC-only precedent
  (`scripts/ci/zerocode_no_zeroclaw_dep_gate.sh`), OS-level socket auth
  (`docs/book/src/architecture/rpc-socket.md`), sidecar staging
  (`scripts/desktop/prepare-kernel.sh`).
- Alternatives (library embedding; gateway HTTP) and revisit criteria are in
  the record and docs/architecture/runtime-integration.md.

## Scope

Review the decision record against any objections raised on the planning PR;
prototype the smallest possible RPC round-trip (connect → `initialize` →
`status`) from a throwaway client to validate latency assumptions on macOS
and Windows; flip status to `accepted` (or write 0004 superseding it).

## Non-goals

Building the desktop app; adding `augur/*` methods (separate issue).

## Proposed approach

Timeboxed (2 days): measurement table (connect, initialize, status, streamed
chunk latency) attached to the issue; then the status-flip PR.

## Acceptance criteria

- Measurement table posted.
- 0001 status is `accepted` (or superseded by an accepted record).

## Dependencies

Planning PR merged.

## Test plan

The prototype IS the test; numbers recorded in the decision record's
Sources.

## Documentation impact

Decision record status; runtime-integration.md if amended.

## Security, privacy, and policy considerations

Socket permission model is upstream-documented; no new surface.
