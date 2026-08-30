# Schemas (drafted in Milestone 0/1 issues)

- `game-state.schema.json` — the `state` payload of the observation envelope.
- `recommendation.schema.json` — game-scoped constraints over the common
  recommendation contract (action verbs: buy/sell/roll/freeze/level/position/play).
- `strategy-frontmatter.schema.json` — front matter contract (see
  docs/architecture/strategy-packs.md).
- `strategy-pack.schema.json` — season `manifest.yaml` contract.

Draft JSON Schemas land with the schema issues (M0 #common-envelope, M1
#bg-state-schema); committing hand-waved schemas now would only create churn.
Envelope-level fields live in `crates/augur-core`, not here.
