# Testing and Evaluation

No test requires a live game. Everything runs from recorded,
privacy-reviewed, legally shareable fixtures.

## Test taxonomy (inherited from upstream)

Upstream's five levels (`docs/book/src/contributing/testing.md`) plus
architecture-invariant tests (`tests/test_architecture.rs` greps the source
tree). Augur adds invariants:

- Platform crates contain no concrete game identifiers.
- `apps/augur-desktop` links no `zeroclaw-*`/`augur-runtime` crates
  (dependency gate script, copied from
  `scripts/ci/zerocode_no_zeroclaw_dep_gate.sh`).
- No input-synthesis symbols anywhere in Augur crates.
- Strategy packs contain only allowed file types.

## Fixture-based tests

`games/<id>/fixtures/`: screenshots (synthetic or privacy-reviewed),
observation envelopes, recommendation records. Cover: window detection
(mocked enumerations), capture normalization (DPI matrix), state extraction
(frame → envelope), change detection (frame pairs), retrieval (scope →
expected ids), recommendation generation (trace replay), invalidation
(envelope sequences), overlay rendering states, voice state machine, error
paths. A `ReplayCaptureProvider` feeds recorded frames through the real
pipeline.

## Strategy tests

Front-matter schema validation, season-manifest validation, broken-reference
and duplicate-id detection, patch-compatibility, active-pack uniqueness,
retrieval relevance (query → must-include/must-exclude ids),
deprecated-exclusion, citation validity, and prompt-injection regression
fixtures. All run by the strategy validation CLI locally and in CI.

## Recommendation evaluation

Built on `crates/zeroclaw-eval` (deterministic replay of the real agent loop
from `LlmTrace` fixtures; `Grader` trait is the documented extension seam).
Augur adds property graders over an evaluation record:

```
{ observation, pack_version, retrieved_docs, provider, model,
  recommendation, expected_properties, human_rating,
  latency_ms, tokens_in, tokens_out, est_cost, failure_category }
```

Graded properties (not exact text match): valid structured output; correct
observation binding; citations ⊆ retrieved; no impossible action against the
observed state; no stale advice; uncertainty stated when confidence low;
top action relevant (human-rated); policy compliance (no gameplay
automation).

A small expert-reviewed **golden set** (~25–50 scenarios across phases and
game stages) gates the first release; human ratings are recorded in the
fixture so regressions are diffable.

## Cross-platform tests

macOS permission flows (mocked TCC states), Windows capture, DPI matrix,
multi-monitor enumeration, overlay exclusion (capture-with-overlay
assertion), packaging smoke (upstream's release smoke-test pattern: boot the
bundled kernel, poll an endpoint), daemon start/stop, missing credentials,
network loss, provider timeout, non-vision model routing (fails closed with
the right UI state).

## CI

Upstream workflows are disabled at fork time (they target upstream's
self-hosted runners and required checks). Augur CI (Milestone 0 issue)
defines its own required gate on hosted runners: fmt, clippy, arch tests,
unit/component, strategy validation, docs links. Heavy platform matrices run
scheduled, mirroring upstream's `platform-tests.yml` split.
