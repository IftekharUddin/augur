# Security and Privacy

## Threat model

| Threat | Vector | Mitigation |
|---|---|---|
| Capture of unintended windows | Display-level capture | Window-scoped capture APIs only (ScreenCaptureKit filter / GraphicsCaptureItem); display capture is not a supported mode |
| Private notifications in frames | Overlapping windows in windowed mode | Window-scoped capture excludes other windows by construction; occlusion detection drops ambiguous frames |
| Provider data retention | Frames/context sent to model APIs | Onboarding discloses transmission per provider; provider choice includes local models (Ollama vision); no frames sent when coaching stopped |
| Microphone capture | Voice features | PTT default; visible mic indicator; audio never retained |
| Transcript retention | Voice features | Off by default; explicit opt-in; excluded from diagnostics by default |
| Strategy prompt injection | Community pack text | Fixed separate system prompts, delimited data framing, static suspicious-pattern checks, human review for `stable`, regression fixtures (see strategy-packs.md) |
| Screenshot prompt injection | Text rendered inside the game (chat, names) | Same data-not-instructions framing; coach prompt instructs ignoring embedded instructions; regression fixtures with adversarial frames |
| Malicious game adapters | Executable adapter code | Compile-time adapters only (decision 0002); per-game CODEOWNERS review; two-review rule for executable changes |
| Malicious strategy PRs | Data PRs | Validation CI + static checks + human review; lower-privilege path than code by construction |
| Local IPC exposure | Socket/pipe | OS permissions (0600/user ACL) per upstream contract; no TCP listener for local coaching |
| Unauthorized remote gateway access | Gateway enabled by user | Gateway not required for desktop; if enabled, upstream pairing/localhost gates apply unchanged |
| Credential storage | Provider keys | Upstream encrypted secrets (`enc2:` ChaCha20-Poly1305, `crates/zeroclaw-config/src/secrets.rs`); keyring backend is an upstream seam (`KeySource`) |
| Log leakage | Frames/keys in logs | Upstream `record!` pipeline: tool-I/O redaction default-on, LLM payload capture default-off, leak detector, ephemeral-credential quarantine (`crates/zeroclaw-log`) |
| Diagnostic bundles | User-shared reports | Opt-in, previewable before send; frames/audio excluded by default |
| Automatic updates | Update channel compromise | No updater exists yet (verified upstream absence); when built: Tauri updater with signed artifacts, keys in CI secrets, decision-record gated |
| Supply chain | Dependencies | Inherit upstream `deny.toml` + audit workflows once CI is adapted |
| Fork drift from upstream security fixes | Divergence | Upstream-sync policy (upstream-sync.md): security fixes prioritized ahead of features |

## Trust boundaries

Desktop webview ↔ Tauri native: capability-scoped IPC (upstream precedent:
`apps/tauri/tests/capability_security.rs` proves remote content gets no IPC).
Tauri native ↔ Augur runtime: local RPC socket, OS-authenticated.
Runtime ↔ model/speech providers: TLS, keys from encrypted config, disclosed.
Runtime ↔ strategy repo: validated data only.
Runtime ↔ captured window: read-only pixels; input injection is prohibited by
product rule and no input-synthesis code path exists.
Runtime ↔ filesystem: workspace-scoped via upstream `SecurityPolicy`
(`workspace_only`, `forbidden_paths`).

## Privacy defaults

- Capture only the selected game window; visible active-capture indicator;
  one-click stop.
- Screenshots transient (temp files content-addressed, deleted after turn +
  startup sweep); never persisted by default.
- No audio retention; transcripts opt-in.
- Secrets redacted from logs (upstream leak detector + policies).
- Provider transmission disclosed; feedback uploads opt-in and previewable.
- No telemetry phoning home; metrics are local.

## The coach is not a bot — enforcement, not just policy

- No gameplay input APIs are linked in any Augur crate (architecture test
  greps for input-synthesis symbols: `CGEventPost`, `SendInput`, `enigo`, …).
- The upstream `browser` tool's computer-use actions and the `applescript`
  capability are excluded from the coaching agent's tool registry via
  `SecurityPolicy.allowed_tools` (sealed by `ScopedToolRegistry::assemble`).
- No process-memory reading, no code injection, no network manipulation, no
  anti-cheat circumvention — enforced by review policy on `augur-capture` and
  the per-game policy review gate (docs/policy/game-policy-review.md).
