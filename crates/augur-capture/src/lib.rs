//! Window-scoped, read-only frame capture.
//!
//! Two constraints define this crate, and both are enforced elsewhere rather
//! than merely documented here:
//!
//! 1. **Window-scoped only.** Display capture is not a supported mode. A player
//!    coaching a game should not have their notifications, messages, or other
//!    windows leave the machine because they happened to overlap.
//! 2. **Read-only.** Nothing here, or anywhere in Augur, synthesizes input.
//!    `tests/architecture/augur_no_input_synthesis.rs` greps for the platform
//!    symbols that would make it possible and fails the build if one appears.
//!
//! Upstream ZeroClaw's screenshot tool is not reused: it shells out to
//! `screencapture`/`scrot`, has no Windows support, and cannot enumerate or
//! scope to a window. See `docs/architecture/zeroclaw-reuse-audit.md`.
//!
//! # Status
//!
//! Milestone 0 defines the provider seam. The macOS (ScreenCaptureKit) and
//! Windows (Windows.Graphics.Capture) providers are later milestones.

use augur_core::Privacy;

/// A capturable window, as enumerated by a platform provider.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameWindow {
    /// Opaque, platform-specific handle. Never a filesystem path.
    pub id: String,
    /// Window title, used for adapter detection and shown when the user picks a
    /// capture target.
    pub title: String,
    /// Owning executable name.
    pub process_name: String,
}

/// One captured frame, normalized and hashed.
///
/// The pixels are deliberately not a field here. A frame reaches the model as a
/// content-addressed temp file referenced by path, deleted after the turn; a
/// struct that owned pixel data would make the "never persisted by default"
/// promise depend on everyone downstream remembering to drop it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapturedFrame {
    /// Window the frame came from.
    pub window_id: String,
    /// Content hash of the normalized frame. Change detection compares these.
    pub frame_hash: String,
    /// Path to the transient frame file.
    pub path: std::path::PathBuf,
    /// Retention classification. Transient by default.
    pub privacy: Privacy,
}

/// Why capture failed.
///
/// `PermissionDenied` is separated from the rest because it is the one failure
/// with a specific, actionable remedy: the interface deep-links to the relevant
/// operating-system settings pane rather than showing a generic error.
#[derive(Debug, thiserror::Error)]
pub enum CaptureError {
    /// The operating system has not granted screen-recording permission.
    #[error("screen capture permission has not been granted")]
    PermissionDenied,
    /// The selected window no longer exists.
    #[error("window {0} is no longer available")]
    WindowGone(String),
    /// The platform provider failed for another reason.
    #[error("capture failed: {0}")]
    Backend(String),
}

/// A platform capture backend.
///
/// Implementations are per-operating-system. A `ReplayCaptureProvider` feeding
/// recorded frames through the real pipeline is what lets the whole system be
/// tested without a running game; see
/// `docs/architecture/testing-and-evaluation.md`.
pub trait CaptureProvider: Send + Sync {
    /// Enumerate windows that can be captured.
    fn enumerate(&self) -> Result<Vec<GameWindow>, CaptureError>;

    /// Capture one frame from a previously selected window.
    fn capture(&self, window_id: &str) -> Result<CapturedFrame, CaptureError>;
}
