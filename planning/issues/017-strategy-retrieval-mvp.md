## Problem

Decision 0003's pipeline (exact filters → entity filters → lexical rank →
caps → citation post-check → fundamentals fallback) is unimplemented.

## Context and repository evidence

docs/decisions/0003-strategy-retrieval.md; strategy-packs.md retrieval
section; FTS5 precedent `crates/zeroclaw-infra/src/session_sqlite.rs`;
`knowledge_bundles` confirmed dead (do not build on it); rag/mod.rs pattern.

## Scope

`augur-strategy` retrieval over the CLI-built index: `StrategyScope` in →
ranked `StrategyDocument` refs out, with document+token caps; retrieval
trace (what was filtered/ranked/why) recorded for evaluation; exposed to the
coaching turn as a runtime tool (registered via `all_tools()` →
`ScopedToolRegistry::assemble`) or direct call — decided in-issue with a
one-paragraph note.

## Non-goals

Embeddings/hybrid (post-MVP, evidence-gated); agent-driven multi-hop
search.

## Acceptance criteria

- Relevance tests: scoped queries return must-include ids, exclude
  deprecated/wrong-season/draft-by-default (fixture pack).
- Caps enforced; trace recorded; deterministic given index+query.
- p50 retrieval latency measured against the 30ms budget on the skeleton
  corpus × 100 duplicated docs.

## Dependencies

#strategy-validation-cli (index), #common-envelope-schemas (scope types).

## Test plan

tests/retrieval fixture suite per testing-and-evaluation.md.

## Documentation impact

Decision 0003 Sources gain measured numbers.

## Security, privacy, and policy considerations

Retrieved text enters prompts delimited as data (injection framing).
