# MVP Definition (Milestone 1: Battlegrounds Advice MVP)

One narrow, complete path — deliberately not continuous monitoring:

1. Desktop app launches; onboarding completes (permissions, provider,
   strategy-pack readiness, detection test).
2. User selects or auto-detects the Hearthstone window.
3. User triggers an observation manually (button or hotkey).
4. The frame reaches a vision-capable model correctly (as an `[IMAGE:]`
   marker on the coaching turn).
5. The active seasonal strategy pack is searched (metadata + lexical,
   capped).
6. The coach returns schema-valid advice with strategy references.
7. The GUI displays the recommendation: summary, prioritized actions,
   confidence, evidence split, citations.
8. Advice remains bound to its `observation_id`; a newer observation marks it
   stale.
9. Recorded fixtures and baseline evaluation tests exist and pass in CI.
10. No gameplay automation exists anywhere.

## Observation scope (shop phase first)

| Field | MVP class |
|---|---|
| Current phase | Required |
| Hero | Required |
| Health / armor | Required |
| Tavern tier | Required |
| Gold | Required |
| Shop offerings | Required |
| Board minions | Required |
| Hand contents | Useful, optional (partial occlusion tolerated) |
| Frozen state | Useful, optional |
| Remaining turn time | Useful, optional (timer OCR) |
| Season spells/mechanics | Useful, optional |
| Lobby tribes | Required (banner visible in shop) |
| Opponent preview | Deferred (only when visible) |
| Known triples/pairs | Deferred (derived state) |
| Recent recommendation history | Required (local, not vision) |
| Opponent boards / next-fight inference | Unreliable from screen-only capture — stated as uncertainty |
| Precise combat odds | Not possessed — never claimed |

Feasibility is investigated, not assumed: the extraction evaluation harness
(fixtures → per-field accuracy) is an MVP exit criterion, and each field's
class can be demoted by evidence. Log-file augmentation (`Power.log`) is
gated behind the policy review (docs/policy/game-policy-review.md).

## Advice scope

Shop-phase buy/sell, freeze/no-freeze, leveling, economy/curve, board-space,
basic positioning when the board is visible, composition direction,
transition warnings, and explicit uncertainty when recognition is incomplete.
Deferred: autonomous action (never), hidden-information inference, claims
requiring data Augur does not possess.

## Performance and cost budgets (targets to measure against, not claims)

| Stage | Budget (p50 / p95) |
|---|---|
| Capture + normalize | 50ms / 150ms |
| Change detection (M2) | 10ms / 30ms |
| State extraction + recommendation (single combined model call, MVP) | 4s / 8s |
| Strategy retrieval (local) | 30ms / 100ms |
| Validation + publish | 20ms / 50ms |
| **End-to-end (manual trigger → advice visible)** | **5s / 9s** |
| TTS start after recommendation (M3) | 700ms / 1.5s |
| PTT transcription (M3) | 1s / 2.5s |

Within a ~60–90s Battlegrounds shop turn, 5s advice is actionable. Tracked
per turn in the local metrics ledger: end-to-end latency, frames captured vs
discarded (M2), vision requests, tokens in/out, estimated cost, retrieval
count, stale recommendations discarded, extraction confidence, model
failures, user feedback. Cost levers: adaptive capture, state diffs, image
downscale/compression, response caching on unchanged scope, bounded
retrieval.
