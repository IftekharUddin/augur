//! Augur desktop application.
//!
//! The player-facing surface: onboarding, capture-target selection, the
//! coaching dashboard, and later the overlay and push-to-talk controls.
//!
//! # The one rule
//!
//! This binary talks to the Augur runtime **only** over the local JSON-RPC
//! socket, and links neither `zeroclaw-*` nor `augur-runtime`. Not a
//! convention: a CI gate parses this crate's manifest and an architecture test
//! greps its sources.
//!
//! Three things follow from that boundary. A runtime panic cannot take the
//! interface down, and the reverse. Upstream refactors of ZeroClaw internals
//! cannot break the app, because the contract is a wire protocol rather than a
//! set of Rust signatures. And access control is the operating system's:
//! socket permissions on Unix, the named-pipe ACL on Windows, with no token to
//! store or leak.
//!
//! Upstream's `apps/zerocode` is the working precedent for a rich streaming
//! client that links nothing. See
//! `docs/decisions/0001-runtime-integration.md`.
//!
//! # Status
//!
//! Milestone 0 establishes the crate and the boundary it is gated on. The Tauri
//! shell, the frontend, and the RPC client arrive with the desktop-shell work.

fn main() {
    // Deliberately not `println!`-ing a fake readiness message. The shell is
    // not built; saying anything else here would make an unimplemented binary
    // look like a working one.
    std::process::exit(0);
}
