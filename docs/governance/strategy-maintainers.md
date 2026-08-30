# Strategy Maintainership

Strategy maintainers own the truthfulness and freshness of a game's strategy
corpus without needing any Rust. Scopes are declared in
`games/<game-id>/maintainers.yaml`:

```yaml
strategy_maintainers:
  - github: example-strategist
    scopes:
      - season:<season-id>        # everything in that season
      - tribe:murloc              # a category slice across the season
```

## Duties

- Review strategy PRs in scope (accuracy, sourcing, front-matter quality,
  no injection patterns).
- Promote documents `draft → reviewed → stable`; demote to `deprecated` when
  the meta or patch invalidates them.
- Run the season lifecycle: scaffold new seasons, copy forward still-valid
  docs as drafts, keep `manifest.yaml` patch ranges honest.
- Arbitrate strategy disputes (below).

## Nomination

Anyone with accepted strategy contributions can be nominated (or
self-nominate) via a `type:governance` issue; a game maintainer approves and
lands the `maintainers.yaml` change. Removal mirrors the game-maintainer
inactivity process.

## Dispute resolution

Strategic disagreements are issues labeled `type:strategy`, argued with
evidence (replays, statistics, sourced reasoning). The scope's strategy
maintainer decides; the losing position is preserved in `known_exceptions` or
`sources` when it has merit. Escalation: game maintainer, then core. The bar
for `stable` is "a strong player following this will not be misled", not
personal preference.
