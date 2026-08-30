## Problem

The GameAdapter API has been evolving freely through M1–M3; multi-game
support needs it stable or explicitly versioned.

## Context and repository evidence

game-adapter.md stability section (frozen only after second-game proof);
`GameManifest.adapter_version`; upstream WIT versioning discipline
(`wit/VERSIONING.md`) as the cautionary model for what versioning costs.

## Scope

API review pass over `augur-game-api` (naming, error types, envelope
boundaries) informed by the real Battlegrounds adapter; document
compatibility policy (what a minor vs breaking change is, deprecation
window); freeze marker + changelog discipline; architecture test that
platform crates still contain no game ids (ratchet check).

## Non-goals

Runtime-loaded plugins (rejected in decision 0002; revisit criteria there).

## Acceptance criteria

- Versioned contract documented; Battlegrounds adapter compiles against it
  unmodified or with a recorded migration.
- Compatibility policy in game-adapter.md.

## Dependencies

M1 adapter experience; #second-game-proof co-developed (the proof exercises
the freeze).

## Test plan

Second-game proof is the test; plus API-doc examples compile
(doc-tests).

## Documentation impact

game-adapter.md marked stable/versioned.

## Security, privacy, and policy considerations

None new.
