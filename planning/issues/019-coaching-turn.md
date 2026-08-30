## Problem

The bounded coaching turn (state-and-recommendations.md ten steps) doesn't
exist: system prompt, context assembly, structured recommendation,
validation (schema + citations + observation currency), publish/discard.

## Context and repository evidence

- Turn engine to drive: `run_tool_call_loop`
  (`crates/zeroclaw-runtime/src/agent/turn/mod.rs`); structured output seam
  `ToolOutput{text,data}`/`output_schema()` (`zeroclaw-api/src/tool.rs`);
  provider retry/fallback `ReliableModelProvider`.
- Prompts dir contract: games/hearthstone-battlegrounds/prompts/README.md
  (coach-system requirements list).
- Vision routing + `[IMAGE:]` handoff: #screenshot-vision-handoff.

## Scope

`augur-runtime` coaching pipeline: assemble delimited context (fixed system
prompt + envelope summary + retrieved strategy as quoted data + frame
marker); request schema-constrained recommendation; validate; bind to
observation_id; publish via `augur/coach/subscribe` or discard with reason;
record turn metrics. Author `coach-system.md`, `recommendation-review.md`
per the prompts README requirements (concise, cited, uncertainty-honest,
injection-resistant, no-automation).

## Non-goals

Voice prompts (M3); multi-turn dialogue.

## Acceptance criteria

- Trace-replay tests (zeroclaw-eval): valid recommendation on good input;
  citation-violation and stale-observation cases discarded with recorded
  reasons.
- Recommendation card renders from a real provider round-trip on the
  manual-trigger path.

## Dependencies

#strategy-retrieval-mvp, #common-envelope-schemas,
#screenshot-vision-handoff, #augur-rpc-extensions.

## Test plan

LlmTrace fixtures per outcome class; one live-marked provider test.

## Documentation impact

state-and-recommendations.md bounded-turn marked implemented; prompts
committed beside their tests.

## Security, privacy, and policy considerations

The prompt's injection-resistance instructions get regression fixtures in
#injection-defense-fixtures; tool registry for the coaching agent excludes
everything not needed (sealed via `ScopedToolRegistry`).
