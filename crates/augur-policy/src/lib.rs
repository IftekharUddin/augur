//! Game-policy gating.
//!
//! Every game carries a recorded policy-review outcome. A game whose review is
//! still `Pending`, or which came back `NoGo`, cannot be offered to a player,
//! and that is enforced here rather than left to whoever wires up the registry.
//! The standing product rules, no gameplay automation, no game-memory reading,
//! no anti-cheat circumvention, apply regardless of any review outcome; a
//! favourable review does not unlock them.
//!
//! The review process and its checklist are in
//! `docs/policy/game-policy-review.md`.

use augur_core::GameId;
use augur_game_api::SupportStatus;
use serde::{Deserialize, Serialize};

/// Recorded outcome of a game's policy review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PolicyReviewStatus {
    /// Not yet reviewed. Blocks release.
    Pending,
    /// Reviewed, no restrictions found.
    Go,
    /// Reviewed, permitted subject to recorded conditions.
    Conditional,
    /// Reviewed, must not ship.
    NoGo,
}

/// Why a game may not be offered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyRefusal {
    /// The policy review has not been performed.
    ReviewPending(GameId),
    /// The review concluded the game must not ship.
    ReviewNoGo(GameId),
    /// The game is held back pending review.
    SupportStatusBlocked(GameId),
}

/// Decide whether a game may be offered to a player.
///
/// Both gates are checked, and both fail closed. A `Conditional` outcome
/// passes here because the conditions are recorded in the game's manifest and
/// enforced where they apply; this function answers only "may it ship at all".
pub fn may_offer(
    game: &GameId,
    support: SupportStatus,
    review: PolicyReviewStatus,
) -> Result<(), PolicyRefusal> {
    if !support.is_shippable() {
        return Err(PolicyRefusal::SupportStatusBlocked(game.clone()));
    }
    match review {
        PolicyReviewStatus::Pending => Err(PolicyRefusal::ReviewPending(game.clone())),
        PolicyReviewStatus::NoGo => Err(PolicyRefusal::ReviewNoGo(game.clone())),
        PolicyReviewStatus::Go | PolicyReviewStatus::Conditional => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn game() -> GameId {
        GameId::new("test-game")
    }

    #[test]
    fn pending_review_blocks_even_a_maintained_game() {
        // The failure mode this guards against: a game gets polished to
        // `Maintained` and ships while its review was never done.
        let result = may_offer(
            &game(),
            SupportStatus::Maintained,
            PolicyReviewStatus::Pending,
        );
        assert_eq!(result, Err(PolicyRefusal::ReviewPending(game())));
    }

    #[test]
    fn no_go_blocks() {
        let result = may_offer(&game(), SupportStatus::Maintained, PolicyReviewStatus::NoGo);
        assert_eq!(result, Err(PolicyRefusal::ReviewNoGo(game())));
    }

    #[test]
    fn disabled_support_status_blocks_even_with_a_go_review() {
        let result = may_offer(
            &game(),
            SupportStatus::DisabledPolicyReview,
            PolicyReviewStatus::Go,
        );
        assert_eq!(result, Err(PolicyRefusal::SupportStatusBlocked(game())));
    }

    #[test]
    fn conditional_review_may_ship() {
        assert!(
            may_offer(
                &game(),
                SupportStatus::Community,
                PolicyReviewStatus::Conditional
            )
            .is_ok()
        );
    }
}
