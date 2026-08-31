# Fork Touchpoints

Every modification Augur makes to a file it inherited from ZeroClaw is
recorded here. This is the inventory that
[upstream-sync.md](upstream-sync.md) requires: it is what tells a future sync
reviewer where `git merge upstream/master` can conflict, and *why* the
divergence exists, without reading the whole diff.

**Rule.** Augur code belongs in added paths: `crates/augur-*`,
`apps/augur-desktop`, `games/`, `docs/{product,architecture,governance,policy,decisions}`,
`planning/`, and new `scripts/ci/augur_*` gates. Those never conflict and are
not listed here. A change to any pre-existing upstream file is a touchpoint,
must be justified, and must be added to the table below in the same pull
request that makes it.

Columns:

- **Path**: the inherited file.
- **Kind**: `modified` (content changed in place), `moved` (relocated,
  content unchanged), or `replaced` (upstream content substituted wholesale at
  an upstream path).
- **Conflict risk**: how likely an upstream change to this file collides with
  Augur's, and how expensive the resolution is.

## Inventory

| Path | Kind | Landed in | Why | Conflict risk |
|---|---|---|---|---|
| `README.md` | modified | Planning PR (#41) | Product identity: Augur is a game-coaching product, not the ZeroClaw CLI. Upstream's README is preserved verbatim at `docs/ZEROCLAW-README.md`. | High but cheap. Upstream edits are almost always wholly superseded, so take Augur's side. |
| `NOTICE` | modified | Planning PR (#41) | Attribution to ZeroClaw retained; upstream's trademark claim requires Augur not to present itself as an official ZeroClaw repository. | Low. Re-read on any upstream `NOTICE` change; licensing judgment calls escalate to the maintainer. |
| `SECURITY.md` | modified | Planning PR (#41) | Vulnerability reports must reach Augur's maintainer, not upstream's. | Low. |
| `CONTRIBUTING.md` | modified | Planning PR (#41) | Adds Augur's lanes (strategy packs, game adapters) on top of inherited Rust conventions. | Medium. Upstream conventions still apply to Rust work; merge both sides rather than choosing one. |
| `.github/CODEOWNERS` | modified | Planning PR (#41) | Upstream's 275-line ownership map named upstream teams that do not exist here; replaced with Augur ownership plus per-game owners. | Low. Augur's map is authoritative. |
| `.github/ISSUE_TEMPLATE/config.yml` | modified | Planning PR (#41) | Points contributors at Augur's discussion surfaces. | Low. |
| `.github/workflows/ci.yml` | replaced | CI adaptation (#8) | Upstream's 25-job gate targets the self-hosted `blacksmith-8vcpu-ubuntu-2404` pool and upstream's required-check names. Replaced by the Augur gate on GitHub-hosted runners. Upstream's file is preserved unmodified at `.github/workflows-upstream/ci.yml`. | High. On sync, upstream `ci.yml` changes land as conflicts on a file Augur no longer uses. Resolve by keeping Augur's, and mirror any genuinely useful new job into the archived copy. |
| `.github/workflows/*` (26 workflows + `README.md`, `master-branch-flow.md`), moved to `.github/workflows-upstream/` | moved | CI adaptation (#8) | GitHub only schedules workflows under `.github/workflows/`, so the move is the disable. Content is byte-identical to upstream; rationale and revival criteria are in [`.github/workflows-upstream/README.md`](../../.github/workflows-upstream/README.md). | Medium. Renames make upstream edits arrive as adds under the old path; delete the re-added copy and apply the change to the archived file. |
| `.github/workflows/pr-title.yml` | *not modified* | n/a | Listed for the reader's benefit: it is inherited, still live, and deliberately unchanged. Conventional Commits with a required scope is Augur's rule too. | None. |

## Deliberately untouched

Recorded so a later contributor does not "tidy" them:

- `LICENSE-MIT`, `LICENSE-APACHE`: never modified. Augur inherits the dual
  license unchanged.
- `docs/book/**`: upstream's mdBook, including the `docs/book/po` translation
  submodule. Augur documentation lives outside it.
- `crates/zeroclaw-*`, `apps/zerocode`, `apps/tauri`: upstream crate names are
  not product identity and are never renamed.
- `.github/dependabot.yml`: not a workflow; left running.

## Pending touchpoints

Anticipated by the architecture, not yet made. Listed so their arrival is not a
surprise:

| Path | Expected in | Why |
|---|---|---|
| Root `Cargo.toml` `members` | Augur crate skeleton | New `crates/augur-*` and `apps/augur-desktop` workspace members. |
| `crates/zeroclaw-runtime/src/rpc/dispatch.rs` | `augur/*` RPC methods | Upstream has no method-registration seam yet; the fork-local patch is isolated in one module and is the first upstream contribution candidate. |
