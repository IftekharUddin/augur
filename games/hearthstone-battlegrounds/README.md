# Hearthstone Battlegrounds — Augur game module

The first Augur-supported game. This directory owns everything
Battlegrounds-specific: the (future) adapter crate, machine-validated
schemas, coaching prompts, the seasonal strategy corpus, fixtures, and
game-scoped tests. Platform crates must never import anything from here.

- Manifest: [`game.yaml`](game.yaml) · Ownership: [`maintainers.yaml`](maintainers.yaml)
- Architecture: [`docs/architecture/game-adapter.md`](../../docs/architecture/game-adapter.md)
- Strategy system: [`docs/architecture/strategy-packs.md`](../../docs/architecture/strategy-packs.md)
- Contributing strategy (no Rust needed): [`docs/governance/strategy-review.md`](../../docs/governance/strategy-review.md)
- Policy gate: [`docs/policy/game-policy-review.md`](../../docs/policy/game-policy-review.md) — status `pending`; the game cannot ship until recorded.

Status: **planning scaffold.** `adapter/` does not exist yet (Milestone 1);
the season pack under `strategies/season-2026-08/` is a placeholder skeleton
whose documents are all `status: draft` and whose season id will be renamed
to the live patch identifier by the first strategy maintainer to populate it.
