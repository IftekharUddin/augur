# Upstream Synchronization

Augur is a downstream fork of ZeroClaw
(https://github.com/zeroclaw-labs/zeroclaw) with full history preserved.
The repository is standalone on GitHub (not a fork-network member) by
deliberate choice — the maintainer's personal fork of ZeroClaw remains the
vehicle for upstream contributions; Augur is a product.

## Remote setup

```bash
git remote add upstream https://github.com/zeroclaw-labs/zeroclaw.git
git fetch upstream master
```

## Sync policy

- **Cadence**: fetch weekly; merge upstream `master` at least once per Augur
  release cycle; **security fixes are pulled immediately** on upstream
  advisory or a `security`-labeled upstream fix.
- **Merge, not rebase**: Augur's `master` merges `upstream/master`
  (`git merge upstream/master`). History is shared and published; rebasing is
  prohibited.
- **Conflict tracking**: each sync PR lists conflicted paths; recurring
  conflicts are a signal to move code across the fork boundary (into
  `augur-*` crates or upstream).

## Fork-boundary rules (what keeps sync cheap)

- Augur code lives in **added** paths: `crates/augur-*`, `apps/augur-desktop`,
  `games/`, `docs/{product,architecture,governance,policy,decisions}`,
  `planning/`. These never conflict.
- Modifications to upstream files are minimized and inventoried in
  `docs/architecture/fork-touchpoints.md` (created when the first touchpoint
  lands; expected: workspace `Cargo.toml` members, RPC dispatch registration
  until the upstream seam exists, root README/NOTICE/CODEOWNERS/.github).
- Upstream mdBook (`docs/book/`) is left untouched; Augur docs live outside
  it.
- No Augur-specific assumptions may enter generic runtime code — if a change
  is generic, it goes upstream via the maintainer's ZeroClaw fork; if
  Augur-specific, it lives in an `augur-*` crate.

## Version compatibility

Each Augur release records the upstream commit it builds on (workspace
version + merge commit). The kernel sidecar and desktop app ship together;
the RPC `initialize` protocol version is the compatibility contract
(mismatch → explicit UI state).

## Candidate upstream contributions

In order (rationale in zeroclaw-reuse-audit.md): screenshot-tool `[IMAGE:]`
marker fix; RPC method-registration seam; uniform/documented tool-result
image capability across providers; macOS permission FFI as a helper crate;
daemon lifecycle API polish; per-model vision capability reporting. Each is
tracked as an Augur issue with an "upstream-candidate" note; none is assumed
accepted — Augur carries a thin compatibility layer either way.
