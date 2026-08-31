//! The `GameAdapter` seam.
//!
//! This crate is the entire surface a new game implements. If supporting a
//! second game requires changing anything outside `games/<id>/` and one
//! registry entry, this seam has failed, and Milestone 4's second-game proof
//! exists to find that out before the API is frozen.
//!
//! Loading model: adapters are compile-time Rust crates in a static registry,
//! not dynamically loaded plugins. The reasoning, and the rejected
//! alternatives, are in `docs/decisions/0002-game-adapter-loading.md`.
//!
//! # Stability
//!
//! Unstable until the second-game proof compiles against it without platform
//! changes. After that, changes follow a decision record and a versioned
//! `GameManifest::adapter_version`.
//!
//! # Status
//!
//! Milestone 0 fixes the shape: the trait, its associated types, and the
//! responsibilities each method owns. Bodies arrive with the game work.

use augur_core::{Confidence, GameId, GameStateEnvelope, Recommendation};
use serde::{Deserialize, Serialize};

/// How far along a game's support has come.
///
/// Status is data in `game.yaml`, surfaced in the interface, and enforced in
/// CI: a `DisabledPolicyReview` game cannot ship enabled, whatever the code
/// says.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SupportStatus {
    /// Works for the author; no promises.
    Experimental,
    /// Community-maintained.
    Community,
    /// Actively maintained with fixtures and evaluation coverage.
    Maintained,
    /// No longer maintained; still loadable.
    Deprecated,
    /// Held back pending a game-policy review outcome. Cannot ship enabled.
    DisabledPolicyReview,
}

impl SupportStatus {
    /// Whether a game in this status may be offered to a user.
    ///
    /// The policy-review gate is the reason this is a method and not a
    /// comparison at each call site: one place to be right.
    pub fn is_shippable(self) -> bool {
        match self {
            SupportStatus::Experimental
            | SupportStatus::Community
            | SupportStatus::Maintained
            | SupportStatus::Deprecated => true,
            SupportStatus::DisabledPolicyReview => false,
        }
    }
}

/// Static description of a supported game, loaded from `games/<id>/game.yaml`.
///
/// Field-level contracts and the JSON Schema that validates the file arrive
/// with the manifest-validation work; this is the identity the registry needs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameManifest {
    /// Stable identifier, matching the directory under `games/` and the
    /// `game_id` key of that game's `game.yaml`.
    pub game_id: GameId,
    /// Name shown to a player.
    pub display_name: String,
    /// Version of the adapter implementing this game.
    pub adapter_version: String,
    /// How far along support is.
    pub support_status: SupportStatus,
}

/// What the platform knows when asking an adapter whether its game is running.
///
/// Deliberately not "the whole system": an adapter gets enumerated windows and
/// process names, never the ability to go looking for itself.
#[derive(Debug, Clone, Default)]
pub struct DetectionContext {
    /// Executable names of running processes, as the platform observed them.
    pub process_names: Vec<String>,
    /// Titles of enumerated top-level windows.
    pub window_titles: Vec<String>,
}

/// An adapter's answer to "is your game here?".
#[derive(Debug, Clone, PartialEq)]
pub enum DetectionResult {
    /// Not present.
    NotFound,
    /// Present, with the adapter's confidence in the match.
    Found {
        /// Which enumerated window the adapter believes is the game.
        window_title: String,
        /// How sure the adapter is.
        confidence: Confidence,
    },
}

/// Capture hints an adapter supplies for its game.
///
/// Hints, not commands: the capture layer owns the platform APIs and may refuse
/// a cadence the system cannot sustain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaptureProfile {
    /// Fastest cadence worth capturing at, in milliseconds.
    pub min_interval_ms: u32,
    /// Slowest cadence before the coach is too far behind to be useful.
    pub max_interval_ms: u32,
}

/// The retrieval scope derived from observed state.
///
/// Retrieval is deterministic filtering before it is anything else, so this is
/// a set of exact filters rather than a free-text query. See
/// `docs/decisions/0003-strategy-retrieval.md`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StrategyScope {
    /// Season or pack identifier to search within.
    pub season: Option<String>,
    /// Game phase, used to exclude documents that do not apply.
    pub phase: Option<String>,
    /// Recognized entities (heroes, tribes, minions, mechanics) to match
    /// against document `applies_to` front matter.
    pub entities: Vec<String>,
}

/// The bounded, delimited context assembled for one coaching turn.
///
/// `strategy_refs` is not decoration: the recommendation validator rejects
/// advice citing a document that is not in this list, which is what stops a
/// model from inventing a citation.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CoachingContext {
    /// The prompt body, with untrusted content already delimited.
    pub prompt: String,
    /// Identifiers of the strategy documents supplied to this turn.
    pub strategy_refs: Vec<String>,
}

/// The outcome of an adapter's game-specific checks on a recommendation.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ValidationReport {
    /// Findings that make the advice unsafe to show.
    pub rejections: Vec<String>,
    /// Findings worth surfacing alongside the advice.
    pub warnings: Vec<String>,
}

impl ValidationReport {
    /// Whether the recommendation may be published.
    pub fn is_acceptable(&self) -> bool {
        self.rejections.is_empty()
    }
}

/// Why an observation could not be produced.
#[derive(Debug, thiserror::Error)]
pub enum ObservationError {
    /// The frame did not show a state this adapter recognizes.
    #[error("frame did not match a known game state: {0}")]
    Unrecognized(String),
    /// Extraction ran but the result was too uncertain to publish.
    #[error("extraction confidence too low to publish")]
    ConfidenceTooLow,
}

/// Everything a game must implement, and nothing more.
///
/// Note what is *absent*: no method returns pixels, no method sends input, and
/// no method reaches the model provider directly. Vision-heavy extraction is
/// delegated to the coaching pipeline; the adapter owns the schema and the
/// prompt, not the network call.
pub trait GameAdapter: Send + Sync {
    /// Static description of this game.
    fn manifest(&self) -> &GameManifest;

    /// Is this game running, and which window is it?
    fn detect(&self, ctx: &DetectionContext) -> DetectionResult;

    /// Cadence and framing hints for capturing this game.
    fn capture_profile(&self) -> CaptureProfile;

    /// Turn extracted state into a validated envelope.
    ///
    /// `previous` is supplied so the adapter can compute `change_summary` and
    /// carry forward fields the current frame does not show. Fields that cannot
    /// be read stay missing; inventing a plausible value is the failure mode
    /// this whole contract exists to prevent.
    fn parse_observation(
        &self,
        previous: Option<&GameStateEnvelope>,
    ) -> Result<GameStateEnvelope, ObservationError>;

    /// Map observed state to the retrieval scope for this turn.
    fn strategy_scope(&self, state: &GameStateEnvelope) -> StrategyScope;

    /// Assemble the bounded coaching context.
    fn coaching_context(&self, state: &GameStateEnvelope) -> CoachingContext;

    /// Game-specific sanity checks, for example refusing a purchase the
    /// observed gold cannot cover *when gold was actually observed*.
    fn validate_recommendation(
        &self,
        state: &GameStateEnvelope,
        recommendation: &Recommendation,
    ) -> ValidationReport;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn policy_review_blocks_shipping() {
        assert!(!SupportStatus::DisabledPolicyReview.is_shippable());
        assert!(SupportStatus::Experimental.is_shippable());
        assert!(SupportStatus::Deprecated.is_shippable());
    }

    #[test]
    fn a_report_with_rejections_is_not_acceptable() {
        let mut report = ValidationReport::default();
        assert!(report.is_acceptable());
        report.warnings.push("low confidence on tier".into());
        assert!(
            report.is_acceptable(),
            "warnings must not block publication"
        );
        report.rejections.push("cannot afford".into());
        assert!(!report.is_acceptable());
    }
}
