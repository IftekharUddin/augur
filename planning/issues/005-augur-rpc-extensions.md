## Problem

The desktop app needs Augur-specific RPC methods (`augur/status`,
`augur/capture/*`, `augur/observe/once`, `augur/coach/subscribe`,
`augur/strategy/packs`, `augur/feedback/submit`, `augur/event`) that upstream
`rpc/dispatch.rs` doesn't know about.

## Context and repository evidence

- Method table: docs/architecture/runtime-integration.md.
- Upstream single source of truth: `Method::ALL`
  (`crates/zeroclaw-runtime/src/rpc/dispatch.rs`); notifications:
  `SessionUpdateEvent` (`rpc/types.rs`); NDJSON framing, 8MiB cap
  (`rpc/local.rs`).
- Fork-local patch is expected until an upstream registration seam exists
  (upstream candidate #2 in zeroclaw-reuse-audit.md).

## Scope

Implement the `augur/*` method set returning honest stub data where
subsystems don't exist yet (adapter registry from #augur-crate-skeleton;
capture returns "unimplemented" states); `augur/event` notification channel;
protocol documentation.

## Non-goals

Capture/observation behavior (M1); upstream PR for the seam (tracked in the
issue as a follow-up once the shape stabilizes).

## Proposed approach

Minimal dispatch patch, isolated in one clearly-marked module to keep
upstream merges cheap; wire types in `augur-core`.

## Acceptance criteria

- A test client can list games, read status, and receive an `augur/event`
  over the real socket on macOS and Windows.
- Fork-touchpoints doc lists the dispatch patch.

## Dependencies

#ratify-decision-0001, #augur-crate-skeleton.

## Test plan

Component tests over an in-process transport; one system test over the real
socket.

## Documentation impact

runtime-integration.md method table marked implemented.

## Security, privacy, and policy considerations

Same OS-permission trust model as upstream socket; no new listeners.
