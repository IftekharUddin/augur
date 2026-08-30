## Problem

Two untrusted text channels reach the model: strategy documents and pixels
(in-game chat/names rendered in frames). The defenses are designed
(strategy-packs.md, security-and-privacy.md) but nothing proves they hold.

## Context and repository evidence

strategy-packs.md trust section (delimiting, fixed prompts, static checks);
security-and-privacy.md threat rows; upstream ingress-policy precedent
(`security::ingress` in the turn loop) and prompt-injection config
(`prompt_injection_mode` on runtime profiles).

## Scope

Regression fixtures: hostile strategy docs (instruction smuggling, tool-call
syntax, credential requests, exfil URLs) and hostile frames (adversarial
text in chat regions) driven through the real coaching turn via trace
replay; assertions that tool access, capture scope, credentials, and output
schema are unaffected and that citations never launder hostile docs into
`stable` provenance. Wire the static-checker findings from
#strategy-validation-cli into review docs.

## Non-goals

Model-level jailbreak research; guaranteeing model behavior (defense in
depth is the claim, not immunity — worded accordingly in docs).

## Acceptance criteria

- Fixture suite in CI; each defense layer has at least one red test proving
  the assertion bites.
- security-and-privacy.md updated with "verified by" pointers.

## Dependencies

#coaching-turn, #strategy-validation-cli.

## Test plan

The suite is the plan; runs offline via replay.

## Documentation impact

Threat-model rows gain test pointers.

## Security, privacy, and policy considerations

This is the verification for the fork's central injection claims.
