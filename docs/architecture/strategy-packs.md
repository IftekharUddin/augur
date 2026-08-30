# Strategy Pack System

The coach's strategic grounding: version-controlled, human-maintained
Markdown + YAML in the repository. Retrieval decision:
[decision 0003](../decisions/0003-strategy-retrieval.md).

## Layout

```text
games/hearthstone-battlegrounds/
  game.yaml                    # game manifest
  maintainers.yaml             # adapter + strategy maintainers (feeds CODEOWNERS)
  README.md
  schemas/
    game-state.schema.json
    recommendation.schema.json
    strategy-frontmatter.schema.json
    strategy-pack.schema.json
  prompts/
    coach-system.md            # fixed system prompt — never mixed with strategy text
    observation-repair.md
    recommendation-review.md
    voice-conversation.md
  strategies/<season-id>/
    manifest.yaml              # season id, patch range, active flag, doc index
    README.md
    fundamentals.md
    economy-and-leveling.md
    tempo.md
    transitions.md
    positioning.md
    heroes/<hero-slug>.md
    tribes/<tribe-slug>.md
    minions/<minion-slug>.md
    spells/<spell-slug>.md
    mechanics/<mechanic-slug>.md
    curves/<curve-slug>.md
    matchups/<matchup-slug>.md
    examples/<scenario-slug>.md
  fixtures/{screenshots,observations,recommendations}/
  tests/{retrieval,extraction,advice}/
```

## Front matter

Validated against `strategy-frontmatter.schema.json`:

```yaml
schema_version: 1
id: battlegrounds/<season>/<category>/<slug>
game: hearthstone-battlegrounds
season: <season-id>
patch: { min: null, max: null }
title: <human-readable>
status: draft | reviewed | stable | deprecated
category: economy | hero | tribe | minion | positioning | transition | mechanic
applies_to: { heroes: [], tribes: [], minions: [], phases: [], tavern_tiers: [] }
tags: []
authors: []
reviewers: []
last_reviewed: YYYY-MM-DD
confidence: low | medium | high
sources: []
supersedes: []
superseded_by: []
known_exceptions: []
```

Cross-references between documents use ids (`supersedes`, `superseded_by`,
and inline `[[battlegrounds/<season>/…]]` links) — resolved and validated by
the CLI; no includes, no executable behavior of any kind.

## Retrieval pipeline

1. Exact filters: game, active season, patch compatibility, status
   (`deprecated` excluded always; `draft` excluded unless opted in), phase.
2. Entity filters via `applies_to` against recognized entities.
3. Lexical ranking (FTS5 index built at validation time) over survivors.
4. Strict cap on document count and total tokens.
5. Ids preserved through the model call; post-generation citation check.
6. Fundamentals fallback on incomplete recognition.

Comparison that produced this design (summary; full analysis in the M1
research issue): metadata-only is cheap but blind to content; pure lexical
misses entity structure; embeddings add a provider dependency, network
latency, and non-determinism to every turn for unproven gain at corpus sizes
of a few hundred documents; agent-driven tool search adds a model round-trip
inside a latency budget that cannot afford one. Hybrid (metadata + lexical
now, embeddings later behind evaluation evidence) captures the value without
the costs.

## Trust and prompt-injection defense

Strategy content is **untrusted reference material**:

- Fixed system prompts live in `prompts/`, owned by the adapter maintainers,
  physically separate from `strategies/`.
- Strategy text is delimited in the coaching context and framed as quoted
  reference data.
- Schema validation, maximum file size, allowed extensions (`.md`, `.yaml`),
  no remote includes, no shell commands, no tool-permission grants from
  strategy text.
- Static checks flag suspicious instruction patterns ("ignore previous",
  "system:", tool-call syntax, base64 blobs) for human review.
- `stable` status requires human review by a strategy maintainer
  (see [strategy-review](../governance/strategy-review.md)).
- Recommendations carry provenance (`strategy_refs`), so bad advice is
  traceable to the document and author that produced it.
- Regression fixtures: packs containing hostile instructions must not alter
  tool access, capture scope, provider credentials, or runtime permissions —
  asserted by tests in `games/*/tests/` plus an architecture-level test.

## Lifecycle

Documented procedures (governance owns the how-to):

- **New season**: `strategies/<new-season>/` scaffolded from a template;
  still-valid documents copied forward with `status: draft` and
  `last_reviewed` reset; season `manifest.yaml` declares patch range.
- **Active season selection**: exactly one season manifest is `active: true`
  per game; the validation CLI enforces it; the runtime reads only the active
  pack (plus explicit archives on request).
- **Patch update**: affected docs re-reviewed or bounded by `patch.max`.
- **Deprecation**: `status: deprecated` + `superseded_by`; retrieval excludes.
- **Disputes**: issue with the `type:strategy` label; resolution recorded in
  `known_exceptions` / `sources`; strategy maintainers arbitrate.
- **Archival**: old seasons stay in-tree (they are history), excluded by the
  active-season filter.
- Validation runs locally (`augur strategy validate`) and in CI on every PR
  touching `games/**`.
