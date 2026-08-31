//! The observation envelope: what Augur believes the game looks like right now.
//!
//! The envelope is game-agnostic; the `state` it carries is game-owned and
//! validated against the adapter's own schema. That split is what lets a second
//! game arrive without touching platform code.
//!
//! Field-level contracts, the generated JSON Schema, and unknown-field
//! rejection land with the envelope-schema work; the shape here is the one
//! documented in `docs/architecture/state-and-recommendations.md`.

use serde::{Deserialize, Serialize};

use crate::evidence::{Confidence, FieldEvidence, Privacy};
use crate::ids::{AdapterVersion, GameId, ObservationId, SchemaVersion, SessionId};

/// Where a frame came from.
///
/// `frame_hash` is a hash, never pixels. Nothing in Augur persists frame
/// contents by default, and a type that could carry them would make that
/// promise unenforceable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-export", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct CaptureSource {
    /// How the frame was obtained, for example `screen_capture`.
    pub kind: String,
    /// Opaque, platform-specific window handle. Not a title, not a path.
    pub window_id: String,
    /// Content hash of the normalized frame, used for change detection.
    pub frame_hash: String,
}

/// Coarse game phase, as classified by the adapter.
///
/// A `String` rather than an enum on purpose: phases are game-specific
/// vocabulary, and an enum here would be a concrete game leaking into a
/// platform crate.
pub type ObservationPhase = String;

/// One observation of game state.
///
/// Every recommendation is bound to the [`ObservationId`] of the envelope it
/// was generated from. When a newer envelope's `change_summary` intersects a
/// recommendation's invalidation triggers, that recommendation is dead. See
/// [`crate::recommendation::Validity`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-export", derive(schemars::JsonSchema))]
// Unknown fields are rejected, not ignored. An envelope carrying a field this
// build does not understand is an envelope from a different contract, and
// silently dropping it is how a consumer ends up confidently wrong about a
// state it only partly read.
#[serde(deny_unknown_fields)]
pub struct GameStateEnvelope {
    /// Envelope schema version. See [`crate::ids::CURRENT_SCHEMA_VERSION`].
    pub schema_version: SchemaVersion,
    /// Which game this observation is of.
    pub game_id: GameId,
    /// Season or patch identifier, as the adapter reports it.
    pub game_version: String,
    /// Version of the adapter that produced this envelope.
    pub adapter_version: AdapterVersion,
    /// Coaching session this observation belongs to.
    pub session_id: SessionId,
    /// This observation's identity.
    pub observation_id: ObservationId,
    /// Adapter-classified phase, for example `shop` or `combat`.
    pub phase: ObservationPhase,
    /// Provenance of the frame this was extracted from.
    pub source: CaptureSource,
    /// Overall extraction confidence. Per-field confidence is in `evidence`.
    pub confidence: Confidence,
    /// What changed relative to the previous envelope. Drives invalidation.
    pub change_summary: Vec<String>,
    /// Retention classification for everything this envelope refers to.
    pub privacy: Privacy,
    /// Per-field extraction evidence.
    pub evidence: Vec<FieldEvidence>,
    /// The game-owned state payload.
    ///
    /// Deliberately untyped here. This crate must not know what a Battlegrounds
    /// board looks like, and the architecture test enforces that. The payload
    /// is validated against the adapter's own schema
    /// (`games/<id>/schemas/game-state.schema.json`), which is where a game's
    /// vocabulary belongs.
    pub state: serde_json::Value,
}
