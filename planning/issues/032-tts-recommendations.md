## Problem

Completed recommendations should be speakable. Upstream has 5 TTS providers
behind a trait but zero client-side playback anywhere (verified absence).

## Context and repository evidence

voice.md design (providers in runtime, playback in desktop);
`crates/zeroclaw-channels/src/tts.rs` (`TtsProvider`: OpenAI-compatible,
ElevenLabs, Google, Edge, Piper-local; per-agent `tts_provider` selection;
cost tables); upstream budget table (TTS first-audio 200–700ms,
`docs/book/src/channels/voice.md`).

## Scope

`augur-voice` runtime side: synthesize the recommendation summary via the
configured `TtsProvider`, stream bytes over RPC (chunked under the 8MiB
frame cap); desktop side: audio playback (rodio or cpal-output), voice
toggle, volume/mute, auto-speak-on-publish option; invalidation stops
playback of stale advice.

## Non-goals

STT (next issue); streaming synthesis (providers are request/response —
recorded limitation).

## Acceptance criteria

- A published recommendation is audible end-to-end with each of two
  providers (one hosted, Piper local).
- Stale-advice event stops playback mid-utterance.
- TTS-start latency measured against budget.

## Dependencies

#coaching-turn, #augur-rpc-extensions, #desktop-shell-mvp.

## Test plan

Provider mocks for pipeline tests; live-marked audio smoke; latency ledger
rows.

## Documentation impact

voice.md TTS stage implemented.

## Security, privacy, and policy considerations

No audio retained; provider transmission disclosed in voice settings.
