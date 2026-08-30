# Augur Vision

Augur is an easy-to-use, real-time AI coaching platform for video games,
built on the ZeroClaw agent runtime. It watches your game (with permission),
consults community-maintained, season-specific strategy, and tells you the
highest-value next action — in a desktop app, an optional overlay, and
eventually a spoken conversation.

**Augur is a coach, not a bot.** It never clicks, drags, or presses keys; it
never buys, sells, rerolls, targets, or moves anything; it never reads or
modifies game memory, injects code, manipulates traffic, circumvents
anti-cheat, or automates gameplay; and it never presents uncertain
observations as facts. Acceptable inputs: screen capture of the selected game
window, approved local files, user speech, user-provided logs, official APIs,
and explicitly permitted game data — always consistent with the game's terms.

## Product principles

1. Advice must be timely enough to matter.
2. Advice must expire when the underlying observation changes.
3. Strategy-grounded claims cite their strategy sources.
4. Model inference is visually distinguishable from repository-grounded
   strategy and from observed fact.
5. Game-specific code never leaks into shared platform layers.
6. Strategy-only contributions are substantially easier and safer than
   executable-code contributions.
7. User screenshots, audio, and transcripts are private by default.
8. Augur makes no gameplay decisions invisibly.
9. The desktop experience stays usable when the model provider is slow,
   unavailable, or misconfigured.
10. Every supported game has an explicit maintainer and support status.

## First game: Hearthstone Battlegrounds

An 8-player auto-battler with a shop phase that rewards exactly the kind of
knowledge a curated seasonal corpus captures: economy curves, tier timing,
composition direction, transitions, positioning. Turn-based decision points
give the latency budget room; the community already maintains rich seasonal
strategy content; and the read-only coach model fits the game's rules.

## Who it's for

- Players who want to improve with grounded, cited advice — not a black box.
- Strategy authors who want their seasonal knowledge to reach players in-game,
  with attribution.
- Game communities that want a coaching platform without writing a desktop
  app: a game adapter plus a strategy pack.

## What Augur hides

ZeroClaw's machinery — agents, tools, daemons, prompt construction — is
invisible to ordinary users. Provider configuration appears in an advanced
setup flow only. The user's mental model is: install → point at game →
Start Coaching → get advice.
