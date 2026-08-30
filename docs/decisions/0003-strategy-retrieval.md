---
id: 0003
title: Staged strategy retrieval — metadata filters + lexical first
date: 2026-08-29
status: proposed
relates-to:
  - crates/zeroclaw-runtime/src/rag/mod.rs
  - crates/zeroclaw-infra/src/session_sqlite.rs
  - crates/zeroclaw-memory
  - docs/architecture/strategy-packs.md
---

# 0003 — Staged strategy retrieval: metadata filters + lexical first

## Context

The coach must ground advice in the seasonal strategy corpus without placing
an entire season into each prompt. Options compared: metadata-only retrieval,
full-text lexical search, embedding search, hybrid, and agent-driven tool
search (see `docs/architecture/strategy-packs.md` for the full matrix).

Repository evidence:

- ZeroClaw has **no generic markdown-corpus RAG**. The closest precedent is
  `crates/zeroclaw-runtime/src/rag/mod.rs` ("RAG pipeline for hardware
  datasheet retrieval") — a keyword/board-keyed markdown chunker, domain-locked
  to hardware.
- `knowledge_bundles` config exists but is dead: nothing reads
  `KnowledgeBundleConfig.sources` (verified by repo-wide grep).
- SQLite + FTS5 is an established in-repo pattern
  (`crates/zeroclaw-infra/src/session_sqlite.rs` builds an FTS5 mirror with
  triggers).
- Embedding, vector, and MMR-rerank infrastructure exists in
  `crates/zeroclaw-memory` (`embeddings.rs`, `vector.rs`, `rerank.rs`) but is
  coupled to the memory subsystem, requires an embedding provider, and adds a
  network dependency to every retrieval.

## Decision

**MVP retrieval is deterministic: exact metadata filtering from validated YAML
front matter, then lexical ranking over the surviving candidates, with a
strict document/token cap and citation post-verification. Embeddings are a
later, additive stage.**

Pipeline (Milestone 1):

1. Exact filters: game, active season, patch range, `status` (exclude
   `deprecated`; exclude `draft` unless the user opts in), phase.
2. Entity filters: recognized hero / tribes / minions / spells / mechanics
   against `applies_to`.
3. Lexical ranking (SQLite FTS5 index built by the strategy validation CLI)
   over remaining candidates.
4. Hard cap on retrieved documents and total tokens.
5. Strategy IDs travel through the model call; a post-generation check rejects
   recommendations citing documents that were not retrieved.
6. Fallback to season `fundamentals.md` when state recognition is incomplete.

Later (post-MVP, separate decision to accept): hybrid retrieval that adds an
embedding stage reusing `zeroclaw-memory` embedding/rerank infrastructure, only
if evaluation shows lexical retrieval misses relevant documents.

## Consequences

- Retrieval is testable with fixtures and runs offline — no provider needed,
  which keeps the degraded-mode story honest (cached general strategy remains
  available without network).
- Front-matter quality becomes load-bearing; the validation CLI and CI gate
  (Milestone 1) are prerequisites, not nice-to-haves.
- The corpus index is rebuilt at pack-validation time, not at runtime, so
  startup cost stays flat as seasons accumulate.
