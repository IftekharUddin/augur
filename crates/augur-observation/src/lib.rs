//! Match session state and the observation lifecycle.
//!
//! Explicit match state is the source of truth for what has happened in a
//! match, not open-ended agent memory. That is a product rule, not an
//! implementation convenience: a coach that "remembers" a match through a
//! summarization pipeline will eventually contradict the board in front of the
//! player, and there is no way to test that it has not.
//! `zeroclaw-memory` is therefore deliberately not used here.
//!
//! # Status
//!
//! Milestone 0 defines the store seam and the identity a match is keyed by. The
//! SQLite-backed implementation, sitting beside the upstream session store in
//! its own table namespace, arrives with the live-pipeline work.

use augur_core::{GameId, GameStateEnvelope, MatchId, SessionId};

/// What makes a match distinct from the one before it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchIdentity {
    /// Which game.
    pub game_id: GameId,
    /// The coaching session this match belongs to.
    pub session_id: SessionId,
    /// This match's identity.
    pub match_id: MatchId,
}

/// Why a match-state operation failed.
#[derive(Debug, thiserror::Error)]
pub enum MatchStateError {
    /// No match has been started for this session.
    #[error("no active match for session {0}")]
    NoActiveMatch(SessionId),
    /// The backing store failed.
    #[error("match state store failed: {0}")]
    Store(String),
}

/// Durable per-match observation history.
///
/// Survives daemon restarts so that "what happened earlier this match" is
/// answerable after a crash, which is when a player is least willing to lose
/// context.
pub trait MatchSessionStore: Send + Sync {
    /// Begin a new match, ending any active one for the same session.
    fn begin_match(&self, identity: &MatchIdentity) -> Result<(), MatchStateError>;

    /// Append an observation to the active match.
    fn append(&self, envelope: &GameStateEnvelope) -> Result<(), MatchStateError>;

    /// The most recent observation for a session, if any.
    fn latest(&self, session: &SessionId) -> Result<Option<GameStateEnvelope>, MatchStateError>;

    /// End the active match for a session.
    fn end_match(&self, session: &SessionId) -> Result<(), MatchStateError>;
}
