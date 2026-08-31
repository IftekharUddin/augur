#!/usr/bin/env bash
#
# Augur desktop dependency gate.
#
# Copied from `scripts/ci/zerocode_no_zeroclaw_dep_gate.sh`, which upstream uses
# to hold `apps/zerocode` to the same rule. Decision 0001 makes the Augur
# desktop app an RPC-only surface: everything it knows arrives over the local
# JSON-RPC socket, not by linking backend crates. Augur adds `augur-runtime` to
# the forbidden list, because linking the Augur runtime layer would defeat the
# boundary just as thoroughly as linking ZeroClaw's.
#
# `tests/architecture/augur_desktop_rpc_only.rs` asserts the same rule from
# Rust and additionally greps the sources. Two gates on purpose: this one is
# what a reviewer runs by hand and what fails fast in CI without a compile.

set -euo pipefail

echo "==> augur-desktop gate: no zeroclaw-* or augur-runtime crate dependency"

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

manifest="apps/augur-desktop/Cargo.toml"

# `tomllib` is Python 3.11+. macOS still ships 3.9 as `python3`, so a
# contributor running the documented local checks on a Mac would otherwise get
# a bare ModuleNotFoundError traceback from inside a heredoc. Find an
# interpreter that actually works, and say something useful if none does.
interpreter=""
for candidate in python3 python3.14 python3.13 python3.12 python3.11 \
    /opt/homebrew/bin/python3 /usr/local/bin/python3; do
    if command -v "$candidate" >/dev/null 2>&1 \
        && "$candidate" -c "import tomllib" >/dev/null 2>&1; then
        interpreter="$candidate"
        break
    fi
done

if [ -z "$interpreter" ]; then
    echo "::error::no Python 3.11+ interpreter with tomllib was found; this gate needs one to parse ${manifest}" >&2
    echo "On macOS: brew install python@3.13, or run the gate in CI." >&2
    exit 1
fi

offending="$(
    "$interpreter" - "$manifest" <<'PY'
import sys
import tomllib

with open(sys.argv[1], "rb") as handle:
    manifest = tomllib.load(handle)

own_name = manifest.get("package", {}).get("name", "")

dep_tables = []
for key in ("dependencies", "dev-dependencies", "build-dependencies"):
    table = manifest.get(key)
    if isinstance(table, dict):
        dep_tables.append(table)

target = manifest.get("target")
if isinstance(target, dict):
    for cfg in target.values():
        if not isinstance(cfg, dict):
            continue
        for key in ("dependencies", "dev-dependencies", "build-dependencies"):
            table = cfg.get(key)
            if isinstance(table, dict):
                dep_tables.append(table)

found = set()

FORBIDDEN_PREFIXES = ("zeroclaw-", "zeroclaw_", "augur-runtime", "augur_runtime")


def flag(label):
    if label.startswith(FORBIDDEN_PREFIXES):
        found.add(label)


for table in dep_tables:
    for name, spec in table.items():
        if name == own_name:
            continue
        flag(name)
        # Cargo renamed dependencies declare the real crate under `package`
        # while the table key is an arbitrary local alias. Inspect both so a
        # rename like `kernel = { package = "augur-runtime" }` cannot slip past.
        if isinstance(spec, dict):
            package = spec.get("package")
            if isinstance(package, str):
                flag(package)

for name in sorted(found):
    print(name)
PY
)"

if [ -n "$offending" ]; then
    echo "::error file=${manifest}::augur-desktop must not depend on any zeroclaw-* or augur-runtime crate; found:"
    while IFS= read -r dep; do
        echo "  - ${dep}"
    done <<<"$offending"
    echo "augur-desktop is an RPC-only surface: everything it knows must come over the wire, not by linking backend crates."
    echo "See docs/decisions/0001-runtime-integration.md."
    exit 1
fi

echo "augur-desktop gate passed: no zeroclaw-* or augur-runtime dependencies."
