## Problem

Advice must die when its observation is superseded — the core honesty
mechanic (product principle 2) — and every surface (GUI, overlay, TTS
later) must react.

## Context and repository evidence

state-and-recommendations.md invalidation rules (`invalidated_by` ×
`change_summary` intersection, phase change, turn end, expiry);
`advice_invalidated` on `augur/event`.

## Scope

Invalidation engine in `augur-recommendation`: subscribe to new envelopes,
kill standing recommendations per rules, emit events; in-flight discard
(recommendation arriving for a superseded observation never publishes);
GUI/overlay stale rendering.

## Non-goals

Re-generation policy tuning (cooldown logic lives in cadence).

## Acceptance criteria

- Fixture sequences: shop roll invalidates buy advice; phase flip
  invalidates shop advice; unrelated change does not; in-flight discard
  proven with a delayed-trace test.
- UI shows struck/stale within one event cycle.

## Dependencies

#match-session-state, #capture-cadence-change-detection, #coaching-turn.

## Test plan

Envelope-sequence fixtures per rule; race test for in-flight discard.

## Documentation impact

Invalidation section implemented.

## Security, privacy, and policy considerations

Prevents stale-advice harm; no new surface.
