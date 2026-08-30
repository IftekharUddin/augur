## Problem

Observations must persist across a match (turn history, recent
recommendations, composition trajectory) — explicitly not via open-ended
agent memory.

## Context and repository evidence

state-and-recommendations.md match-session section (SQLite beside the
upstream sessions store, `<workspace>/sessions/`, separate namespace);
product rule against substituting `zeroclaw-memory`.

## Scope

`augur-observation` ObservationStore: begin/append/latest/end;
MatchIdentity derivation (game + detected match start heuristics); recent
recommendation history feed into coaching context; retention policy (local,
capped, user-clearable per privacy controls); daemon-restart survival.

## Non-goals

Cross-match analytics; cloud sync (never, by default posture).

## Acceptance criteria

- Envelope sequences round-trip; restart mid-match resumes the session;
  `latest()` behavior under out-of-order appends defined and tested.
- Coaching context includes recent-recommendation summary (visible in trace
  fixtures).

## Dependencies

#common-envelope-schemas.

## Test plan

Component tests with envelope fixtures; restart simulation.

## Documentation impact

state-and-recommendations.md implemented markers.

## Security, privacy, and policy considerations

State is text (no frames); covered by retention controls
(#privacy-retention-feedback).
