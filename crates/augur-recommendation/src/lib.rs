//! Recommendation validation and staleness.
//!
//! This crate answers one question: may this advice be shown to the player
//! right now? Three ways the answer is no, and each is checked rather than
//! trusted:
//!
//! 1. The advice cites a strategy document that was not retrieved this turn.
//! 2. The observation it was generated from has been superseded by a change it
//!    declared itself sensitive to.
//! 3. The game adapter's own checks rejected it as impossible against the
//!    observed state.
//!
//! The first is the anti-fabrication check and it is why citations travel as a
//! separate list rather than as prose.
//!
//! # Status
//!
//! Milestone 0 defines the seam and the invalidation vocabulary.

use augur_core::{GameStateEnvelope, ObservationId, Recommendation};
use augur_strategy::StrategyDocumentId;

/// Why a recommendation is no longer showable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidationReason {
    /// A newer observation reported a change this advice declared itself
    /// sensitive to.
    StateChanged {
        /// The observation that superseded it.
        superseded_by: ObservationId,
        /// The change kinds that matched `validity.invalidated_by`.
        changes: Vec<String>,
    },
    /// The advice's wall-clock validity elapsed.
    Expired,
    /// The match ended.
    MatchEnded,
}

/// The outcome of validating one recommendation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationOutcome {
    /// Safe to publish.
    Accepted,
    /// Refused, with reasons a developer can act on.
    Rejected {
        /// One entry per failed check.
        reasons: Vec<String>,
    },
}

/// Validates a recommendation against the turn that produced it.
pub trait RecommendationValidator: Send + Sync {
    /// Check schema, citations, and observation currency.
    ///
    /// `retrieved` is the exact set of documents this turn was given. A
    /// citation outside that set is a fabrication and must be rejected, not
    /// warned about.
    fn validate(
        &self,
        observation: &GameStateEnvelope,
        recommendation: &Recommendation,
        retrieved: &[StrategyDocumentId],
    ) -> ValidationOutcome;
}
