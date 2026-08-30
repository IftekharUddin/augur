## Problem

Strategy packs are load-bearing data with no validator: front matter,
manifests, references, duplicate ids, patch ranges, active-season
uniqueness, injection-pattern static checks — all unenforced.

## Context and repository evidence

strategy-packs.md (front matter contract, lifecycle, trust controls);
season skeleton at games/hearthstone-battlegrounds/strategies/season-2026-08/
(documents with front matter, `manifest.yaml` with `documents: []` awaiting
the indexer); governance strategy-review.md (CI must prove strategy PRs
touch only strategy paths).

## Scope

`augur strategy validate` (new xtask-style CLI in `augur-strategy`):
schema-validate front matter + manifests; resolve `[[id]]` and
supersedes/superseded_by refs; duplicate-id and orphan detection; patch
range sanity; exactly-one-active-season; file-type/size allowlist;
suspicious-pattern static checks (flag, not auto-reject); `--index` emits
the FTS5 index + document list consumed by retrieval. CI job: run on
`games/**` PRs; separate check that strategy-labeled PRs touch only
strategy/fixture paths.

## Non-goals

Retrieval itself; semantic review (human).

## Acceptance criteria

- Skeleton pack validates; each rule has a red fixture proving it fires.
- Index output is deterministic (byte-stable across runs).
- CI wired per governance doc.

## Dependencies

#augur-crate-skeleton; #ci-adaptation.

## Test plan

Red/green fixture packs under the crate's tests.

## Documentation impact

strategy-packs.md lifecycle "runs locally and in CI" becomes true;
strategy-review.md contributor steps reference the real command.

## Security, privacy, and policy considerations

This is the injection-defense front line for community content; static
checks documented so authors aren't surprised.
