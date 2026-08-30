## Problem

Privacy defaults are documented but not implemented: retention controls,
active-capture indicator, immediate stop, and the opt-in incorrect-advice
feedback bundle bound to its observation/model-call/strategy context.

## Context and repository evidence

security-and-privacy.md privacy defaults; user-experience.md privacy &
retention controls + report-incorrect-advice; upstream redaction machinery
(`crates/zeroclaw-log` policies, leak detector) to reuse.

## Scope

Retention controls UI + enforcement (session state clear, match history
cap, transcript opt-in placeholder for M3); always-visible capture
indicator + one-click stop (kills stream, flushes buffers); feedback
bundle: recommendation + envelope (minus frame) + retrieval trace + optional
user note, previewable before save/share, frames included only with explicit
per-bundle consent.

## Non-goals

Any upload endpoint (bundles are files the user shares manually);
analytics.

## Acceptance criteria

- Stop provably halts frame flow (test hooks); indicator state matches
  stream state in fixtures.
- Bundle preview shows exact contents; leak-scan pass; frame inclusion off
  by default.
- Retention actions verifiably delete (store inspected in tests).

## Dependencies

#match-session-state, #metrics-degraded-states, #overlay (indicator
placement).

## Test plan

Store-level deletion tests; bundle content assertions; indicator state
machine tests.

## Documentation impact

security-and-privacy.md defaults become "implemented, verified by" rows.

## Security, privacy, and policy considerations

This issue IS the privacy implementation; review by security maintainer
required (two-review rule).
