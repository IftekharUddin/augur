## Problem

The multi-game claim is unproven until a second adapter exists. A minimal
fixture-driven adapter (turn-based or synthetic) must pass the pipeline with
zero platform edits.

## Context and repository evidence

game-adapter.md stability criterion; new-game-process.md; decision 0002
consequences ("validates that the registry and dependency-direction rules
actually hold"); ReplayCaptureProvider (from #macos-capture-provider).

## Scope

Pick a deliberately simple target (a solitaire-like or a synthetic
"test-game" with generated frames); scaffold from the template; implement
detection (fixture), parsing (deterministic from generated frames),
strategy scope, a five-document strategy pack, coaching through the real
pipeline via replay capture + trace replay; document every friction point
as issues.

## Non-goals

Shipping the second game to users (it may remain a test asset,
`status: experimental`, hidden by default).

## Acceptance criteria

- End-to-end fixture run: generated frame → envelope → retrieval →
  recommendation → validation, green in CI.
- Zero platform-crate diffs in the PR (architecture test + review
  assertion).
- Friction report filed as issues.

## Dependencies

#new-game-template, #adapter-api-freeze.

## Test plan

The CI run is the proof; kept as a permanent regression suite.

## Documentation impact

game-adapter.md "validated by" pointer; roadmap M4 exit.

## Security, privacy, and policy considerations

Synthetic game needs no policy review (recorded as such in its game.yaml).
