## Problem

The fork still carries upstream identity in places users and tooling read:
Cargo package metadata (`description`, `repository`, binary name), tray
`productName: "ZeroClaw"` / identifier `ai.zeroclawlabs.desktop`
(`apps/tauri/tauri.conf.json`), AGENTS.md/CLAUDE.md instructions, and the
upstream-auto-labeled bug/feature issue templates. Attribution must stay
intact while identity changes.

## Context and repository evidence

- NOTICE, README.md, CODEOWNERS, SECURITY.md, CONTRIBUTING.md were adapted in
  the founding planning PR; LICENSE-MIT/LICENSE-APACHE intentionally
  untouched.
- Upstream NOTICE's trademark claim ("only official ZeroClaw repository…")
  obligates rebranding away from ZeroClaw marks.
- Upstream remote and sync policy: docs/architecture/upstream-sync.md.

## Scope

Rename user-facing identity (repo description/topics/homepage, workspace
`Cargo.toml` root-package metadata where product-facing, desktop identifiers)
to Augur; audit remaining "ZeroClaw"-branded user-facing strings; keep
runtime crate names (`zeroclaw-*`) unchanged (they are upstream code, not
product identity); document the `upstream` remote and run one sync drill
(fetch + clean merge) to prove the workflow.

## Non-goals

Renaming inherited crates or binaries used internally; rewriting history;
touching the mdBook.

## Proposed approach

Inventory via `grep -ri zeroclaw` filtered to product-facing surfaces; PR per
surface class; record fork-touchpoints in
docs/architecture/fork-touchpoints.md as modifications land.

## Acceptance criteria

- Repo metadata (description, topics) says Augur; desktop bundle identifier
  is Augur's.
- `docs/architecture/fork-touchpoints.md` exists listing every modified
  upstream file.
- One upstream sync merge completed and documented.
- NOTICE/LICENSE files verified against docs/architecture/zeroclaw-reuse-audit.md
  licensing row.

## Dependencies

None (M0 root).

## Test plan

`git merge upstream/master` drill on a branch; grep inventory checked into
the issue; existing CI (once adapted) green.

## Documentation impact

fork-touchpoints.md (new), upstream-sync.md (drill results).

## Security, privacy, and policy considerations

Trademark compliance (upstream NOTICE claim); no license terms altered.
