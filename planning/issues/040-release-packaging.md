## Problem

Augur has no release pipeline: the inherited one builds ZeroClaw-branded
artifacts, signs only macOS (conditionally), marks Windows/Linux desktop
jobs continue-on-error and unsigned, and ships no auto-updater (verified: no
tauri-plugin-updater anywhere).

## Context and repository evidence

Upstream `release-stable-manual.yml` (universal-macOS DMG: sign → notarize
→ staple, kernel smoke test; sidecar staging via
`scripts/desktop/prepare-kernel.sh`; SBOM + attestations);
zeroclaw-reuse-audit.md release row; security-and-privacy.md auto-update
threat row (updater must be signed + decision-gated).

## Scope

Augur release workflow: bundle identifiers/branding, macOS signed +
notarized DMG (secrets to be provisioned), kernel-sidecar smoke test
adapted, Windows MSI/NSIS as required artifacts (signing approach decided
in-issue: OV cert vs Azure Trusted Signing — cost/logistics recorded),
SBOMs, versioning tied to workspace version; auto-updater gets its own
decision record (ship-without-updater is the default until then);
CHANGELOG process (adopt upstream CHANGELOG-next.md convention or simpler —
decided in-issue).

## Non-goals

Package managers (brew/winget) — follow-ups; Linux desktop.

## Acceptance criteria

- Tagged build produces installable, signed macOS DMG and Windows
  installer; smoke tests pass; artifacts SBOM'd.
- Updater decision record exists (even if the decision is "not yet").

## Dependencies

#ci-adaptation; M2 desktop maturity.

## Test plan

Release dry-run workflow on a prerelease tag; install smoke on both
platforms.

## Documentation impact

Release runbook (new doc, adapted from upstream's
docs/book/src/maintainers/release-runbook.md structure).

## Security, privacy, and policy considerations

Signing keys in CI secrets only; attestations retained; update security
explicitly deferred-and-recorded rather than implied.
