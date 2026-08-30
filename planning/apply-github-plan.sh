#!/usr/bin/env bash
# Idempotent GitHub setup for Augur, driven by planning/github-plan.yaml.
# Requires: gh (authenticated), python3, git. No credentials in this script.
#
# Usage: ./planning/apply-github-plan.sh [--repo OWNER/NAME] [--skip-pr]
# Re-running is safe: labels are upserted, milestones/issues are matched by
# title and skipped if present, dependency links are (re)written in bodies.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
REPO="IftekharUddin/augur"; SKIP_PR=0
while [[ $# -gt 0 ]]; do case "$1" in
  --repo) REPO="$2"; shift 2;; --skip-pr) SKIP_PR=1; shift;;
  *) echo "unknown arg: $1" >&2; exit 2;; esac; done
PLAN=planning/github-plan.yaml
py() { python3 - "$@"; }

echo "== labels =="
py <<'PY' | while IFS=$'\t' read -r n c d; do
import re,sys
s=open("planning/github-plan.yaml").read()
for m in re.finditer(r'- name: "([^"]+)"\n    color: "([^"]+)"\n    description: "([^"]*)"',s):
    print(f"{m.group(1)}\t{m.group(2)}\t{m.group(3)}")
PY
  gh label create "$n" --repo "$REPO" --color "$c" --description "$d" --force >/dev/null && echo "  label: $n"
done

echo "== milestones =="
EXISTING_MS=$(gh api "repos/$REPO/milestones?state=all&per_page=100" --jq '.[].title')
py <<'PY' | while IFS=$'\t' read -r t d; do
import re
s=open("planning/github-plan.yaml").read()
ms=s.split("milestones:")[1].split("issues:")[0]
for m in re.finditer(r'- title: "([^"]+)"\n    description: "((?:[^"\\]|\\.)*)"',ms):
    print(m.group(1)+"\t"+m.group(2).replace('\\"','"'))
PY
  if grep -qxF "$t" <<<"$EXISTING_MS"; then echo "  exists: $t"; else
    gh api "repos/$REPO/milestones" -f title="$t" -f description="$d" >/dev/null && echo "  created: $t"
  fi
done

echo "== issues (pass 1: create) =="
MAP=$(mktemp)
EXISTING=$(gh issue list --repo "$REPO" --state all --limit 500 --json number,title --jq '.[] | "\(.number)\t\(.title)"')
py <<'PY' > /tmp/augur-issues.tsv
import re
s=open("planning/github-plan.yaml").read().split("issues:",1)[1]
for m in re.finditer(r'- slug: (\S+)\n    title: "([^"]+)"\n    milestone: "([^"]+)"\n    labels: \[([^\]]*)\]\n    body_file: (\S+)',s):
    labels=",".join(x.strip().strip('"') for x in m.group(4).split(","))
    print("\t".join([m.group(1),m.group(2),m.group(3),labels,m.group(5)]))
PY
while IFS=$'\t' read -r slug title ms labels body; do
  num=$(awk -F'\t' -v t="$title" '$2==t{print $1; exit}' <<<"$EXISTING" || true)
  if [[ -n "${num:-}" ]]; then echo "  exists: #$num $slug";
  else
    url=$(gh issue create --repo "$REPO" --title "$title" --body-file "$body" --milestone "$ms" --label "$labels")
    num="${url##*/}"; echo "  created: #$num $slug"
  fi
  printf '%s\t%s\n' "$slug" "$num" >> "$MAP"
done < /tmp/augur-issues.tsv

echo "== issues (pass 2: rewrite #slug refs to numbers) =="
while IFS=$'\t' read -r slug title ms labels body; do
  num=$(awk -F'\t' -v s="$slug" '$1==s{print $2; exit}' "$MAP")
  tmp=$(mktemp)
  python3 - "$body" "$MAP" > "$tmp" <<'PY'
import sys
body=open(sys.argv[1]).read()
for line in open(sys.argv[2]):
    slug,num=line.rstrip("\n").split("\t")
    body=body.replace(f"#{slug}",f"#{num}")
open(1,"w").write(body)
PY
  if ! cmp -s "$tmp" "$body"; then gh issue edit "$num" --repo "$REPO" --body-file "$tmp" >/dev/null; echo "  linked: #$num $slug"; fi
  rm -f "$tmp"
done < /tmp/augur-issues.tsv
rm -f "$MAP" /tmp/augur-issues.tsv

if [[ "$SKIP_PR" -eq 0 ]]; then
  echo "== planning PR =="
  if gh pr view feat/augur-planning-foundation --repo "$REPO" >/dev/null 2>&1; then echo "  exists"
  else
    gh pr create --repo "$REPO" --base master --head feat/augur-planning-foundation \
      --title "docs(planning): establish Augur architecture, roadmap, and governance plan" \
      --body-file planning/pr-body.md
  fi
fi
echo "done."
