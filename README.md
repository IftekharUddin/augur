<div align="center">

# Augur

**Real-time AI game coaching. A coach, never a bot.**

</div>

Augur watches your game (with your permission), consults community-maintained
season-specific strategy, and tells you the highest-value next action — in a
desktop app, an optional overlay, and eventually a spoken conversation. First
supported game: **Hearthstone Battlegrounds**.

Augur never clicks, drags, or presses keys; never takes game actions; never
reads or modifies game memory; never injects code, manipulates traffic, or
circumvents anti-cheat; and never presents uncertain observations as facts.

> **Status: planning.** This repository was just forked from
> [ZeroClaw](https://github.com/zeroclaw-labs/zeroclaw) (full history
> preserved) and carries the founding architecture, governance, and roadmap
> plan. No coaching product exists yet. Start at
> [`docs/roadmap.md`](docs/roadmap.md) and the
> [architecture overview](docs/architecture/overview.md).

## How it will work

1. Install Augur and complete a short first-run setup (screen-recording
   permission, model provider).
2. Augur detects your running game; you confirm the window.
3. Press **Start Coaching**. Augur observes the game window — read-only.
4. Advice appears with confidence, expiry conditions, and citations into the
   [strategy corpus](docs/architecture/strategy-packs.md).
5. Later: ask questions by voice, get spoken answers grounded in the current
   match ([voice roadmap](docs/architecture/voice.md)).

## Built on ZeroClaw

Augur uses the [ZeroClaw](https://github.com/zeroclaw-labs/zeroclaw) agent
runtime (agent loop, model providers, vision pipeline, local RPC, config and
secrets, logging) as a bundled sidecar daemon —
[decision 0001](docs/decisions/0001-runtime-integration.md). The
[reuse audit](docs/architecture/zeroclaw-reuse-audit.md) maps exactly what is
reused, extended, wrapped, or excluded, and the
[upstream-sync policy](docs/architecture/upstream-sync.md) keeps the fork
current — security fixes first. The original ZeroClaw README is preserved at
[`docs/ZEROCLAW-README.md`](docs/ZEROCLAW-README.md).

## Contributing

Two paths, deliberately unequal in friction:

- **Strategy knowledge (no Rust required)** — write or review seasonal
  strategy documents: [strategy review workflow](docs/governance/strategy-review.md).
- **Platform and adapter code** — see [CONTRIBUTING.md](CONTRIBUTING.md),
  the [game adapter API](docs/architecture/game-adapter.md), and the
  [new-game process](docs/governance/new-game-process.md).

Planning artifacts (labels, milestones, the full issue graph) are
reproducible from [`planning/github-plan.yaml`](planning/github-plan.yaml).

## License

Dual-licensed: [MIT](LICENSE-MIT) OR [Apache 2.0](LICENSE-APACHE), unchanged
from upstream ZeroClaw. See [NOTICE](NOTICE) for attribution. "ZeroClaw" is
upstream's mark; Augur is an independent downstream project and is not
endorsed by ZeroClaw Labs.
