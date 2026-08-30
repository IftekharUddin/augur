# User Experience

## First-run

Product explanation → privacy explanation (what leaves the machine, to whom)
→ screen-recording permission (status + request; macOS FFI exists upstream)
→ optional microphone permission (deferred to first voice use) → provider
setup (guided; local and hosted options; advanced flow exposes ZeroClaw
provider aliases) → connectivity test → strategy-pack readiness → game
detection test → diagnostics link → the pledge: **Augur advises; it never
controls your game.**

## Main window

Detected game + window (manual selector fallback), game support status,
active season/patch, pack version + update status, runtime connection status,
provider selection, Start/Stop Coaching, current match state, latest
recommendation (summary, actions, confidence, evidence split, citations),
advice history, error/degraded banners, voice toggle + PTT binding (M3),
overlay toggle, privacy/retention controls, report-incorrect-advice, and
diagnostics export.

## Overlay (M2)

Transparent, always-on-top, click-through by default, movable/anchorable,
resizable, excluded from captures (structurally: sharingType /
WDA_EXCLUDEFROMCAPTURE), accessible (contrast-checked over light and dark
scenes, scalable text), showing one to three prioritized actions with
confidence and a stale/invalid indicator, strategy refs on demand, hotkey
quick-collapse, and auto-disabled where policy or platform requires.

## Tray and lifecycle

Tray icon (state: idle/coaching/error/disconnected — upstream ships four tray
PNGs and the swap pattern), start/stop, mute, PTT state, current game, quit
behavior (quit stops capture; daemon shutdown policy configurable), crash
recovery (socket-loss detection → reconnect/restart offer), update
notifications (once the updater exists), diagnostics access.

## Degraded and offline states (explicit, honest)

No supported game detected · screen-recording denied · microphone denied ·
runtime unavailable · provider not configured · model lacks vision · network
unavailable · strategy pack missing · pack stale for current patch ·
extraction uncertain · recommendation timed out · voice unavailable ·
unsupported patch · policy-disabled game.

Each is a distinct UI state with a next action. Cached general strategy and
prior advice may remain visible **marked stale**; Augur never implies live
awareness when capture or inference failed.
