# Schemas

## Game-owned (drafted in Milestone 1 issues)

- `game-state.schema.json` - the `state` payload of the observation envelope.
- `recommendation.schema.json` - game-scoped constraints over the common
  recommendation contract (action verbs: buy/sell/roll/freeze/level/position/play).
- `strategy-frontmatter.schema.json` - front matter contract (see
  docs/architecture/strategy-packs.md).
- `strategy-pack.schema.json` - season `manifest.yaml` contract.

These land with the Battlegrounds state-schema work. Committing hand-waved
schemas now would only create churn.

## Envelope-level (already generated)

Envelope-level fields live in `crates/augur-core`, not here. Their schemas are
**generated from the Rust types**, which are canonical, and committed at:

- [`crates/augur-core/schemas/observation-envelope.schema.json`](../../../crates/augur-core/schemas/observation-envelope.schema.json)
- [`crates/augur-core/schemas/recommendation.schema.json`](../../../crates/augur-core/schemas/recommendation.schema.json)

Regenerate with:

```bash
cargo run -p augur-core --bin augur-schema-export -- crates/augur-core/schemas
```

Do not hand-edit them. `crates/augur-core/tests/schema_drift.rs` regenerates
and compares on every test run, so an edit will simply fail; the direction of
truth is Rust type to schema, never the reverse.

The `state` object in the envelope schema is deliberately unconstrained: it is
this game's payload, validated against `game-state.schema.json` above, and
`augur-core` must not know what a Battlegrounds board looks like.
