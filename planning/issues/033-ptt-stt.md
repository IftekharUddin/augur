## Problem

Push-to-talk transcription: hold hotkey → capture mic → transcribe →
show text. Upstream mic capture exists only server-side (voice_wake, cpal,
feature-gated) and all 6 STT providers are file-upload request/response.

## Context and repository evidence

voice.md (capture in desktop, providers in runtime, PTT default,
mandatory mic indicator); `crates/zeroclaw-channels/src/transcription.rs`
(Groq default, OpenAI, Deepgram, AssemblyAI, Google, LocalWhisper);
`voice_wake.rs` cpal patterns (12 sample formats → mono f32); macOS mic
permission FFI + `NSMicrophoneUsageDescription` already present upstream.

## Scope

Desktop: PTT hotkey (global-shortcut plugin), cpal capture to WAV/opus
buffer, mic-active indicator (tray + window), device selection; runtime:
`augur/voice/utterance` RPC → `TranscriptionProvider` → transcript back;
permission flow wiring (macOS FFI, Windows prompt).

## Non-goals

Wake word (explicitly deferred per product rule); VAD (PTT bounds the
utterance); conversation semantics (next issue).

## Acceptance criteria

- Hold-to-talk produces a transcript in the UI with two providers (one
  hosted, LocalWhisper).
- Indicator provably tracks stream state; release stops capture instantly.
- Denied-mic path renders the documented degraded state.

## Dependencies

#tts-recommendations (shared voice plumbing), #augur-rpc-extensions.

## Test plan

Recorded-audio fixtures through the RPC path; indicator state tests;
live mic smoke.

## Documentation impact

voice.md PTT stage implemented.

## Security, privacy, and policy considerations

Audio buffers dropped after transcription; transcripts not retained unless
opted in; mic indicator is non-negotiable UI.
