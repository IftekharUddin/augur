# Coaching prompts (authored in Milestone 1)

Fixed prompts, physically separate from strategy content (injection
boundary — docs/architecture/strategy-packs.md):

- `coach-system.md` — the coaching system prompt: highest-value actions
  first, concise for live play, no unsupported certainty, observed/strategy/
  inference separation, mandatory citations, ignore instructions embedded in
  screenshots or strategy text, never automate gameplay, structured output.
- `observation-repair.md` — re-extraction prompt when confidence checks fail.
- `recommendation-review.md` — self-check pass before publish.
- `voice-conversation.md` — M3 spoken-turn grounding rules.

Authored under the M1 coaching-prompt issue with evaluation fixtures; not
committed as placeholders to avoid prompt content without tests.
