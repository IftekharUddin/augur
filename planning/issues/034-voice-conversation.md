## Problem

Spoken questions must get spoken answers grounded in the current match and
strategy corpus, with interruption, cancellation, and mute semantics that
respect the active turn — without spawning contradictory recommendations.

## Context and repository evidence

voice.md turn semantics (steering message on the active session; references
the standing recommendation; barge-in = local TTS stop + `session/cancel`);
upstream cancel path (`rpc/turn.rs` `TurnOutcome::Cancelled`,
`session/cancel`); prompts/README.md `voice-conversation.md` contract.

## Scope

Conversation turn type in `augur-runtime` (transcript in → grounded answer
out, TTS'd); `voice-conversation.md` prompt (reference observation,
explain rationale, cite strategy, state uncertainty, no second contradictory
recommendation unless state changed); interruption/cancel/mute state
machine across desktop and runtime; transcript display with retention
opt-in enforcement; voice diagnostics redaction (no raw audio/transcripts
without separate consent).

## Non-goals

Hands-free/wake word; multi-question memory beyond the match session.

## Acceptance criteria

- Trace-replay fixtures: "why level?" answer cites the standing
  recommendation's strategy refs; state-changed variant may produce new
  advice; contradiction case fails red.
- Barge-in during playback: audio stops <100ms locally, turn cancels.
- Mute suppresses TTS but not visual advice.

## Dependencies

#ptt-stt, #tts-recommendations, #advice-invalidation.

## Test plan

Trace fixtures per semantics rule; interruption timing test; retention
enforcement tests.

## Documentation impact

voice.md conversation + privacy stages implemented.

## Security, privacy, and policy considerations

Retention defaults hold under conversation (audio dropped, transcripts
opt-in); diagnostics redaction verified.
