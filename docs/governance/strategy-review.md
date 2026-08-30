# Strategy Review Workflow

The low-barrier path: contribute knowledge, not code.

## For contributors

1. Fork; edit or add Markdown under
   `games/<game-id>/strategies/<season>/…` with valid front matter (template
   in each season README).
2. Run `augur strategy validate` locally (also runs in CI).
3. Open a PR — the **strategy contribution** template asks for sources and
   scope; new documents land as `status: draft`.

Strategy-only PRs need one in-scope strategy maintainer approval + green
validation CI. They never require Rust review, and CI proves they cannot:
the PR check fails if a strategy PR touches anything outside
`games/*/strategies/**` and `games/*/fixtures/**`.

## Review checklist (maintainers)

- Front matter valid, id unique, `applies_to` honest, patch range plausible.
- Claims sourced or clearly marked experiential; confidence field honest.
- No injection patterns (static checker output reviewed, not just green).
- No plagiarism (uncredited copies of others' guides are rejected;
  quotation with attribution and permission is fine).
- Supersedes/superseded_by links coherent.

## Status promotions

- `draft`: merged, retrievable only when the user opts into drafts.
- `reviewed`: an in-scope strategy maintainer has verified accuracy; listed
  reviewer + `last_reviewed` updated.
- `stable`: held to the "would not mislead a strong player" bar; eligible for
  default retrieval priority.
- `deprecated`: excluded from retrieval; kept for history.

## Attribution and provenance

Authors ride in front matter and in git history; recommendations cite
document ids, so in-app advice is traceable to its authors. Removal of
malicious content is immediate (see game-maintainers.md).
