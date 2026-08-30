# planning/

Machine-readable record of Augur's founding GitHub setup.

- `github-plan.yaml` — repository metadata, labels, milestones, and all 40
  issues (titles, labels, milestone, dependencies, epic membership, body
  file). Sufficient to reproduce the setup if GitHub state is lost.
- `issues/*.md` — the exact issue bodies. `#slug` references are rewritten
  to real issue numbers by the apply script after creation.
- `apply-github-plan.sh` — idempotent: upserts labels, creates missing
  milestones/issues (matched by title), rewrites dependency links, opens the
  planning PR. Safe to re-run.
- `pr-body.md` — the planning PR description.

The dependency graph and critical path are rendered in
[docs/roadmap.md](../docs/roadmap.md); `depends_on` in the YAML is the
authoritative edge list.
