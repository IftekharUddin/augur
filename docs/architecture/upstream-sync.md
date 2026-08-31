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

## Sync drill: 2026-08-30

The first real exercise of the policy above, run to find out whether the
fork-boundary rules actually hold rather than to confirm that they sound
sensible.

**Setup.** `upstream/master` at `bd8c73f14d`, merge base `4d47d7955d`
(the fork point), 18 upstream commits to absorb.

```bash
git fetch upstream master
git checkout -b drill/upstream-sync-2026-08-30 origin/master
git merge --no-commit --no-ff upstream/master
```

**Result: one conflict, in exactly the file
[fork-touchpoints.md](fork-touchpoints.md) predicted.**

```
Auto-merging .github/workflows/ci.yml
CONFLICT (content): Merge conflict in .github/workflows/ci.yml
Automatic merge failed; fix conflicts and then commit the result.
```

That file is the fork's only `replaced` touchpoint, and its inventory row
called it "High" risk with the resolution written down in advance. The
inventory earned its keep on its first outing.

**Resolution**, following the rule already recorded in the archive README:
Augur's gate wins at the live path, and upstream's edit is mirrored into the
archived baseline so it stays byte-identical to upstream.

```bash
git checkout --ours -- .github/workflows/ci.yml
git show upstream/master:.github/workflows/ci.yml > .github/workflows-upstream/ci.yml
```

Upstream's one change to that file in this range was
`ci(plugins): execute every live-config regression in the required job`, which
is now reflected in the archived copy.

**What did not go wrong, and why that matters.**

- The 26 archived workflows did **not** reappear under `.github/workflows/`.
  They were moved with `git mv`, so git tracked them as renames and merged
  upstream's edits into the archived copies without a conflict. Had they been
  deleted and re-created, every upstream workflow edit would have arrived as a
  re-added file at the old path, and the archive would silently drift.
- The merge touched 78 files and **none** of them were `Cargo.toml`,
  `Cargo.lock`, or anything under `tests/`. The two touchpoints in those files
  cost nothing this time.
- Augur's added paths (`crates/augur-*`, `games/`, `docs/{architecture,decisions,
  governance,policy,product}`, `planning/`) did not appear in the merge at all,
  which is the whole point of keeping new code in added paths.

**Gate result on the merged tree.** `cargo fmt --all -- --check` clean, workspace
clippy clean with warnings denied, and the test suite green apart from the
pre-existing `grok_cli` flake tracked separately. Upstream's 18 commits
introduce no new failure.

**Conclusion.** The boundary holds. The cost of a sync is currently one
predicted conflict with a pre-written resolution. If that number grows, the
policy above says to treat it as a signal to move code across the fork
boundary rather than to keep paying it.
