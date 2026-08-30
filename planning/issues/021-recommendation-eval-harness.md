## Problem

Recommendation quality needs property-based evaluation and an
expert-reviewed golden set before the MVP can claim anything.

## Context and repository evidence

testing-and-evaluation.md evaluation-record and graded-properties lists;
`crates/zeroclaw-eval` `Grader` trait ("side-effect/budget/LLM-judge graders
in later phases" is the documented seam), `LlmTrace` replay, JSON reports.

## Scope

Augur graders over the evaluation record (valid structure, observation
binding, citations ⊆ retrieved, no impossible action vs observed state, no
stale advice, uncertainty when confidence low, policy compliance); golden
set (~25–50 scenarios across shop stages) with human ratings recorded in
fixtures; CI job advisory→gating.

## Non-goals

Live model benchmarking; extraction accuracy (#extraction-eval-harness).

## Acceptance criteria

- Harness runs the golden set in CI from traces (no provider needed);
  seeded violations per property fail.
- Baseline report committed; MVP exit references it.

## Dependencies

#coaching-turn, #extraction-eval-harness (record formats align).

## Test plan

Self-test via seeded-violation traces.

## Documentation impact

testing-and-evaluation.md golden-set section becomes concrete.

## Security, privacy, and policy considerations

Golden fixtures privacy-reviewed; "policy compliance" property enforces
no-automation phrasing in advice.
