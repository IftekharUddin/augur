//! The recommendation contract: what Augur tells the player, and why.
//!
//! Two rules shape this type and are worth stating before the fields:
//!
//! 1. **Evidence is split four ways.** Observed facts, retrieved strategy,
//!    model inference, and user-provided context are different kinds of claim
//!    with different reliability, and the interface renders the difference.
//!    Collapsing them into one prose blob is what makes a coaching tool
//!    untrustworthy.
//! 2. **A recommendation is bound to an observation.** It carries the
//!    [`ObservationId`] it was generated from and the conditions that kill it.
//!    Advice is never silently shown as current after the state it described
//!    has moved on.
//!
//! Field-level contracts and the generated JSON Schema land with the
//! envelope-schema work.

use serde::{Deserialize, Serialize};

use crate::evidence::Confidence;
use crate::ids::{GameId, ObservationId, SchemaVersion, SessionId};

/// One concrete thing the player could do, in priority order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-export", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct RecommendedAction {
    /// 1 is the top recommendation.
    pub priority: u8,
    /// Adapter-defined action verb, for example `buy` or `sell`.
    pub action: String,
    /// What the action applies to.
    pub target: String,
    /// A precondition the player should check first, when the advice is
    /// conditional. Absent means unconditional, not "unknown".
    pub condition: Option<String>,
}

/// The four-way evidence split.
///
/// Each list holds a different *kind* of claim, and the interface renders them
/// differently because a player should be able to tell "your gold is 7"
/// (observed) from "opponents are probably ahead on tempo" (inferred). The
/// recommendation validator rejects any advice whose `strategy_refs` cite a
/// document that was not retrieved for this turn, which is only checkable
/// because the citations live in their own list.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-export", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct RecommendationEvidence {
    /// Facts read from the observation. Each should be traceable to a field in
    /// the envelope.
    pub observed_facts: Vec<String>,
    /// Identifiers of strategy documents retrieved for this turn. Verified
    /// against what retrieval actually returned.
    pub strategy_refs: Vec<String>,
    /// Claims the model produced that are neither observed nor cited.
    pub model_inferences: Vec<String>,
    /// Context the user supplied, for example an answer to a spoken question.
    pub user_provided: Vec<String>,
}

/// When a recommendation stops being true.
///
/// `invalidated_by` names change kinds; they are matched against the
/// `change_summary` of later observation envelopes. An empty
/// `invalidated_by` is a claim that nothing observable can falsify this
/// advice, which should be rare and deliberate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-export", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct Validity {
    /// Wall-clock expiry, when the advice is time-bounded rather than
    /// state-bounded. `None` means it dies only on state change.
    pub expires_at: Option<String>,
    /// Change kinds that kill this recommendation.
    pub invalidated_by: Vec<String>,
}

/// A single piece of coaching advice bound to one observation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-export", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct Recommendation {
    /// Contract schema version. See [`crate::ids::CURRENT_SCHEMA_VERSION`].
    pub schema_version: SchemaVersion,
    /// Which game this advice is for.
    pub game_id: GameId,
    /// Coaching session this advice belongs to.
    pub session_id: SessionId,
    /// The observation this advice describes. The binding that makes staleness
    /// detectable rather than a matter of opinion.
    pub observation_id: ObservationId,
    /// One-line advice, the thing a player reads mid-turn.
    pub summary: String,
    /// Ranked concrete actions.
    pub actions: Vec<RecommendedAction>,
    /// Why, in prose, for a player who wants to understand rather than obey.
    pub rationale: String,
    /// How confident the coach is in this advice as a whole.
    pub confidence: Confidence,
    /// What this advice rests on, split by kind of claim.
    pub evidence: RecommendationEvidence,
    /// Caveats the player should see alongside the advice, for example that a
    /// field could not be read.
    pub warnings: Vec<String>,
    /// Conditions under which this advice dies.
    pub validity: Validity,
}
