# Runtime Integration

How the Augur desktop application integrates the ZeroClaw runtime. The
decision itself is recorded in
[decision 0001](../decisions/0001-runtime-integration.md); this document
covers mechanics.

## Topology

- The Augur desktop app bundles the runtime kernel as a Tauri sidecar
  (`externalBin`), staged by the upstream pattern in
  `scripts/desktop/prepare-kernel.sh` and `apps/tauri/tauri.bundled.conf.json`.
- On launch, the desktop app checks for a live socket, spawns the daemon if
  needed (detached, logs to a temp file — pattern:
  `apps/tauri/src/daemon.rs`), and connects to the local JSON-RPC endpoint:
  Unix `<data_dir>/daemon.sock` (mode `0600`), Windows
  `\\.\pipe\zeroclaw-<hash>` (endpoint resolution:
  `crates/zeroclaw-runtime/src/rpc/local.rs`).
- All UI↔runtime traffic is JSON-RPC 2.0 over NDJSON (`MAX_FRAME_BYTES` 8MiB),
  with the mandatory `initialize` handshake (protocol version 1, mismatch →
  `-32011`).

## Authentication

None beyond the OS: socket permissions (0600 + 0700 parent, `O_NOFOLLOW`,
advisory lock lifecycle) on Unix, default creating-user+SYSTEM ACL on the
Windows named pipe. This is upstream's documented contract
(`docs/book/src/architecture/rpc-socket.md`). No pairing token, no listening
TCP port for local coaching. The gateway (HTTP + pairing tokens) is not part
of the desktop path.

## Streaming

Coaching output rides upstream's `session/update` notification stream
(`SessionUpdateEvent`, `crates/zeroclaw-runtime/src/rpc/types.rs`):
`agent_message_chunk`, `agent_thought_chunk`, `tool_call`, `tool_result`,
`approval_request`, `context_usage`, `plan`, `turn_complete`,
`history_trimmed`. The UI renders chunks live and uses `turn_complete` to
finalize a recommendation card.

## Augur RPC extensions (`augur/*`)

New methods registered by the Augur runtime layer (fork-local patch to
`rpc/dispatch.rs` until an upstream registration seam exists):

| Method | Direction | Purpose |
|---|---|---|
| `augur/status` | req | Adapter registry, active game, capture state, pack versions |
| `augur/games/list` | req | Manifests + support/policy status |
| `augur/capture/windows` | req | Enumerated capturable windows |
| `augur/capture/select` | req | Bind capture to a window |
| `augur/capture/start` / `stop` | req | Session control (M2 continuous mode) |
| `augur/observe/once` | req | Manual observation trigger (M1 path) |
| `augur/coach/subscribe` | notify | Recommendation + invalidation stream |
| `augur/strategy/packs` | req | Active pack, versions, validation status |
| `augur/feedback/submit` | req | Incorrect-advice report bound to observation id |

Notifications reuse the `session/update` channel where possible; coaching
events that are not agent-turn events (e.g. `advice_invalidated`,
`capture_state_changed`) get an `augur/event` notification.

## Sidecar vs embedded: revisit criteria

Embedding `zeroclaw-runtime` in-process would be reconsidered only if (a) RPC
round-trip latency proves material against the [performance budgets](../product/mvp.md)
— unlikely: budgets are dominated by model inference — or (b) upstream breaks
the socket transport contract. Either would be a new decision record.

## Version compatibility

The desktop app records the kernel's `initialize` protocol version and the
daemon's reported version (`status` RPC). A mismatch outside the supported
window renders an explicit UI state (never a silent degradation) and links to
the upgrade path. Kernel and desktop app ship from the same tag; mixed
versions are supported one minor back, matching the fork's release cadence
(see [upstream-sync.md](upstream-sync.md)).
