# ZeroClaw Reuse Audit

Audit of the ZeroClaw workspace (commit `4d47d7955d`, v0.8.4) for reuse in
Augur. Every claim cites a concrete repository path. Categories:

- **Reuse** — used unchanged.
- **Extend upstream** — generic improvement Augur needs; propose to ZeroClaw.
- **Wrap/fork** — Augur-only wrapper or fork-local change.
- **Exclude** — not used by Augur.

## Reuse matrix

| ZeroClaw component | Relevant paths | Current behavior | Reuse unchanged | Extend upstream | Augur wrapper / fork change | Risks | Recommended owner |
|---|---|---|---|---|---|---|---|
| Agent loop / turn engine | `crates/zeroclaw-runtime/src/agent/turn/mod.rs` (`run_tool_call_loop`), `agent/loop_.rs` | Iterative tool-call loop; budgets, approval gate, steering, streaming | ✅ | — | Coaching turns configured via runtime profile | Large file (`loop_.rs` ~682KB) churns upstream; sync via RPC boundary insulates us | platform |
| Local RPC (desktop IPC) | `crates/zeroclaw-runtime/src/rpc/` (`local.rs`, `dispatch.rs`, `types.rs`), `docs/book/src/architecture/rpc-socket.md` | JSON-RPC 2.0/NDJSON over unix socket (0600) / named pipe; 88 methods; streamed `session/update` | ✅ core | Method-registration seam so forks add methods without patching `dispatch.rs` | `augur/*` methods (observation push, advice stream, adapter status) | Fork-local dispatch patch until upstream seam lands | platform |
| RPC-only client precedent | `apps/zerocode`, `scripts/ci/zerocode_no_zeroclaw_dep_gate.sh` | TUI client, CI-gated to zero `zeroclaw-*` deps | pattern ✅ | — | Copy gate for `apps/augur-desktop` | — | desktop |
| Model providers | `crates/zeroclaw-providers` (`factory.rs`, `reliable.rs`, `router.rs`) | Factory-selected providers; retries, fallback chains, cooldowns | ✅ | — | — | — | platform |
| Vision / multimodal | `crates/zeroclaw-providers/src/multimodal.rs`, `agent/turn/vision_route.rs` | `[IMAGE:<path>]` markers → base64 → provider content blocks; per-alias `vision` override; separate vision-provider routing | ✅ mechanism | Tool-result image support is provider-uneven (works: `anthropic.rs:694`, `compatible.rs:2458`; dropped: gemini/ollama/openrouter/copilot/codex/bedrock) — document + widen upstream | Coach attaches frames to the observation turn (user-role path) for portability | Provider drift in image handling; only newest tool round keeps images (`multimodal.rs:593/646`) | platform |
| Multimodal budgets | `MultimodalConfig` (`zeroclaw-config/src/schema.rs`, `max_images=4`, `max_image_size_mb=5`) | Image count/size caps, stale-image stripping | ✅ | — | Tuned `[multimodal]` defaults for coaching | Default caps too small for frame streams; must tune, not assume | platform |
| Screenshot tool | `crates/zeroclaw-tools/src/screenshot.rs` | Shells out to `screencapture`/`scrot`; **no Windows**, no window enumeration; emits raw data URI, not `[IMAGE:]` | ❌ | Fix marker emission (`image_info.rs:240` is the correct pattern) | Replaced by `augur-capture` (native, window-scoped) | Anthropic sweeps raw data URIs (`anthropic.rs:772`) making tool's base64 dead weight | capture |
| Tool system | `zeroclaw-api/src/tool.rs` (`Tool`, `ToolOutput{text,data}`, `output_schema()`), `runtime/src/tools/scoped.rs` (`ScopedToolRegistry::assemble`) | Sealed registry; structured output; policy filters | ✅ | — | Augur tools (strategy search, observation) fed via `all_tools()` | Registry is sealed by design — integrate at assembly, don't bypass | platform |
| Sessions | `crates/zeroclaw-infra/src/session_sqlite.rs` (SQLite + FTS5) | Per-session persistence, metadata, search | ✅ | — | Match sessions layered on top (own store keyed by `MatchIdentity`) | — | platform |
| Memory | `crates/zeroclaw-memory` | Backends, embeddings, MMR rerank | partial | — | **Not** used for match state (explicit product rule); embeddings reusable for later hybrid retrieval | Long-term memory must not substitute for explicit match state | platform |
| Retrieval precedent | `crates/zeroclaw-runtime/src/rag/mod.rs` (hardware datasheets) | Keyword-keyed markdown chunker | pattern only | Generalization possible later | Forked into `augur-strategy` (decision 0003) | `knowledge_bundles` config is dead (nothing reads `.sources`) — do not build on it | strategy |
| Config & secrets | `crates/zeroclaw-config` (`schema.rs`, `secrets.rs`: ChaCha20-Poly1305 `enc2:`, 1Password refs) | Typed TOML schema, encrypted secrets, env overrides | ✅ | — | `[augur]` config tables via `Configurable` derive; runtime-profile placement rule respected | Schema file is huge; add fields following upstream conventions exactly | platform |
| Gateway | `crates/zeroclaw-gateway` (~120 routes, pairing tokens, WS/SSE) | HTTP/WS surface + React dashboard | ❌ for desktop | — | Not the desktop path (decision 0001); may serve advanced/remote later | Exposing a large surface for a small need | — |
| Daemon lifecycle | `crates/zeroclaw-runtime/src/daemon/mod.rs`, `service/mod.rs` (launchd/systemd/OpenRC) | Supervised startup, readiness, reload, OS services | ✅ | Daemon lifecycle API polish | Sidecar spawn from desktop (pattern exists: `apps/tauri/src/daemon.rs`) | No PID file (socket+lock is the contract) | platform |
| Tauri shell | `apps/tauri` (1.5k lines: splash, tray, sidecar spawn, HTTP proxy commands) | Thin launcher; dashboard = remote webview, **no IPC grant** (enforced by `apps/tauri/tests/capability_security.rs`) | partial | — | `apps/augur-desktop` is a new app with a real frontend; reuse tray/single-instance/sidecar-spawn/permission FFI patterns | Upstream shell's HTTP+localStorage token handoff not suitable for coaching UI | desktop |
| macOS permission FFI | `apps/tauri/src/macos/permissions.rs` (305 lines: screen recording, accessibility, input monitoring, mic, camera, speech, FDA, automation), `apps/tauri/Info.plist` (6 usage strings) | Written and compiling; 11 of 12 functions unwired | ✅ (wire it) | Could upstream as a Tauri permission helper crate | Wired into Augur onboarding | Entitlements absent repo-wide; needed for hardened runtime | desktop |
| Overlay / hotkeys | — (verified absent: no `always_on_top`/`transparent`/global-shortcut hits) | Do not exist | ❌ | — | Built new in `apps/augur-desktop` (+ `tauri-plugin-global-shortcut`) | Overlay capture-exclusion is platform-specific work | desktop |
| Voice: TTS/STT providers | `crates/zeroclaw-channels/src/tts.rs` (OpenAI/ElevenLabs/Google/Edge/Piper), `transcription.rs` (Groq/Whisper/Deepgram/AssemblyAI/Google/LocalWhisper) | Server-side, file-based, provider-replaceable | ✅ | — | `augur-voice` orchestrates; client capture/playback new | No streaming STT/TTS; request/response only (latency budget impact, M3) | voice |
| Mic capture / wake | `crates/zeroclaw-channels/src/voice_wake.rs` (cpal), `zeroclaw-api/src/vad.rs` (`NoopVad` only) | RMS-energy VAD + substring wake word; feature-gated off | pattern | — | Push-to-talk capture in desktop; wake-word deferred (product rule) | No real VAD exists; `voice_duplex` gateway path is a logging stub | voice |
| Eval harness | `crates/zeroclaw-eval` (`LlmTrace` replay, declarative `expects`, `Grader` trait) | Deterministic replay of real agent loop; CI-gateable | ✅ | Property graders | Recommendation eval harness built on `Grader` seam | Phase-0 only; does not measure model quality by itself | platform |
| Logging | `crates/zeroclaw-log` (`record!`-only rule, tool-I/O redaction default-on, LLM payload capture default-off, ephemeral-credential quarantine) | JSONL + broadcast + stderr; leak detector | ✅ | — | Augur events through `record!`; frame paths in `log_tool_io_denylist` | Clippy `disallowed_macros` bans raw `tracing` — comply | platform |
| Testing conventions | `tests/{component,integration,system,live}`, `tests/test_architecture.rs` | 5-level taxonomy + architecture-invariant greps | ✅ | — | Augur invariants added (game-isolation, RPC-only desktop) | — | platform |
| Release pipeline | `.github/workflows/release-stable-manual.yml`, `scripts/desktop/prepare-kernel.sh` | Signed+notarized+stapled macOS DMG; sidecar staging; Windows/Linux `continue-on-error`, unsigned | ✅ pattern | — | Augur identifiers/branding; Windows signing is new work; **no auto-updater exists** (no `tauri-plugin-updater` anywhere) | Cannot claim update security until built | release |
| CI | `.github/workflows/ci.yml` (25-job `CI Required Gate`), self-hosted `blacksmith-*` runners | Heavy required gate | ❌ as-is | — | Forked CI: hosted runners, Augur gate set; upstream workflows disabled at fork time | Workflows reference runners the fork does not have — must adapt before enabling Actions | release |
| Plugins (WASM) | `crates/zeroclaw-plugins`, `wit/v0` (unstable, `@unstable` feature-gated) | wasmtime component host | ❌ initially | — | Excluded for game adapters (decision 0002) | ABI churn; signature does not cover `.wasm` | — |
| Channels (Telegram etc.) | `crates/zeroclaw-channels` (except tts/transcription/voice files) | Messaging platforms | ❌ | — | Excluded from Augur product surface | Compile-time feature trimming to keep binary lean | — |
| Hardware | `crates/zeroclaw-hardware` | Serial/GPIO | ❌ | — | Excluded | — | — |
| Licensing | `LICENSE-MIT`, `LICENSE-APACHE`, `NOTICE` (dual MIT OR Apache-2.0; no per-file headers) | Root-file licensing | ✅ obligations | — | Augur NOTICE keeps ZeroClaw attribution, removes ZeroClaw marks per upstream trademark claim; upstream NOTICE's dangling `TRADEMARK.md`/`CLA.md` refs not inherited | Apache §4(d) NOTICE propagation; trademark policy | core |

## Verified absences that shape the plan

- **No native window capture anywhere** (zero hits for `ScreenCaptureKit`,
  `CGWindowList`, `EnumWindows`); no window enumeration; no Windows screenshot
  support.
- **No overlay primitives, no global hotkeys, no auto-updater, no client
  audio.**
- **No generic markdown-corpus retrieval**; `knowledge_bundles`,
  `context_compression`, and `history_pruning` config are declared but dead.
- **No streaming STT/TTS**; gateway `voice_duplex` is a stub behind a
  non-default feature.
- **No snapshot testing**; the eval story is trace replay + declarative
  expectations.

## Candidate upstream contributions

Ordered by expected upstream appetite (see [upstream-sync.md](upstream-sync.md)):

1. `screenshot` tool emits `[IMAGE:]` marker instead of raw data URI
   (`crates/zeroclaw-tools/src/screenshot.rs` vs the correct
   `image_info.rs:240` pattern).
2. RPC method-registration seam for downstream method namespaces.
3. Uniform tool-result image handling across providers (or documented
   capability flag).
4. Tauri macOS permission FFI as a reusable helper.
5. Daemon lifecycle API polish (typed readiness for embedders).
6. Provider capability reporting: per-model vision detection.
