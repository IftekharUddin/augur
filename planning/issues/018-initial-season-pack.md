## Problem

The season pack is a placeholder skeleton (all drafts, `status: skeleton`
manifest, placeholder season id). The MVP needs a real, sourced,
current-season corpus.

## Context and repository evidence

games/hearthstone-battlegrounds/strategies/season-2026-08/README.md
(placeholder contract); strategy-review.md (draft→reviewed ladder);
strategy-packs.md lifecycle.

## Scope

Rename the season dir to the live patch identifier; populate fundamentals,
economy-and-leveling, tempo, transitions, positioning + initial
heroes/tribes coverage for the current meta with sources and honest
confidence; manifest patch range; promote to `reviewed` what a strategy
maintainer actually reviews.

## Non-goals

Complete hero/minion coverage (grows continuously); `stable` promotions in
the first pass.

## Acceptance criteria

- `augur strategy validate` green; retrieval smoke returns sensible docs for
  canonical scopes (early-game economy, a popular tribe build).
- Every document sourced; zero skeleton TODO markers left in retrievable
  docs.
- ≥1 document per category the retrieval fallback chain touches
  (fundamentals mandatory).

## Dependencies

#strategy-validation-cli. Content-parallel with everything else.

## Test plan

Validation CI + retrieval fixtures updated to the real ids.

## Documentation impact

Season README replaced by real season notes.

## Security, privacy, and policy considerations

Attribution for adapted community knowledge (plagiarism rule in
strategy-review.md); no copyrighted text lifted.
