# Capture and Observation Pipeline

The reusable service that turns a game window into validated observations.
ZeroClaw provides none of this natively (verified: no `ScreenCaptureKit` /
`CGWindowList` / `EnumWindows` hits anywhere; `crates/zeroclaw-tools/src/screenshot.rs`
shells out to `screencapture`/`scrot` and returns "not supported" on Windows),
so `augur-capture` and `augur-observation` are new crates.

## Pipeline

```text
window detection → window selection → platform capture → crop/normalize
  → frame hash + change detection → optional region extraction
  → vision/state extraction → confidence checks → session state merge
  → strategy query → coaching request → recommendation validation
  → GUI / overlay / TTS
```

## Platform capture

| Platform | API | Notes |
|---|---|---|
| macOS | **ScreenCaptureKit** (`SCShareableContent`, `SCContentFilter` per-window, `SCStream`) | Window-scoped capture excludes other windows by construction; requires Screen Recording TCC permission — check/request FFI already exists upstream (`apps/tauri/src/macos/permissions.rs`: `CGPreflightScreenCaptureAccess` / `CGRequestScreenCaptureAccess`) and `NSScreenCaptureUsageDescription` is already declared in `apps/tauri/Info.plist` |
| Windows | **Windows.Graphics.Capture** (`GraphicsCaptureItem` per-window) | Per-window capture; OS may draw a yellow border (accepted); fallback `BitBlt`/`PrintWindow` for windowed-mode edge cases |
| Linux | Deferred | Not in scope for M1/M2; tracked as a community milestone-4+ item |

`CaptureProvider` trait: `enumerate_windows`, `request_permission`,
`capture(target, options) -> CapturedFrame`. One implementation per platform,
selected at compile time; a fixture-backed `ReplayCaptureProvider` powers
tests and the second-game proof.

Window-scoped capture (not display capture) is the default and the privacy
baseline: notifications and unrelated windows are structurally excluded.

## Overlay exclusion

The Augur overlay must not appear in captured frames:

- macOS: `NSWindow.sharingType = .none` — excluded from ScreenCaptureKit.
- Windows: `SetWindowDisplayAffinity(WDA_EXCLUDEFROMCAPTURE)`.
- Both are per-window flags on the overlay; a fixture test captures with the
  overlay open and asserts absence (M2 exit criterion).

## Normalization

- DPI/Retina: capture at native pixels, record the scale factor, downscale to
  the adapter's target resolution band.
- Resolution/mode changes (fullscreen ↔ borderless ↔ windowed): re-run
  capture-profile negotiation; a mid-match change invalidates in-flight
  observations.
- Multi-monitor: capture is window-bound, so monitor topology only affects
  enumeration metadata and overlay anchoring.
- Output format: PNG for provider delivery (`PROVIDER_IMAGE_MIME_TYPES` in
  `zeroclaw-api/src/media.rs` accepts png/jpeg/webp/gif), JPEG/webp downscale
  where size budgets demand (`[multimodal] max_image_size_mb` default 5).

## Cadence and change detection

A fixed 1–2s capture loop is explicitly **not** the design. Triggers:

- **M1 (manual)**: user hotkey / button captures exactly one observation.
- **M2 (live)**: adaptive cadence — a cheap capture+hash loop (perceptual hash
  over downscaled luma; frame SHA-256 recorded in the envelope) runs at a few
  Hz; extraction and model calls fire only on material change or
  phase-transition heuristics from the adapter's capture profile. Cooldowns
  cap provider spend; the metrics ledger records frames captured vs frames
  discarded before model invocation.

## Entering the vision pipeline

Frames enter ZeroClaw as `[IMAGE:<absolute path>]` markers on the coaching
turn's user-role message — the portable path across providers. (Tool-result
image delivery works only on Anthropic (`anthropic.rs:694`) and the
OpenAI-compatible adapter (`compatible.rs:2458`), and only for the newest tool
round; Gemini/Ollama/OpenRouter/Copilot/Codex/Bedrock drop tool-result images
— verified per provider.) Temp frame files are content-addressed, deleted
after the turn, and never persisted by default. Stale-image hygiene is
upstream behavior we rely on: older tool-result images are stripped on every
provider prep (`multimodal.rs:593/646`) and image count is capped
(`max_images`).

## State extraction

Staged comparison (full matrix in the M1 research issue):

1. **Full-frame vision model + structured-state request** — MVP baseline.
   Highest flexibility, highest latency/cost, robust across resolutions.
2. **Deterministic OCR/template for stable UI regions** (gold, health, tier
   numerals) — added where the adapter's fixtures show reliable regions; cheap
   and testable, brittle across patches.
3. **Local specialized model** — deferred; revisit with evaluation data.
4. **Permitted game logs** — Hearthstone's local `Power.log`/`Zone.log` are
   used by established deckers; policy review (M0) gates any use.
5. **Hybrid with confidence escalation** — target end-state: deterministic
   first, vision for the rest, escalation on low confidence.

Extraction produces per-field confidence and evidence references; missing
fields remain missing — the schema forbids invented values, and the
recommendation prompt requires stating uncertainty.

## Failure handling

Minimized window, occlusion, capture API failure, and permission revocation
each map to explicit pipeline states surfaced in the UI (see
[user-experience.md](../product/user-experience.md)); the session store marks
gaps rather than fabricating continuity. Memory bounds: frame buffers are
pooled and capped; temp files cleaned on turn end and on startup sweep.
