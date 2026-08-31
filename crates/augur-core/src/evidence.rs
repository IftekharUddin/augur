//! Confidence and evidence: how Augur says what it does not know.
//!
//! A coach that states a guess as a fact is worse than one that says nothing.
//! The product rule is that missing data stays missing and uncertainty is
//! stated, so the vocabulary for expressing both lives here rather than being
//! re-invented per game.

use serde::{Deserialize, Serialize};

/// A confidence in `0.0..=1.0`.
///
/// A newtype rather than a bare `f32` so an out-of-range value cannot be
/// constructed at all. `NaN` is rejected: a comparison against `NaN` silently
/// answers `false`, which would make every confidence threshold in the
/// pipeline quietly fail open.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(try_from = "f32", into = "f32")]
pub struct Confidence(f32);

/// Why a [`Confidence`] could not be constructed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfidenceError {
    /// The value was below 0.0 or above 1.0.
    OutOfRange,
    /// The value was `NaN`, which cannot be meaningfully compared.
    NotANumber,
}

impl std::fmt::Display for ConfidenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfidenceError::OutOfRange => f.write_str("confidence must be within 0.0..=1.0"),
            ConfidenceError::NotANumber => f.write_str("confidence must not be NaN"),
        }
    }
}

impl std::error::Error for ConfidenceError {}

impl Confidence {
    /// Complete absence of confidence.
    pub const NONE: Self = Self(0.0);
    /// Full confidence. Reserved for deterministically read values.
    pub const CERTAIN: Self = Self(1.0);

    /// Construct a confidence, rejecting out-of-range and `NaN` values.
    pub fn new(value: f32) -> Result<Self, ConfidenceError> {
        if value.is_nan() {
            return Err(ConfidenceError::NotANumber);
        }
        if !(0.0..=1.0).contains(&value) {
            return Err(ConfidenceError::OutOfRange);
        }
        Ok(Self(value))
    }

    /// The underlying value.
    pub fn get(self) -> f32 {
        self.0
    }
}

impl TryFrom<f32> for Confidence {
    type Error = ConfidenceError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<Confidence> for f32 {
    fn from(value: Confidence) -> Self {
        value.0
    }
}

/// Per-field extraction evidence.
///
/// The envelope's top-level confidence is an overall figure; this is where a
/// consumer learns that `gold` was read with near-certainty while `tier` was a
/// guess. The UI renders the difference, and the recommendation validator uses
/// it to refuse advice that depends on a field nobody actually observed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldEvidence {
    /// Dotted path of the field within the game-owned `state` payload.
    pub field: String,
    /// Confidence for this field specifically.
    pub confidence: Confidence,
}

/// Privacy classification of the data a record refers to.
///
/// Carried on the envelope so that anything downstream, logs, diagnostics
/// bundles, feedback uploads, can make a retention decision without needing to
/// know how the data was produced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Privacy {
    /// Derived from a frame that exists only for the duration of a turn and is
    /// never persisted. The default for screen capture.
    LocalFrameTransient,
    /// Text derived from a frame, retained locally as part of match state.
    LocalDerivedText,
    /// The user explicitly opted in to retaining this record.
    UserRetained,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn confidence_rejects_out_of_range() {
        assert_eq!(Confidence::new(1.5), Err(ConfidenceError::OutOfRange));
        assert_eq!(Confidence::new(-0.1), Err(ConfidenceError::OutOfRange));
    }

    #[test]
    fn confidence_rejects_nan() {
        // Not pedantry: `NaN < threshold` is false, so a NaN confidence would
        // pass every "is this too uncertain to act on" check in the pipeline.
        assert_eq!(Confidence::new(f32::NAN), Err(ConfidenceError::NotANumber));
    }

    #[test]
    fn confidence_accepts_the_closed_interval() {
        assert_eq!(Confidence::new(0.0).map(Confidence::get), Ok(0.0));
        assert_eq!(Confidence::new(1.0).map(Confidence::get), Ok(1.0));
    }

    #[test]
    fn confidence_rejects_out_of_range_on_the_wire_too() {
        // The validation must survive deserialization, or an envelope from a
        // buggy adapter reintroduces exactly what the newtype prevents.
        let parsed = serde_json::from_str::<Confidence>("1.5");
        assert!(parsed.is_err(), "deserialization must enforce the range");
    }
}
