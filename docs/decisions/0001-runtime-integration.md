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

## Sources: measured latency

The objection this decision has to answer is latency: a coaching interface is
interactive, and an inter-process hop is not free. Answered with numbers rather
than argument.

`scripts/dev/augur_rpc_latency_probe.py` starts a real `zeroclaw daemon`
against a throwaway configuration directory and drives it from a
dependency-free NDJSON client. Every figure is a full client-observed round
trip: bytes written, bytes read back, JSON parsed, nothing subtracted. The
kernel is built with the `ci` profile (release with thin LTO); an unoptimized
build would measure rustc rather than architecture. Reproduce with:

```bash
cargo build --locked --profile ci --bin zeroclaw
python3 scripts/dev/augur_rpc_latency_probe.py --binary target/ci/zeroclaw --iterations 200
```

The `Augur RPC Latency Probe` workflow runs the same probe on `macos-latest`,
`windows-latest`, and `ubuntu-latest`, so the Windows named-pipe transport is
measured rather than extrapolated from the Unix socket. They carry identical
NDJSON framing but are different kernel objects.

### macOS, Unix domain socket

Developer machine, macOS 15 arm64, 10 cores. 200 iterations, each opening a
fresh connection.

| Operation | n | min (ms) | median (ms) | p95 (ms) | max (ms) |
|---|---:|---:|---:|---:|---:|
| connect (fresh socket) | 200 | 0.009 | 0.027 | 0.220 | 1.681 |
| initialize (handshake) | 200 | 0.064 | 0.263 | 2.461 | 9.667 |
| status (first on connection) | 200 | 0.037 | 0.137 | 2.141 | 9.831 |
| status (warm connection) | 200 | 0.032 | 0.100 | 3.220 | 10.880 |

Cold path, connect plus initialize plus status: **0.427 ms median**.

### Hosted runners, all three transports

Pending the first run of the `Augur RPC Latency Probe` workflow. This section
is deliberately left explicit rather than omitted: the Windows named-pipe
figure is a stated gap in the evidence until it is filled, and the record
should not read as though all platforms were measured when one was not.

### Reading these against the budgets

`docs/product/mvp.md` sets end-to-end manual-trigger-to-advice at 5s p50 and 9s
p95, with the tightest per-stage budget, validation and publish, at 20ms and
50ms.

The cold path a desktop client pays once at startup, connect plus initialize
plus status, is a fraction of a millisecond at the median. A warm connection,
which is what a running application actually has, answers a query in about a
tenth of a millisecond. Against the p50 end-to-end budget that is roughly one
ten-thousandth; against the tightest single stage, a few percent. The pipeline
is dominated by a vision model call measured in seconds, and no plausible
reading of these numbers changes that by three orders of magnitude.

### What is deliberately not measured here

**Streamed chunk latency end to end.** Measuring it needs a live model
provider, which means credentials and spend. What the *transport* contributes
is bounded by what is measured above: a `session/update` notification is one
direction of a frame whose full round trip, request plus dispatch plus
response, is about a tenth of a millisecond warm. Per-chunk transport overhead
is therefore well under a tenth of a millisecond against a provider streaming
tokens tens of milliseconds apart. That is an inference from the transport
measurement, not a measurement of streaming, and it is recorded as such.

### Revisit criteria, restated as thresholds

This decision is worth reopening if the measured warm round trip exceeds 5ms at
p95 on any supported platform, which would make it a visible fraction of the
validation-and-publish budget, or if upstream changes the socket transport
contract. Neither is true today.
