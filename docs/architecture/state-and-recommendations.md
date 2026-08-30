# State and Recommendation Contracts

Common envelopes are small and game-agnostic; payloads are game-owned.
Schemas live at `games/<id>/schemas/` (game payloads) and
`crates/augur-core` (envelopes), validated in CI.

## Observation envelope

```json
{
  "schema_version": 1,
  "game_id": "hearthstone-battlegrounds",
  "game_version": "<season-or-patch-identifier>",
  "adapter_version": "0.1.0",
  "strategy_pack_version": "<pack-version>",
  "session_id": "uuid",
  "observation_id": "uuid",
  "captured_at": "RFC3339",
  "phase": "shop",
  "source": { "kind": "screen_capture", "window_id": "opaque", "frame_hash": "sha256" },
  "confidence": 0.93,
  "change_summary": ["shop_contents", "gold"],
  "privacy": "local-frame-transient",
  "evidence": [{ "field": "gold", "region": [x,y,w,h], "confidence": 0.99 }],
  "state": { }
}
```

`state` follows the game adapter's schema
(`games/hearthstone-battlegrounds/schemas/game-state.schema.json`). Envelope
rules: unknown fields rejected; `confidence` is overall extraction confidence;
per-field confidence lives in `evidence`; absent data is absent, not null-y
guessed.

## Recommendation contract

```json
{
  "schema_version": 1,
  "game_id": "hearthstone-battlegrounds",
  "session_id": "uuid",
  "observation_id": "uuid",
  "created_at": "RFC3339",
  "summary": "…one-line advice…",
  "actions": [ { "priority": 1, "action": "buy", "target": "…", "condition": null } ],
  "rationale": "…",
  "confidence": 0.87,
  "evidence": {
    "observed_facts": ["gold=7", "tier=3"],
    "strategy_refs": ["battlegrounds/<season>/economy-and-leveling"],
    "model_inferences": ["opponent likely ahead on tempo"]
  },
  "warnings": [],
  "validity": {
    "expires_at": null,
    "invalidated_by": ["shop_changes", "board_changes", "phase_changes", "turn_ends"]
  }
}
```

The four-way evidence split (observed / strategy / inference / user-provided)
is a product principle: the GUI renders the split, and validation rejects
recommendations whose `strategy_refs` cite documents that were not retrieved
this turn.

## Match session state

`augur-observation` keeps per-match state: `begin_match(MatchIdentity)`,
`append(envelope)`, `latest(session)`, `end_match`. Backed by SQLite beside
the ZeroClaw session store (`<workspace>/sessions/`), separate table
namespace, so match history survives daemon restarts and feeds "recent
recommendation history" context. Explicit match state — not open-ended agent
memory — is the source of truth (product rule; `zeroclaw-memory` is not used
for this).

## Invalidation

A recommendation dies when: its `observation_id` is superseded by an envelope
whose `change_summary` intersects `invalidated_by`; the phase changes; the
turn ends; or `expires_at` passes. Death is an event (`advice_invalidated` on
the `augur/event` stream) so GUI, overlay, and TTS all react; the overlay
renders stale advice visibly struck, never silently fresh.

## Bounded coaching turn

1. Receive validated observation → 2. compute strategy query → 3. retrieve
bounded refs (decision 0003) → 4. construct delimited context → 5. request
structured recommendation → 6. validate schema → 7. validate citations →
8. check observation currency → 9. publish or discard → 10. record latency,
cost, provider, confidence in the metrics ledger.

Structured output uses the upstream `Tool` structured-output path
(`ToolOutput{text,data}` / `output_schema()`, `zeroclaw-api/src/tool.rs`) or
schema-validated JSON from the model, whichever the provider supports better;
the validation step is provider-agnostic either way.
