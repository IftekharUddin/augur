#!/usr/bin/env python3
"""Verify that relative links in Augur-owned Markdown resolve to real paths.

Augur's planning, architecture, decision, governance, and game documents form a
densely cross-linked set that no inherited gate covers: upstream's
`scripts/ci/docs_links_gate.sh` only inspects `docs/book/src/**`. A broken
relative link here is a silent documentation regression, so the Augur required
gate checks them on every run.

Scope is deliberately limited to Augur-authored trees (see `SCOPES`). Inherited
upstream documentation keeps its own upstream gates and is not re-litigated
here.

Only *relative* targets are resolved. External schemes (http, https, mailto,
tel), bare fragments, and protocol-relative URLs are out of scope: reachability
of the public internet is not something a required gate should depend on.
"""

from __future__ import annotations

import argparse
import os
import re
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path

# Augur-owned Markdown. Paths are repo-relative; a directory scope matches every
# `.md` beneath it.
SCOPES: tuple[str, ...] = (
    "docs/architecture",
    "docs/decisions",
    "docs/governance",
    "docs/policy",
    "docs/product",
    "docs/roadmap.md",
    "games",
    "planning",
)

EXTERNAL_SCHEME = re.compile(r"^[a-zA-Z][a-zA-Z0-9+.-]*:")

# Inline links: [text](target) and images ![alt](target). The target stops at
# whitespace so that `[x](path "title")` yields `path`.
INLINE_LINK = re.compile(r"!?\[(?:[^\]\\]|\\.)*\]\(\s*<?([^)<>\s]+)>?(?:\s+[^)]*)?\)")

# Reference definitions: `[label]: target "optional title"`.
REFERENCE_DEF = re.compile(r"^\s{0,3}\[(?:[^\]\\]|\\.)+\]:\s*<?([^\s<>]+)>?")

FENCE = re.compile(r"^\s*(`{3,}|~{3,})")


@dataclass(frozen=True)
class Broken:
    source: str
    line: int
    target: str
    resolved: str


def tracked_markdown(repo_root: Path) -> list[str]:
    """Every tracked `.md` file inside `SCOPES`, sorted and de-duplicated."""
    out = subprocess.run(
        ["git", "ls-files", "-z", "--", *SCOPES],
        cwd=repo_root,
        check=True,
        capture_output=True,
        text=True,
    ).stdout
    files = {p for p in out.split("\0") if p.endswith(".md")}
    return sorted(files)


def strip_code_fences(lines: list[str]) -> list[tuple[int, str]]:
    """Drop fenced code blocks; a link inside a sample is not a real link."""
    kept: list[tuple[int, str]] = []
    fence: str | None = None
    for number, line in enumerate(lines, start=1):
        match = FENCE.match(line)
        if fence is None:
            if match:
                fence = match.group(1)[0]
                continue
            kept.append((number, line))
        elif match and match.group(1)[0] == fence:
            fence = None
    return kept


def targets_in(line: str):
    for match in INLINE_LINK.finditer(line):
        yield match.group(1)
    reference = REFERENCE_DEF.match(line)
    if reference:
        yield reference.group(1)


def is_external(target: str) -> bool:
    return (
        target.startswith("#")
        or target.startswith("//")
        or bool(EXTERNAL_SCHEME.match(target))
    )


def check_file(repo_root: Path, relative: str) -> list[Broken]:
    path = repo_root / relative
    lines = path.read_text(encoding="utf-8").splitlines()
    broken: list[Broken] = []
    for number, line in strip_code_fences(lines):
        for raw in targets_in(line):
            if is_external(raw):
                continue
            # Anchors and query strings address a location inside the target,
            # not a different file.
            target = raw.split("#", 1)[0].split("?", 1)[0]
            if not target:
                continue
            target = target.replace("%20", " ")
            resolved = (path.parent / target).resolve()
            if not resolved.exists():
                broken.append(
                    Broken(
                        source=relative,
                        line=number,
                        target=raw,
                        resolved=os.path.relpath(resolved, repo_root),
                    )
                )
    return broken


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--repo-root",
        default=None,
        help="Repository root (default: `git rev-parse --show-toplevel`).",
    )
    args = parser.parse_args()

    if args.repo_root:
        repo_root = Path(args.repo_root).resolve()
    else:
        repo_root = Path(
            subprocess.run(
                ["git", "rev-parse", "--show-toplevel"],
                check=True,
                capture_output=True,
                text=True,
            ).stdout.strip()
        )

    files = tracked_markdown(repo_root)
    broken: list[Broken] = []
    for relative in files:
        broken.extend(check_file(repo_root, relative))

    print(f"==> Augur docs link gate: {len(files)} Markdown files in scope")

    if broken:
        for item in broken:
            print(
                f"::error file={item.source},line={item.line}::"
                f"broken relative link '{item.target}' "
                f"(resolves to missing {item.resolved})"
            )
        print(f"{len(broken)} broken relative link(s).")
        return 1

    print("All relative links resolve.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
