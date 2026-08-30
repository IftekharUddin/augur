## Problem

The Battlegrounds `state` payload schema and the extraction prompts don't
exist; the MVP field classification (mvp.md table) needs to become a schema
plus prompts that fill it honestly (missing stays missing).

## Context and repository evidence

mvp.md observation-scope table (Required/optional/deferred/unreliable);
state-and-recommendations.md envelope rules;
games/hearthstone-battlegrounds/schemas/README.md;
prompts/README.md (observation-repair described).

## Scope

`game-state.schema.json` (phase, hero, health/armor, tier, gold, shop, board,
hand?, frozen?, timer?, lobby tribes, per-field confidence+evidence);
extraction prompt + observation-repair prompt; adapter `parse_observation`
delegating vision extraction; deterministic-region OCR expressly deferred
(follow-up issue when fixture evidence identifies stable regions).

## Non-goals

Combat-phase depth; opponent inference; log-file augmentation (policy-gated).

## Acceptance criteria

- Schema validates hand-authored fixtures (valid + invalid sets).
- Extraction against the fixture screenshot set fills Required fields at the
  baseline accuracy the eval harness records (no target invented here —
  measured, then ratcheted).
- Absent data provably absent (no hallucinated nulls) in adversarial
  fixtures (occluded/mid-animation frames).

## Dependencies

#common-envelope-schemas, #extraction-eval-harness (co-developed).

## Test plan

Fixture corpus + per-field accuracy report from the harness.

## Documentation impact

schemas README; prompts committed with tests.

## Security, privacy, and policy considerations

Prompts instruct ignoring instructions rendered in-game (injection surface);
fixtures privacy-reviewed.
