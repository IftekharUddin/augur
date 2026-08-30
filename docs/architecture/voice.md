# Voice Architecture (Milestone 3)

Voice follows the visual/text MVP and must not block it. Product staging:
TTS of completed recommendations → push-to-talk STT → grounded spoken Q&A →
interruption/cancellation → mute/volume → (much later, gated) wake word.

## What ZeroClaw already provides (verified)

- **TTS providers** (`crates/zeroclaw-channels/src/tts.rs`, 5 impls:
  OpenAI-compatible, ElevenLabs, Google, Edge, Piper-local) behind a
  `TtsProvider` trait; per-agent selection via `agents.<alias>.tts_provider`;
  cost tables in config.
- **STT providers** (`crates/zeroclaw-channels/src/transcription.rs`, 6 impls:
  Groq Whisper, OpenAI Whisper, Deepgram, AssemblyAI, Google, local
  whisper.cpp server) behind `TranscriptionProvider`.
- **Microphone capture** exists as a pattern (`voice_wake.rs`, `cpal`-based,
  feature-gated off) — but it is server-side and wake-word oriented.
- **Limits to plan around**: all providers are file-upload request/response —
  no streaming STT or TTS; `zeroclaw-api/src/vad.rs` has only `NoopVad`; the
  gateway `voice_duplex` path is a logging stub behind a non-default feature;
  there is no audio playback anywhere; the Tauri app has zero audio code.

## Augur voice design

- **Capture and playback live in the desktop app** (`apps/augur-desktop`):
  push-to-talk hotkey opens a `cpal` capture stream; audio is sent over the
  local RPC socket (`augur/voice/utterance`, bytes chunked under the 8MiB
  frame limit) to the runtime, which invokes the configured
  `TranscriptionProvider`. Playback: TTS bytes returned over RPC, played by
  the desktop audio output. Rationale: microphone permission, device
  selection, and low-latency playback are desktop concerns; provider I/O and
  keys stay in the runtime.
- **Provider replaceability** is inherited: Augur adds no provider code, only
  orchestration (`augur-voice`).
- **Turn semantics**: a spoken question is a steering/user message on the
  active coaching session, answered from current match state + retrieved
  strategy (no second contradictory recommendation unless state changed —
  the answer references the standing recommendation). Interruption maps to
  upstream `session/cancel`; barge-in during playback stops TTS locally then
  cancels the turn.
- **Mute/PTT state** is desktop-owned, shown in tray and main window; the
  microphone-active indicator is mandatory whenever a stream is open.
- **Privacy defaults**: audio never retained; transcripts not retained unless
  explicitly enabled; both stated in onboarding. Voice diagnostics exclude
  raw audio/transcripts without a separate opt-in.
- **Wake word** stays out of scope until privacy, false-activation, and
  platform review complete; the upstream RMS+substring approach is not
  shippable as-is.

## Latency

Upstream's own budget table (`docs/book/src/channels/voice.md`) puts local
Whisper STT at 300–800ms and TTS first-audio at 200–700ms; with
request/response providers, first spoken audio for a short answer lands
roughly 1.5–3s after PTT release. Acceptable for M3 v1; streaming providers
are the recorded follow-up if evaluation says otherwise.
