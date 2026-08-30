# Game Adapter API

The seam that keeps game-specific code out of platform crates. Loading model
and rationale: [decision 0002](../decisions/0002-game-adapter-loading.md).

## Responsibilities

A game adapter owns, for exactly one game:

1. **Detection** — is the game running; which window is it (executable names,
   window-title patterns from `game.yaml`).
2. **Capture profile** — how to capture it (frame cadence bounds, regions of
   interest, resolution normalization hints).
3. **Observation parsing** — turn a captured frame (plus prior state) into a
   `GameStateEnvelope` with per-field confidence; missing fields stay missing.
4. **Strategy scoping** — map current state to a `StrategyScope` (season,
   phase, entities) for retrieval.
5. **Coaching context** — assemble the bounded, delimited context for the
   coaching turn.
6. **Recommendation validation** — game-specific sanity checks (no impossible
   actions, e.g. buying with insufficient gold *when gold was observed*).

Trait sketch (names to be finalized against implementation; conceptually):

```rust
pub trait GameAdapter: Send + Sync {
    fn manifest(&self) -> &GameManifest;
    fn detect(&self, ctx: &DetectionContext) -> DetectionResult;
    fn capture_profile(&self, window: &GameWindow) -> CaptureProfile;
    fn parse_observation(&self, frame: &CapturedFrame, previous: Option<&GameStateEnvelope>)
        -> Result<GameStateEnvelope, ObservationError>;
    fn strategy_scope(&self, state: &GameStateEnvelope) -> StrategyScope;
    fn coaching_context(&self, state: &GameStateEnvelope, strategies: &[StrategyDocument])
        -> CoachingContext;
    fn validate_recommendation(&self, state: &GameStateEnvelope, rec: &Recommendation)
        -> ValidationReport;
}
```

`parse_observation` is expected to delegate vision-heavy extraction to the
runtime (the adapter defines the extraction prompt and the state schema; the
model call happens inside the coaching pipeline, not inside the adapter).
Deterministic extraction (OCR of stable regions) may live in the adapter.

## Registry

Compile-time: each game crate exports a constructor; `augur-runtime` holds the
single registration list. Adding a game touches (a) the new
`games/<id>/adapter` crate, (b) one registry entry, (c) workspace members.
An architecture test fails if platform crates (`augur-core`, `augur-capture`,
`augur-observation`, `augur-strategy`, `augur-recommendation`, `augur-voice`,
`augur-policy`, `augur-game-api`) mention a concrete game id.

## Game manifest (`games/<id>/game.yaml`)

Stable game id, display name, adapter version, supported OSes, detection
rules (executables, window-title patterns), capture profile defaults,
strategy-pack location, schema paths, required permissions, maintainers
pointer, support status, policy review status, minimum Augur version,
optional voice vocabulary, fixture/diagnostics paths. Machine-validated
(JSON Schema) by the strategy/manifest validation CLI.

## Support statuses

`experimental` → `community` → `maintained`; terminal states `deprecated` and
`disabled-policy-review`. Status is data in `game.yaml`, surfaced in the UI
and enforced in CI (a `disabled-policy-review` game cannot ship enabled).

## Stability

The API is unstable until Milestone 4's second-game proof (a minimal
turn-based or synthetic adapter driven entirely by fixtures) compiles against
it without platform changes. After that, changes follow a decision record and
a versioned `GameManifest.adapter_version`.
