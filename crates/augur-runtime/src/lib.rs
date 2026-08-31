//! The Augur runtime layer.
//!
//! This crate is where the platform crates and the game adapters are wired
//! together, and it is the **only** Augur crate permitted to name a concrete
//! game. That is the whole point of the arrangement: the registry below is the
//! single place a new game is mentioned, so adding one touches its own
//! `games/<id>/` directory, one line here, and the workspace member list.
//!
//! It also hosts the `augur/*` JSON-RPC methods the desktop app talks to. The
//! desktop app never links this crate; it reaches it over the local socket. See
//! `docs/decisions/0001-runtime-integration.md`, and
//! `tests/architecture/augur_desktop_rpc_only.rs`, which fails the build if
//! that ever stops being true.
//!
//! # Status
//!
//! Milestone 0 establishes the registry. The session coordinator, the
//! `augur/*` method table, and the coaching turn arrive with their own issues.

pub mod registry;

pub use registry::{AdapterRegistry, RegistryError};
