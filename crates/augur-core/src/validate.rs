//! Contract validation.
//!
//! Rust types are canonical and `serde` does most of the work: unknown fields
//! are rejected by `deny_unknown_fields`, absent fields are a parse error
//! unless the type says `Option`, and a confidence outside `0.0..=1.0` cannot
//! be constructed. What is left over is the set of rules a type system cannot
//! state, and they live here.
//!
//! The design rule behind all of them: **a contract violation is an error, not
//! a warning.** A coaching pipeline that accepts a malformed envelope and
//! carries on produces advice about a state nobody can reconstruct afterwards.

use crate::ids::{CURRENT_SCHEMA_VERSION, SchemaVersion};
use crate::observation::GameStateEnvelope;
use crate::recommendation::Recommendation;

/// Why a contract instance is not acceptable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContractError {
    /// The payload declares a schema version this build does not understand.
    ///
    /// Deliberately not "ignore and hope": a future envelope may give an
    /// existing field a new meaning, and silently reading it with old rules is
    /// how a coach ends up describing a state that never existed.
    UnsupportedSchemaVersion {
        /// What the payload declared.
        found: SchemaVersion,
        /// What this build understands.
        supported: SchemaVersion,
    },
    /// A field that must not be empty was empty.
    Empty {
        /// Dotted path of the offending field.
        field: &'static str,
    },
    /// Action priorities must be unique and start at 1.
    MalformedPriorities {
        /// What was found, in order.
        found: Vec<u8>,
    },
}

impl std::fmt::Display for ContractError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContractError::UnsupportedSchemaVersion { found, supported } => write!(
                f,
                "schema_version {found} is not supported by this build (supports {supported})"
            ),
            ContractError::Empty { field } => write!(f, "{field} must not be empty"),
            ContractError::MalformedPriorities { found } => write!(
                f,
                "action priorities must be unique and start at 1; found {found:?}"
            ),
        }
    }
}

impl std::error::Error for ContractError {}

fn check_schema_version(found: SchemaVersion) -> Result<(), ContractError> {
    if found == CURRENT_SCHEMA_VERSION {
        Ok(())
    } else {
        Err(ContractError::UnsupportedSchemaVersion {
            found,
            supported: CURRENT_SCHEMA_VERSION,
        })
    }
}

impl GameStateEnvelope {
    /// Check the rules `serde` cannot express.
    pub fn validate(&self) -> Result<(), ContractError> {
        check_schema_version(self.schema_version)?;
        if self.game_id.as_str().is_empty() {
            return Err(ContractError::Empty { field: "game_id" });
        }
        if self.source.frame_hash.is_empty() {
            return Err(ContractError::Empty {
                field: "source.frame_hash",
            });
        }
        Ok(())
    }
}

impl Recommendation {
    /// Check the rules `serde` cannot express.
    ///
    /// Citation verification is **not** here: it needs the set of documents
    /// retrieval actually returned this turn, which this crate has no way to
    /// know. That check lives in `augur-recommendation`, and keeping the two
    /// apart is what stops a validator from being satisfied with checking only
    /// the half it can see.
    pub fn validate(&self) -> Result<(), ContractError> {
        check_schema_version(self.schema_version)?;
        if self.summary.trim().is_empty() {
            return Err(ContractError::Empty { field: "summary" });
        }
        let priorities: Vec<u8> = self.actions.iter().map(|action| action.priority).collect();
        if !priorities.is_empty() {
            let mut sorted = priorities.clone();
            sorted.sort_unstable();
            sorted.dedup();
            let expected: Vec<u8> = (1..=priorities.len() as u8).collect();
            if sorted != expected {
                return Err(ContractError::MalformedPriorities { found: priorities });
            }
        }
        Ok(())
    }
}

/// Every schema Augur generates, as `(filename, pretty JSON)` pairs.
///
/// One list, used by both the export binary and the drift test, so the two can
/// never disagree about what should exist.
#[cfg(feature = "schema-export")]
pub fn generated_schemas() -> Vec<(&'static str, String)> {
    fn render<T: schemars::JsonSchema>() -> String {
        let schema = schemars::schema_for!(T);
        let mut json = serde_json::to_string_pretty(&schema)
            .expect("a generated schema is always serializable");
        json.push('\n');
        json
    }

    vec![
        (
            "observation-envelope.schema.json",
            render::<GameStateEnvelope>(),
        ),
        ("recommendation.schema.json", render::<Recommendation>()),
    ]
}
