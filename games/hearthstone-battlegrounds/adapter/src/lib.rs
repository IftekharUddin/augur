//! Hearthstone Battlegrounds adapter.
//!
//! This crate may name Hearthstone freely. Platform crates may not, and
//! `tests/architecture/augur_game_isolation.rs` enforces the difference. Its
//! data, the manifest, state schema, prompts, strategy packs, and fixtures,
//! lives beside it under `games/hearthstone-battlegrounds/`.
//!
//! # Status
//!
//! Milestone 0 proves the seam compiles end to end: this adapter implements
//! [`GameAdapter`] and is reachable through the runtime registry. Every method
//! reports honestly that its subsystem does not exist yet rather than returning
//! a plausible-looking placeholder, because a fake detection or a fabricated
//! observation would be indistinguishable from a broken one during Milestone 1
//! development.
//!
//! # Policy
//!
//! `games/hearthstone-battlegrounds/game.yaml` records
//! `policy_review.status: pending`, which blocks release until the review is
//! recorded. `augur-policy` enforces that; this crate does not decide it.

use augur_core::{GameStateEnvelope, Recommendation};
use augur_game_api::{
    CaptureProfile, CoachingContext, DetectionContext, DetectionResult, GameAdapter, GameManifest,
    ObservationError, StrategyScope, SupportStatus, ValidationReport,
};

/// Stable identifier for this game, matching the `games/` directory name.
pub const GAME_ID: &str = "hearthstone-battlegrounds";

/// The Hearthstone Battlegrounds adapter.
#[derive(Debug)]
pub struct BattlegroundsAdapter {
    manifest: GameManifest,
}

impl BattlegroundsAdapter {
    /// Construct the adapter.
    pub fn new() -> Self {
        Self {
            manifest: GameManifest {
                game_id: augur_core::GameId::new(GAME_ID),
                display_name: "Hearthstone Battlegrounds".to_string(),
                adapter_version: "0.1.0".to_string(),
                // Mirrors `status: experimental` in game.yaml. Shipping is
                // separately blocked by `policy_review.status: pending`, which
                // `augur-policy` enforces; support status and policy review are
                // two gates on purpose, and collapsing them here would hide
                // which one is actually holding the game back.
                support_status: SupportStatus::Experimental,
            },
        }
    }
}

impl Default for BattlegroundsAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl GameAdapter for BattlegroundsAdapter {
    fn manifest(&self) -> &GameManifest {
        &self.manifest
    }

    fn detect(&self, _ctx: &DetectionContext) -> DetectionResult {
        // Detection rules live in `game.yaml` and are read by the detection
        // work. Reporting NotFound is the honest answer while that is unbuilt:
        // a hardcoded match would make a broken enumeration look like success.
        DetectionResult::NotFound
    }

    fn capture_profile(&self) -> CaptureProfile {
        // A Battlegrounds shop turn runs roughly 60 to 90 seconds, so advice is
        // useful at a cadence measured in seconds, not frames. These bounds are
        // the starting point the cadence work measures against, not a tuned
        // result.
        CaptureProfile {
            min_interval_ms: 500,
            max_interval_ms: 5_000,
        }
    }

    fn parse_observation(
        &self,
        _previous: Option<&GameStateEnvelope>,
    ) -> Result<GameStateEnvelope, ObservationError> {
        Err(ObservationError::Unrecognized(
            "Battlegrounds state extraction is not implemented yet".to_string(),
        ))
    }

    fn strategy_scope(&self, _state: &GameStateEnvelope) -> StrategyScope {
        StrategyScope::default()
    }

    fn coaching_context(&self, _state: &GameStateEnvelope) -> CoachingContext {
        CoachingContext::default()
    }

    fn validate_recommendation(
        &self,
        _state: &GameStateEnvelope,
        _recommendation: &Recommendation,
    ) -> ValidationReport {
        ValidationReport::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The manifest file beside this crate, which is the canonical description
    /// of the game. Loading it properly is the manifest-validation work; until
    /// then, this reads it to catch the hardcoded values drifting from it.
    fn game_yaml() -> String {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("game.yaml");
        std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("reading {}: {error}", path.display()))
    }

    fn scalar(text: &str, key: &str) -> String {
        text.lines()
            .find_map(|line| line.strip_prefix(&format!("{key}:")))
            .map(|value| {
                value
                    .split('#')
                    .next()
                    .unwrap_or_default()
                    .trim()
                    .trim_matches(['"', '\''].as_slice())
                    .to_string()
            })
            .unwrap_or_else(|| panic!("game.yaml has no top-level `{key}` key"))
    }

    #[test]
    fn the_hardcoded_manifest_agrees_with_game_yaml() {
        // Two sources of truth for the same facts is exactly the drift this
        // catches: a contributor edits game.yaml, the adapter keeps reporting
        // the old identity, and the registry silently disagrees with the data
        // directory beside it.
        let yaml = game_yaml();
        let adapter = BattlegroundsAdapter::new();
        let manifest = adapter.manifest();

        assert_eq!(manifest.game_id.as_str(), scalar(&yaml, "game_id"));
        assert_eq!(manifest.game_id.as_str(), GAME_ID);
        assert_eq!(manifest.display_name, scalar(&yaml, "display_name"));
        assert_eq!(
            format!("{:?}", manifest.support_status).to_lowercase(),
            scalar(&yaml, "status"),
            "support status must mirror game.yaml's `status`"
        );
    }

    #[test]
    fn the_policy_review_is_still_pending() {
        // Not a permanent assertion. When the review is recorded, this test
        // fails and whoever records it must decide, deliberately, what the
        // adapter should now report. That is the intended behavior: a policy
        // outcome should not land silently.
        let yaml = game_yaml();
        let pending = yaml
            .lines()
            .skip_while(|line| !line.starts_with("policy_review:"))
            .any(|line| line.trim_start().starts_with("status:") && line.contains("pending"));
        assert!(
            pending,
            "policy_review.status is no longer `pending`; update the adapter and this test"
        );
    }

    #[test]
    fn extraction_reports_absence_rather_than_inventing_state() {
        let adapter = BattlegroundsAdapter::new();
        assert!(adapter.parse_observation(None).is_err());
    }
}
