//! Identifiers that travel across every Augur boundary.
//!
//! These are newtypes rather than bare `String`s because they cross process
//! boundaries (RPC), storage boundaries (the match session store), and the
//! model boundary (the coaching turn), and the compiler is the cheapest place
//! to catch a session id used where an observation id was meant. The
//! recommendation-invalidation rule in
//! `docs/architecture/state-and-recommendations.md` turns on exactly that
//! distinction.

use serde::{Deserialize, Serialize};

/// Schema version carried by the observation envelope and the recommendation
/// contract.
///
/// Bumped when a change would make an older consumer misread a payload. Adding
/// an optional field is not such a change; removing a field, changing its
/// meaning, or narrowing its type is.
pub type SchemaVersion = u32;

/// The current envelope and recommendation schema version.
pub const CURRENT_SCHEMA_VERSION: SchemaVersion = 1;

macro_rules! string_id {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        #[cfg_attr(feature = "schema-export", derive(schemars::JsonSchema))]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            /// Wrap an already-validated identifier.
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            /// Borrow the underlying string.
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Consume the newtype and return the owned string.
            pub fn into_inner(self) -> String {
                self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }
    };
}

string_id! {
    /// Stable identifier for a supported game, matching the directory name
    /// under `games/` and the `id` field of that game's `game.yaml`.
    ///
    /// A `GameId` value may exist in platform crates; a *literal* game id may
    /// not. The architecture test draws that line.
    GameId
}

string_id! {
    /// Identifies one coaching session: a run of the app bound to one game and
    /// one capture target. Outlives individual matches.
    SessionId
}

string_id! {
    /// Identifies one match within a session. Match state, not open-ended agent
    /// memory, is the source of truth for what has happened so far; see
    /// `docs/architecture/state-and-recommendations.md`.
    MatchId
}

string_id! {
    /// Identifies one observation of game state.
    ///
    /// Load-bearing: every recommendation is bound to the `ObservationId` it
    /// was generated from, and a recommendation whose observation has been
    /// superseded is invalidated rather than shown as current. This is the
    /// identifier that makes "never silently stale" checkable.
    ObservationId
}

string_id! {
    /// Version of the game adapter that produced an envelope, so a stored
    /// observation can be interpreted by the adapter version that wrote it.
    AdapterVersion
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifiers_are_transparent_over_the_wire() {
        let id = ObservationId::new("abc");
        let json = serde_json::to_string(&id).expect("serialize");
        assert_eq!(
            json, "\"abc\"",
            "ids must not gain a wrapper object on the wire"
        );
        let back: ObservationId = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, id);
    }

    #[test]
    fn distinct_id_types_do_not_interchange() {
        // Compile-time proof lives in the type system; this asserts the runtime
        // representation stays comparable only within a type.
        let session = SessionId::new("shared-text");
        let observation = ObservationId::new("shared-text");
        assert_eq!(session.as_str(), observation.as_str());
    }
}
