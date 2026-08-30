## Problem

No Augur crates exist. The dependency-direction rules (platform never imports
games; desktop never links runtime) are only prose until crates and
architecture tests enforce them.

## Context and repository evidence

- Layout: docs/architecture/overview.md; crates `augur-core`,
  `augur-game-api`, `augur-capture`, `augur-observation`, `augur-strategy`,
  `augur-recommendation`, `augur-runtime`, `augur-voice`, `augur-policy`;
  app `apps/augur-desktop`; per-game `games/<id>/adapter`.
- Upstream architecture-test pattern: `tests/test_architecture.rs`,
  `tests/architecture/no_duplicate_state.rs` (source-grep invariants).
- Upstream workspace conventions: root `Cargo.toml` members list,
  `license.workspace = true`, edition 2024, `publish = false` during
  transition.

## Scope

Empty-but-compiling crates with doc comments and the trait/type stubs from
the architecture docs; workspace membership; three architecture tests:
(a) platform crates contain no concrete game id, (b) `apps/augur-desktop`
has no `zeroclaw-*`/`augur-runtime` deps (copied gate script), (c) no
input-synthesis symbols in Augur crates.

## Non-goals

Any behavior. Trait signatures may still change (game-adapter API freezes in
M4).

## Proposed approach

One PR: crates + members + tests + gate script under `scripts/ci/`.

## Acceptance criteria

- `cargo check --workspace` green with new members.
- All three architecture tests exist, run in CI, and fail when violated
  (each proven by a deliberate red run in the PR description).

## Dependencies

#ratify-decision-0001 (desktop gate encodes it), #ci-adaptation (tests must
run somewhere).

## Test plan

Red/green demonstration per invariant; `cargo test -p` on new crates.

## Documentation impact

overview.md crate table becomes "exists"; CONTRIBUTING pointer.

## Security, privacy, and policy considerations

The no-input-synthesis test is the "coach, not bot" enforcement seam.
