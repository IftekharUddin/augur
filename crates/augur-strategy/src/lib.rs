//! Strategy packs and retrieval.
//!
//! Strategy packs are **data, never code**. They are community-authored
//! Markdown with validated YAML front matter, and they are treated as untrusted
//! input all the way through: delimited when they reach a prompt, never
//! interpreted as instructions, and covered by prompt-injection regression
//! fixtures.
//!
//! Retrieval is deterministic first and clever later. The Milestone 1 pipeline
//! is exact metadata filters, then entity filters, then lexical ranking over
//! what survives, then a hard document and token cap, then citation
//! verification against what was actually retrieved. Embeddings are an additive
//! later stage, and only if evaluation shows lexical retrieval missing relevant
//! documents. The reasoning is in
//! `docs/decisions/0003-strategy-retrieval.md`; the short version is that
//! deterministic retrieval is testable offline, which keeps the degraded-mode
//! story honest when the network is gone.
//!
//! Upstream's `rag` module is a precedent, not a dependency: it is a
//! keyword-keyed chunker locked to hardware datasheets, and `knowledge_bundles`
//! config is dead upstream. See `docs/architecture/zeroclaw-reuse-audit.md`.
//!
//! # Status
//!
//! Milestone 0 defines the retrieval seam and the document identity that
//! citation verification turns on.

use augur_core::GameId;
use serde::{Deserialize, Serialize};

/// Editorial state of a strategy document.
///
/// Retrieval excludes `Deprecated` outright and excludes `Draft` unless the
/// user opted in, so a half-finished document cannot quietly become the basis
/// for advice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DocumentStatus {
    /// Work in progress. Excluded unless the user opts in.
    Draft,
    /// Reviewed and current.
    Stable,
    /// Superseded. Never retrieved.
    Deprecated,
}

/// Stable identifier of a strategy document, as cited by a recommendation.
///
/// Citations are checked against the identifiers retrieval actually returned,
/// so this type is the join key of that check.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct StrategyDocumentId(String);

impl StrategyDocumentId {
    /// Wrap an identifier.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Borrow the underlying string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for StrategyDocumentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// One retrievable strategy document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrategyDocument {
    /// Identifier used in citations.
    pub id: StrategyDocumentId,
    /// Which game this document is about.
    pub game_id: GameId,
    /// Season or pack this document belongs to.
    pub season: String,
    /// Editorial state.
    pub status: DocumentStatus,
    /// Document body. Untrusted text: delimited before it reaches a prompt.
    pub body: String,
}

/// Hard limits on what one coaching turn may retrieve.
///
/// Caps are part of the contract rather than a tuning knob buried in a config
/// file, because an unbounded retrieval is how a coaching turn silently becomes
/// expensive and slow.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetrievalCaps {
    /// Most documents that may be returned.
    pub max_documents: usize,
    /// Most tokens those documents may contribute.
    pub max_tokens: usize,
}

/// Why retrieval failed.
#[derive(Debug, thiserror::Error)]
pub enum RetrievalError {
    /// No pack is active for the requested game.
    #[error("no active strategy pack for game {0}")]
    NoActivePack(GameId),
    /// The pack index could not be read.
    #[error("strategy index unavailable: {0}")]
    IndexUnavailable(String),
}

/// Deterministic retrieval over a validated strategy corpus.
pub trait StrategyRetriever: Send + Sync {
    /// Retrieve documents for a scope, within caps.
    ///
    /// Implementations must fall back to the season's fundamentals when state
    /// recognition is incomplete: incomplete extraction should degrade the
    /// advice, not remove the grounding.
    fn retrieve(
        &self,
        game: &GameId,
        caps: RetrievalCaps,
    ) -> Result<Vec<StrategyDocument>, RetrievalError>;
}
