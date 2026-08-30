# Augur Decision Records

This directory holds Augur-level decision records: durable, numbered records of
architecture choices that constrain future Augur work.

Upstream ZeroClaw keeps its ADRs in `docs/book/src/architecture/decisions/`
(`ADR-NNN-slug.md`). Augur deliberately keeps its own records **outside** the
mdBook tree so that:

1. Upstream syncs never conflict with Augur decisions.
2. Augur decisions are clearly scoped to the fork, not proposals to upstream.

## Conventions

- Files are named `NNNN-kebab-title.md`, numbered from `0001`.
- Frontmatter follows the upstream ADR shape (`id`, `title`, `date`, `status`,
  `relates-to`).
- Statuses: `proposed` → `accepted` → (`superseded by NNNN`).
- **Accepted records are immutable.** If the architecture changes, write a new
  record and mark the old one superseded — never rewrite history.
- A record is accepted by merging the PR that flips its status, after review by
  the owners listed in `CODEOWNERS` for `/docs/decisions/`.

## Index

| ID | Title | Status |
|----|-------|--------|
| [0001](0001-runtime-integration.md) | Sidecar daemon over local JSON-RPC, not library embedding | proposed |
| [0002](0002-game-adapter-loading.md) | Compile-time game-adapter registry, data-only strategy packs | proposed |
| [0003](0003-strategy-retrieval.md) | Staged strategy retrieval: metadata filters + lexical first | proposed |
