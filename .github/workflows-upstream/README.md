# Archived upstream workflows

These are the ZeroClaw workflows Augur inherited at fork time. They are parked
here, not deleted: they are the reference implementations Augur's own CI grows
back toward, and several of them will return once the subsystems they gate
exist.

**Nothing in this directory runs.** GitHub only schedules workflow files under
`.github/workflows/`, so moving a file here is the disable. That is deliberate:
`if: false` guards leave a workflow visible in the Actions UI as a permanently
skipped check, which reads as "passing" to a reviewer scanning a pull request.

The live Augur gate is `.github/workflows/ci.yml` (one required check,
`Augur Required Gate`), plus `.github/workflows/pr-title.yml`, which was kept
running unchanged.

## Why they were parked

Three distinct reasons, and they matter when deciding what to revive:

1. **Wrong runners.** `ci.yml` and `release-stable-manual.yml` select
   `blacksmith-8vcpu-ubuntu-2404`, a self-hosted runner pool belonging to the
   upstream organization. Augur has no access to it; those jobs would queue
   until they time out rather than fail fast.
2. **Wrong publishing identity.** Everything that pushes a package, an image, a
   docs site, or a social post is bound to upstream's registries, domains, and
   secrets (AUR, Scoop, Docker Hub, GitHub Pages via `CNAME`, Discord, X).
   Running these from the fork would either fail on missing secrets or, worse,
   publish under upstream's identity.
3. **Gates for subsystems Augur has not built yet.** Release packaging, desktop
   bundling, and the scheduled platform matrices gate code paths Augur reaches
   in later milestones. They come back with the milestone that needs them.

## Inventory and revival criteria

| File | Upstream name | Parked because | Comes back when |
|---|---|---|---|
| `ci.yml` | Quality Gate | Self-hosted runners; 25-job upstream required-check set | Reference only. Augur's `ci.yml` re-adds jobs from it individually, on hosted runners, as crates land. |
| `desktop-check.yml` | Desktop App Check | Gates `apps/tauri`; Augur's desktop shell is a later milestone | Augur desktop shell work (roadmap M1) |
| `platform-tests.yml` | Scheduled Platform Tests | Heavy scheduled matrix for upstream's platform surface | Capture providers exist and need a macOS/Windows matrix (M1-M2) |
| `cross-platform-clippy.yml` | Cross-Platform Clippy | Same, for lint | With the platform matrix above |
| `cross-platform-build-manual.yml` | Cross-Platform Build | Manual upstream release-engineering aid | Augur release packaging (M4) |
| `release-stable-manual.yml` | Release Stable | Self-hosted runners plus upstream release identity | Augur release packaging (M4) |
| `docker-image-pr.yml`, `docker-publish.yml` | Docker Image PR Check, Docker Publish | Publish to upstream's image registry | Only if Augur ships container images; no milestone requires it today |
| `pub-aur.yml`, `aur-freshness-check.yml`, `pub-scoop.yml`, `scoop-bucket-canary.yml` | AUR / Scoop publishing | Publish to upstream-owned package repositories | Only with an explicit Augur distribution decision |
| `docs-deploy.yml` | Deploy mdBook docs to Pages | Publishes upstream's mdBook to upstream's domain (`CNAME`) | An Augur documentation site is decided on and given its own domain |
| `discord-release.yml`, `tweet-release.yml` | Discord Release, Tweet Release | Post to upstream's community channels | Never as-is; Augur announcements would need Augur channels and secrets |
| `codeql.yml`, `ci-code-analysis.yml`, `ci-sbom.yml`, `trivy-scheduled.yml` | Static analysis, SBOM, image scanning | Not parked for cause; deferred to keep the first Augur gate small and green | Once the Augur gate is stable; this is the intended next CI increment |
| `daily-audit.yml`, `daily-npm-audit.yml`, `monthly-outdated.yml`, `npm-deps-review.yml` | Dependency advisory scans | Same: deferred, not rejected | With the security-workflow increment above |
| `pr-path-labeler.yml` | PR Path Labeler | Labels against upstream's 19k-line `labeler.yml` path map | Governance activation (M4) rewrites the path map for Augur |
| `project-dashboard-plan.yml` | Project Dashboard Planner | Writes to an upstream GitHub Project | Only with an Augur project board |
| `validate-translations-pin.yml` | Validate Translations Pin | Gates the `docs/book/po` translation submodule, an upstream-only asset | If Augur ever localizes its own documentation |
| `master-branch-flow.md`, `README-upstream-workflows.md` | (documentation) | Describe the upstream workflow set | Reference only |

Dependabot is unaffected: `.github/dependabot.yml` is not a workflow and was
left in place.

## Rules for touching these files

- Reviving one means **moving it back** into `.github/workflows/`, replacing
  self-hosted runner selectors with hosted ones, and adding it to the `needs:`
  list of the `gate` job in `.github/workflows/ci.yml`. A check that is not in
  that list is not required, whatever it reports.
- **Four places in the repository read files in this directory by path** and
  must be flipped back if one is revived: `RELEASE_WORKFLOW` in
  `xtask/src/cmd/mdbook/hardware.rs`, the two workflow reads in
  `release_workflows_delegate_target_policy_to_generator`
  (`xtask/src/generate/spec.rs`), `WORKFLOW_PATH` in
  `scripts/ci/release_attestation_contract_test.py`, and
  `cross_platform_workflow` in `scripts/ci/install_release_tool.test.sh`.
  Upstream treats `release-stable-manual.yml` and
  `cross-platform-build-manual.yml` as the canonical prebuilt-binary target
  matrix, so its documentation generator and release-contract tests follow the
  file wherever it lives. They fail loudly, not silently, if the path is wrong.
- Do not edit files in this directory to "fix" them in place. They are the
  upstream baseline; keeping them byte-identical to upstream is what makes
  `git merge upstream/master` cheap. Fixes belong in the revived copy.
- Every file here is an inherited upstream file. The move itself is recorded in
  [`docs/architecture/fork-touchpoints.md`](../../docs/architecture/fork-touchpoints.md).
