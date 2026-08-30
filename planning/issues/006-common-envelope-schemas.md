## Problem

The observation envelope and recommendation contract exist only as prose
examples; nothing validates them.

## Context and repository evidence

docs/architecture/state-and-recommendations.md (field tables, invalidation
semantics, evidence four-way split);
games/hearthstone-battlegrounds/schemas/README.md placeholders.

## Scope

JSON Schemas + Rust types (serde) in `augur-core` for the envelope and
recommendation contract; schema_version discipline; validation helpers;
the game-state payload stays game-owned (M1 issue).

## Non-goals

Battlegrounds `state` schema; extraction.

## Proposed approach

Rust types are canonical; JSON Schema generated (schemars, mirroring
upstream's `schema-export` feature pattern in `zeroclaw-config`).

## Acceptance criteria

- Round-trip tests (serialize → validate → deserialize) for both contracts.
- Unknown-field rejection proven; absent-vs-null semantics tested.
- Generated schemas committed under `games/hearthstone-battlegrounds/schemas/`
  for the envelope-level parts referenced there.

## Dependencies

#augur-crate-skeleton.

## Test plan

Unit round-trips + fixture instances (one valid, several invalid per rule).

## Documentation impact

state-and-recommendations.md marked implemented; schemas README updated.

## Security, privacy, and policy considerations

`privacy` classification field semantics documented; frame_hash is a hash,
never pixels.
