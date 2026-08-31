//! Spoken coaching.
//!
//! Server-side speech is reused from upstream, which ships five text-to-speech
//! and six speech-to-text providers, all request/response rather than
//! streaming. Client-side capture and playback are new work in the desktop app.
//! Push-to-talk is the default and wake-word listening is deliberately not
//! built: a coach that listens continuously is a different product with a
//! different privacy story.
//!
//! Audio is never retained. Transcripts are retained only on explicit opt-in.
//!
//! # Status
//!
//! Milestone 0 reserves the crate and records the constraints above so the
//! voice work starts from them rather than rediscovering them. The state
//! machine, provider orchestration, and barge-in handling are Milestone 3.

/// What the voice subsystem is doing right now.
///
/// Interruption is a state transition rather than a special case: a player who
/// starts talking over the coach expects it to stop immediately, and modelling
/// that as an afterthought is how half-spoken advice ends up racing the next
/// recommendation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoiceState {
    /// Nothing is being captured or spoken.
    Idle,
    /// Speaking a recommendation.
    Speaking,
    /// Capturing push-to-talk input.
    Listening,
    /// Output is muted by the player; recommendations still arrive silently.
    Muted,
}
