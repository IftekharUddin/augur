## Problem

Extraction quality claims need measurement: fixtures in, per-field
accuracy/confidence-calibration out — otherwise field classifications and
model/prompt changes are vibes.

## Context and repository evidence

testing-and-evaluation.md; `crates/zeroclaw-eval` (deterministic replay,
`Grader` seam, JSON reports, CI exit codes) as the base;
games/hearthstone-battlegrounds/fixtures/ scaffold.

## Scope

Harness comparing extracted envelopes against labeled ground truth per
fixture: per-field accuracy, false-presence rate (hallucinated fields),
confidence calibration summary; JSON + table reports; CI job (advisory
first, gating once baselined); labeling format for fixtures.

## Non-goals

Recommendation quality (separate harness); live-game capture.

## Acceptance criteria

- Run over ≥20 labeled fixtures produces the report; a seeded wrong label
  fails as designed.
- Wired into CI as advisory with baseline recorded.

## Dependencies

#bg-state-schema-extraction (formats), #common-envelope-schemas.

## Test plan

Self-test with synthetic perfect/degraded extractions.

## Documentation impact

testing-and-evaluation.md harness section.

## Security, privacy, and policy considerations

Fixture screenshots privacy-reviewed and legally shareable (own gameplay,
no third-party names beyond what review clears).
