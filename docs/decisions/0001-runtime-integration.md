---
id: 0001
title: Sidecar daemon over local JSON-RPC, not library embedding
date: 2026-08-29
status: proposed
relates-to:
  - crates/zeroclaw-runtime/src/rpc/
  - apps/zerocode
  - docs/architecture/runtime-integration.md
---

# 0001 — Sidecar daemon over local JSON-RPC, not library embedding

## Context

Augur's desktop application must drive ZeroClaw agent sessions (streamed
responses, tool calls, approvals) and receive coaching output. Three candidate
integrations were investigated against the actual ZeroClaw source at
`4d47d7955d`:

1. **Embed `zeroclaw-runtime` as a library** inside the Tauri process.
2. **Bundled sidecar daemon** speaking the existing local JSON-RPC transport.
3. **HTTP/WS to the gateway** (what the current upstream Tauri shell does via
   `apps/tauri/src/gateway_client.rs`, hardcoded to `127.0.0.1:42617`).

Repository evidence:

- ZeroClaw ships a complete **JSON-RPC 2.0 over NDJSON** local IPC in
  `crates/zeroclaw-runtime/src/rpc/` — Unix domain socket
  (`<data_dir>/daemon.sock`, mode `0600`) / Windows named pipe
  (`\\.\pipe\zeroclaw-<hash>`), 88 methods (`rpc/dispatch.rs`, `Method::ALL`),
  and typed streaming `session/update` notifications
  (`rpc/types.rs` `SessionUpdateEvent`: `agent_message_chunk`,
  `agent_thought_chunk`, `tool_call`, `tool_result`, `approval_request`,
  `context_usage`, `plan`, `turn_complete`, `history_trimmed`).
- Access control is **OS-enforced** (socket permissions), no token handshake
  required (`docs/book/src/architecture/rpc-socket.md`).
- `apps/zerocode` (the TUI) is a working, **CI-enforced** RPC-only client:
  `scripts/ci/zerocode_no_zeroclaw_dep_gate.sh` fails the build if zerocode
  links any `zeroclaw-*` crate. This proves a rich streaming client needs no
  runtime linkage.
- The daemon (`crates/zeroclaw-runtime/src/daemon/mod.rs`) provides supervised
  lifecycle, readiness tracking, in-place reload, and OS service integration
  (`launchd`/`systemd`/OpenRC via `runtime/src/service/mod.rs`).
- The release pipeline already stages the kernel as a Tauri sidecar
  (`scripts/desktop/prepare-kernel.sh`, `tauri.bundled.conf.json`
  `externalBin`).
- The gateway (`crates/zeroclaw-gateway`) exposes ~120 HTTP routes plus
  pairing-token auth — a far larger surface than a coaching UI needs, and its
  bearer-token dance (`/admin/paircode/new` → `/pair`) exists precisely because
  HTTP has no OS-level caller identity.

## Decision

**Augur runs ZeroClaw as a bundled sidecar daemon and the desktop app talks to
it exclusively over the local JSON-RPC socket transport, following the zerocode
pattern.**

- `apps/augur-desktop` must not link `zeroclaw-*` or `augur-runtime` crates; a
  copied and renamed dependency gate script enforces this in CI.
- Augur-specific functionality (observation push, coaching stream, adapter
  status) is exposed as new namespaced RPC methods (`augur/*`) registered by
  the Augur runtime layer.
- The gateway remains available for advanced/remote scenarios but is not the
  desktop integration path.

## Consequences

- Crash isolation: a runtime panic cannot take down the UI, and vice versa.
- Upstream sync stays cheap: the boundary is a wire protocol, not internal
  APIs. Upstream refactors of `zeroclaw-runtime` internals do not break the
  desktop app.
- Security: no listening TCP port is required for local coaching; socket
  permissions replace token management.
- Cost: Augur must extend the RPC method table. Upstream candidate: a
  pluggable RPC method-registration seam so forks do not patch
  `rpc/dispatch.rs` directly.
- The existing upstream Tauri shell's HTTP path (`gateway_client.rs`) is not
  reused for coaching; parts of the shell (tray, sidecar spawn, macOS
  permission FFI in `apps/tauri/src/macos/permissions.rs`) are reused.
