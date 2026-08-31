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

Augur's gate is `.github/workflows/ci.yml`, on GitHub-hosted runners only.
Branch protection on `master` requires exactly one check,
**`Augur Required Gate`**: an aggregate job that fails if any member job
failed or was cancelled. Internal jobs can be added or split without touching
branch-protection settings; this is upstream's aggregate pattern, kept
deliberately.

Member jobs today:

| Job | What it gates |
|---|---|
| Format | `cargo fmt --all -- --check` |
| Docs Links | Relative links in Augur-owned Markdown resolve (`scripts/ci/augur_docs_links_gate.py`, with its own contract tests) |
| Architecture Boundaries | The RPC-only desktop rule. Runs upstream's `zerocode_no_zeroclaw_dep_gate.sh` today; gains the Augur invariants (no game identifiers in platform crates, no `zeroclaw-*`/`augur-runtime` deps in `apps/augur-desktop`, no input-synthesis symbols) with the crate skeleton |
| Lint | `cargo clippy --locked --workspace --exclude zeroclaw-desktop --all-targets -- -D warnings` |
| Test | `cargo test --locked --workspace --exclude zeroclaw-desktop --no-fail-fast` |

Two scope choices are deliberate and should be revisited, not inherited by
accident:

- The compile jobs run the **default feature surface**, not upstream's
  `ci-all`. The full feature matrix costs far more wall-clock on hosted
  runners than a coaching fork needs per pull request.
- `zeroclaw-desktop` is excluded exactly as upstream excludes it: it pulls
  GTK/glib system libraries the gate does not install.

Strategy-pack validation joins the gate when the validation CLI exists
(Milestone 1); the extraction and recommendation evaluation harnesses join as
they land. Heavy platform matrices run scheduled, mirroring upstream's
`platform-tests.yml` split, once capture providers exist.

Every inherited upstream workflow is parked under
`.github/workflows-upstream/` with the reason it was parked and what would
bring it back; `.github/workflows/pr-title.yml` (Conventional Commits with a
required scope) was kept running unchanged. Note that `dev/ci.sh`, referenced
from the inherited `CONTRIBUTING.md`, still reproduces *upstream's* Docker CI,
not this gate; the closest local equivalent to the Augur gate is the five
commands in the table above.
