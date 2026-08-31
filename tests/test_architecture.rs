//! Workspace architecture-invariant test entry. Each submodule here is a
//! detector that fails the workspace test suite when the corresponding
//! invariant is violated. See AGENTS.md §1 ("ABSOLUTE RULE — SINGLE
//! SOURCE OF TRUTH") for context on why these gates exist.

#[path = "architecture/no_duplicate_state.rs"]
mod no_duplicate_state;

#[path = "architecture/config_save_isolation.rs"]
mod config_save_isolation;

#[path = "architecture/release_workflow.rs"]
mod release_workflow;

#[path = "architecture/cli_fluent_coverage.rs"]
mod cli_fluent_coverage;

#[path = "architecture/desktop_release.rs"]
mod desktop_release;

#[path = "architecture/container_release.rs"]
mod container_release;

// ── Augur invariants ──────────────────────────────────────────────────────
// Added by the fork. Kept in one block at the end so upstream's list above
// merges without contest; see docs/architecture/fork-touchpoints.md.

#[path = "architecture/augur_game_isolation.rs"]
mod augur_game_isolation;

#[path = "architecture/augur_desktop_rpc_only.rs"]
mod augur_desktop_rpc_only;

#[path = "architecture/augur_no_input_synthesis.rs"]
mod augur_no_input_synthesis;
