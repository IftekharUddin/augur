# Roadmap

Five milestones; exit criteria are the milestone descriptions on GitHub (and
mirrored in `planning/github-plan.yaml`). Summary:

| # | Milestone | Theme | Headline exit criteria |
|---|---|---|---|
| 0 | Augur Foundation | Fork, audit, decisions, schemas, CI | Reuse audit committed; decisions 0001–0003 ratified; envelopes drafted; upstream-sync policy live; CI adapted; issue graph created |
| 1 | Battlegrounds Advice MVP | One manual observation → cited advice | The 10-point MVP path (docs/product/mvp.md) demonstrably works; fixtures + baseline evals in CI; zero automation |
| 2 | Live Battlegrounds Coaching | Continuous, honest, measured | Auto-capture + change detection; session state; stale-advice invalidation; capture-excluded overlay; latency/cost measured; privacy documented |
| 3 | Voice Conversation | Spoken advice + PTT Q&A | TTS of recommendations; PTT STT; grounded answers; interruption/mute; replaceable providers; no retention by default |
| 4 | Multi-Game & Community Platform | Second game proves the seams | Versioned GameAdapter; new-game template; per-game CODEOWNERS; second-game/synthetic adapter passes fixtures with no platform edits |

## Critical path

```text
reuse audit → decision 0001 (runtime) → augur/* RPC + desktop shell
  → screenshot-to-vision handoff → manual observation (M1 spine)
  → state schema → strategy retrieval → structured advice → GUI render
  → MVP evaluation gate
  → auto capture → change detection → session state → invalidation → overlay (M2)
  → TTS → PTT STT → conversational voice (M3)
  → adapter API freeze → second-game proof (M4)
```

Parallel workstreams once M0 lands: capture providers (macOS/Windows) ‖
strategy-pack tooling + corpus ‖ desktop UI ‖ evaluation harness. Blocking
decisions are all in M0.
