#!/usr/bin/env bash
# Contract tests for augur_docs_links_gate.py.
#
# The gate is only worth having if it actually fails on a broken link, so the
# red case is asserted here rather than trusted. Runs against a throwaway git
# repository so the real documentation tree is never modified.

set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
gate="${repo_root}/scripts/ci/augur_docs_links_gate.py"

workdir="$(mktemp -d)"
trap 'rm -rf "$workdir"' EXIT

cd "$workdir"
git init --quiet .
git config user.email "ci@example.invalid"
git config user.name "CI"

mkdir -p docs/architecture docs/decisions

cat > docs/architecture/overview.md <<'MD'
# Overview

Resolves: [decision](../decisions/0001-example.md)
Resolves with anchor: [decision](../decisions/0001-example.md#context)
External, not resolved: [upstream](https://example.invalid/thing)
Bare fragment, not resolved: [top](#overview)

```text
Inside a fence, not a link: [nope](./does-not-exist.md)
```
MD

cat > docs/decisions/0001-example.md <<'MD'
# 0001 example

Back: [overview](../architecture/overview.md)
MD

git add -A
git commit --quiet -m "fixture"

echo "--- case 1: all links resolve (expect pass) ---"
if ! python3 "$gate" --repo-root "$workdir"; then
    echo "FAIL: gate rejected a document whose links all resolve"
    exit 1
fi

echo "--- case 2: broken relative link (expect failure) ---"
cat >> docs/architecture/overview.md <<'MD'

Broken: [missing](./no-such-file.md)
MD
git add -A
git commit --quiet -m "break a link"

if python3 "$gate" --repo-root "$workdir"; then
    echo "FAIL: gate accepted a broken relative link"
    exit 1
fi

echo "--- case 3: untracked files are out of scope (expect pass) ---"
git revert --quiet --no-edit HEAD >/dev/null
cat > docs/architecture/untracked.md <<'MD'
[missing](./no-such-file.md)
MD
if ! python3 "$gate" --repo-root "$workdir"; then
    echo "FAIL: gate inspected an untracked file"
    exit 1
fi

echo "augur_docs_links_gate contract tests passed."
