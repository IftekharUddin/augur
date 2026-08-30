## Problem

"Real-time" and "affordable" are claims that need a ledger; degraded modes
need to be real UI states, not error toasts.

## Context and repository evidence

mvp.md budgets + tracking list; user-experience.md degraded-state
inventory (14 states); upstream metrics precedent: Prometheus `/metrics`
exists on the gateway but the desktop path needs a local ledger; upstream
logging via `record!` (`crates/zeroclaw-log`).

## Scope

Local metrics ledger (per-turn: end-to-end latency, stage timings, frames
captured/discarded, vision requests, tokens, est. cost, retrieval count,
stale discards, extraction confidence, failures, feedback links) persisted
under the workspace; surfaced in a diagnostics pane + export; every
degraded state in the UX doc reachable, distinct, and carrying a next
action; state-injection dev tool for walkthroughs.

## Non-goals

Remote telemetry (never by default); dashboards beyond the diagnostics
pane.

## Acceptance criteria

- Budget table rendered from real measured data after a fixture session.
- Scripted walkthrough renders all 14 degraded states.
- Export produces a shareable, secrets-free bundle (leak-scan pass).

## Dependencies

#coaching-turn, #capture-cadence-change-detection, #desktop-shell-mvp.

## Test plan

Ledger unit tests; walkthrough script in CI (headless render or state
snapshot assertions).

## Documentation impact

mvp.md budgets get a "measured" column.

## Security, privacy, and policy considerations

Ledger is local; export leak-scanned via upstream detector patterns.
