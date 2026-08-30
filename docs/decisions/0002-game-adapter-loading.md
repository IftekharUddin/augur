---
id: 0002
title: Compile-time game-adapter registry, data-only strategy packs
date: 2026-08-29
status: proposed
relates-to:
  - crates/zeroclaw-plugins
  - wit/VERSIONING.md
  - docs/architecture/game-adapter.md
---

# 0002 — Compile-time game-adapter registry, data-only strategy packs

## Context

Augur must support additional games without rewriting the platform. The
loading mechanism for game adapters (executable code) was evaluated against
ZeroClaw's three existing extension mechanisms:

1. **Compile-time trait implementations** — how all first-party ZeroClaw
   tools/channels/providers work (upstream ADR-002 "Trait-driven
   extensibility").
2. **WASM component plugins** (`crates/zeroclaw-plugins`, wasmtime, WIT ABI).
3. **Subprocess extensions** (MCP servers over stdio).

Repository evidence on the WASM path:

- The ABI is explicitly unstable: everything in `wit/v0` is
  `@unstable(feature = plugins-wit-v0)`; `wit/v0/.frozen` does not exist;
  `wit/v0/README.md` says the ABI "can be freely modified until the Component
  Model ABI ships."
- The host is heavy (~9,750 lines in `crates/zeroclaw-plugins/src/`): fuel
  metering, wall-clock deadlines, egress allowlists, schema-compile DoS
  bounds, Ed25519 manifest signatures.
- The signature covers the **manifest only**, not the `.wasm` binary
  (`docs/book/src/plugins/distributing-plugins.md`).
- One tool per tool-plugin; 32-bit guest address space; large payloads cross
  by value — poor fit for high-rate frame/observation data.

## Decision

**Game adapters are compile-time Rust crates registered in a static registry;
strategy packs are data-only (Markdown + YAML) and are the low-barrier
community extension path.**

- Each game lives under `games/<game-id>/` with its adapter crate at
  `games/<game-id>/adapter/` (a workspace member) and its data (manifest,
  schemas, prompts, strategies, fixtures) beside it.
- The registry lives in `augur-game-api` consumers; adding a game must not
  require editing a switch statement across many crates — one registration
  entry plus the crate.
- Strategy packs must never contain executable code, remote includes, or shell
  commands; they are validated data (see decision 0003 and
  `docs/architecture/strategy-packs.md`).
- Dynamically loaded native plugins are **rejected** for now (code signing,
  ABI stability, and sandboxing costs are not justified before a second game
  exists). Revisit only after the GameAdapter API is versioned and a second
  game has shipped.

## Consequences

- Third-party games require a PR (and per-game CODEOWNERS review), not a
  binary drop — this is an accepted trade-off; it matches the security posture
  and keeps the trust model simple.
- The adapter API can evolve freely pre-1.0 without ABI adapters.
- The second-game proof (Milestone 4) validates that the registry and
  dependency-direction rules actually hold, enforced by an
  architecture-invariant test in the style of upstream
  `tests/test_architecture.rs`.
