//! Game-agnostic core types for Augur.
//!
//! Everything here is common to every game: identifiers, the observation
//! envelope, the recommendation contract, and the confidence and evidence
//! vocabulary they share. Game-specific payloads never appear in this crate.
//! That separation is not a style preference: it is asserted by
//! `tests/architecture/augur_game_isolation.rs`, which fails the build if a
//! concrete game identifier reaches any platform crate.
//!
//! # Status
//!
//! Milestone 0 establishes the seams. The types below carry their identity,
//! their documentation, and the invariants that are already decided; their
//! field-level contracts, `serde` representations, and JSON Schemas land with
//! the envelope-schema work. Where a type is deliberately still a shell, its
//! documentation says so rather than pretending completeness.
//!
//! # Layering
//!
//! `augur-core` depends on no other Augur crate. Every other Augur crate may
//! depend on it.

pub mod evidence;
pub mod ids;
pub mod observation;
pub mod recommendation;

pub use evidence::{Confidence, ConfidenceError, FieldEvidence, Privacy};
pub use ids::{AdapterVersion, GameId, MatchId, ObservationId, SchemaVersion, SessionId};
pub use observation::{CaptureSource, GameStateEnvelope, ObservationPhase};
pub use recommendation::{Recommendation, RecommendationEvidence, RecommendedAction, Validity};
