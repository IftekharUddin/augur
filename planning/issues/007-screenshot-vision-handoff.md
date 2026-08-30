## Problem

Tool-produced images reach vision models unevenly, and the upstream
screenshot tool actively wastes budget: it emits a raw base64 data URI that
Anthropic's adapter sweeps away, reaching vision only via the incidental
file-path rewrite. Augur's coaching turn must have one deliberate, portable,
budget-tuned image path.

## Context and repository evidence

- `crates/zeroclaw-tools/src/screenshot.rs` emits `data:` URI (dead weight);
  correct pattern is `image_info.rs:240` (`[IMAGE:<path>]` marker).
- `canonicalize_tool_result_media_markers_for`
  (`runtime/src/agent/history.rs`) rewrites printed paths to markers.
- Tool-result images survive only the newest round and only on Anthropic
  (`anthropic.rs:694`) / OpenAI-compatible (`compatible.rs:2458`); dropped on
  gemini/ollama/openrouter/copilot/codex/bedrock (verified per adapter).
- `sweep_residual_image_data` (`anthropic.rs:772`) destroys bare data URIs.
- Budgets: `MultimodalConfig` — `max_images=4`, `max_image_size_mb=5`,
  `max_image_turns=0`; stale-image stripping `multimodal.rs:593/646`.
- Vision routing: `agent/turn/vision_route.rs`,
  `[multimodal] vision_model_provider`.

## Scope

(a) Coaching turns attach frames as `[IMAGE:<path>]` on the user-role
message (portable path); (b) tuned `[multimodal]` defaults for coaching
profiles, with measurement; (c) upstream PR fixing the screenshot tool's
marker emission; (d) documented provider-capability matrix for tool-result
images.

## Non-goals

Rewriting provider adapters; continuous capture (M2).

## Proposed approach

Small runtime change in `augur-runtime` turn assembly + config preset;
upstream fix as a separate ZeroClaw PR from the maintainer's fork.

## Acceptance criteria

- A frame from disk reaches Anthropic, OpenAI-compatible, and Gemini
  routes correctly in system tests (mock providers asserting content
  blocks).
- Budget tuning documented with before/after token counts.
- Upstream PR opened and linked.

## Dependencies

#augur-crate-skeleton.

## Test plan

Provider-adapter unit tests with image-bearing histories; trace-replay
system test.

## Documentation impact

capture-and-observation.md "Entering the vision pipeline" updated with
measured numbers.

## Security, privacy, and policy considerations

Frames are transient; paths must live under the workspace so upstream
path-guard tooling applies.
