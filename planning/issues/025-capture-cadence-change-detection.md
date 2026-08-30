## Problem

Live coaching must not send frames at a fixed expensive rate; cadence must
adapt to change (explicit product requirement).

## Context and repository evidence

capture-and-observation.md cadence section (cheap hash loop at a few Hz;
perceptual hash over downscaled luma; extraction only on material change or
adapter phase heuristics; cooldowns; captured-vs-discarded metrics);
`SCStream`/frame-pool notes in the platform section.

## Scope

Continuous capture mode in `augur-capture` (SCStream on macOS, frame pool on
Windows); change detector (frame SHA-256 for identity, perceptual diff with
per-region weighting from the adapter's capture profile); trigger policy
(material-change threshold, min/max intervals, cooldown after each model
call); `augur/capture/start|stop` wiring; discarded-frame accounting.

## Non-goals

Model-call batching strategies beyond cooldowns; combat-phase cadence tuning
(follow-up once fixtures exist).

## Acceptance criteria

- Recorded gameplay fixture (frame sequence) yields: expected trigger points,
  ≥X% frames discarded pre-model (measured, baseline recorded), no trigger
  during static shop idle.
- CPU overhead of the hash loop measured and within budget
  (mvp.md table).

## Dependencies

#macos-capture-provider, #windows-capture-provider, #augur-rpc-extensions.

## Test plan

Frame-sequence fixtures with labeled change points.

## Documentation impact

Cadence section gains measured numbers.

## Security, privacy, and policy considerations

Idle = no frames leave the machine; stop is immediate.
