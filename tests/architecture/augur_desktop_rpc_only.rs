//! Architecture gate: the desktop app links nothing it should talk to.
//!
//! Decision 0001 puts the Augur desktop application behind the local JSON-RPC
//! socket rather than linking the runtime as a library. Everything that
//! decision buys, crash isolation in both directions, cheap upstream syncs
//! because the contract is a wire protocol rather than a set of Rust
//! signatures, evaporates the moment someone adds one `zeroclaw-*` dependency
//! "just for a type". The pull that direction is real and constant, which is
//! why this is a gate and not a guideline.
//!
//! Upstream holds `apps/zerocode` to the same rule with
//! `scripts/ci/zerocode_no_zeroclaw_dep_gate.sh`, and it has worked: zerocode
//! is a rich streaming client that links no backend crate. Augur copies the
//! precedent rather than inventing one.
//!
//! Two gates cover this, deliberately. The shell script parses the manifest in
//! CI and is what a reviewer can run by hand; this test additionally greps the
//! sources, so a path dependency snuck in through a `[patch]` section or a
//! `use zeroclaw_runtime::...` added alongside an unrelated edit still fails.

use std::fs;
use std::path::{Path, PathBuf};

const DESKTOP_MANIFEST: &str = "apps/augur-desktop/Cargo.toml";
const DESKTOP_SRC: &str = "apps/augur-desktop/src";

/// Crate-name prefixes the desktop app must never link.
const FORBIDDEN_DEPENDENCIES: &[&str] =
    &["zeroclaw-", "zeroclaw_", "augur-runtime", "augur_runtime"];

#[test]
fn the_desktop_manifest_declares_no_runtime_dependency() {
    let root = workspace_root();
    let manifest = fs::read_to_string(root.join(DESKTOP_MANIFEST))
        .expect("apps/augur-desktop/Cargo.toml must exist");

    let violations = manifest_violations(&manifest);

    assert!(
        violations.is_empty(),
        "apps/augur-desktop must not depend on any zeroclaw-* or augur-runtime crate; found:\n{}\n\n\
         The desktop app is an RPC-only surface: everything it knows arrives over \
         the socket, not by linking backend crates. See \
         docs/decisions/0001-runtime-integration.md.",
        violations.join("\n")
    );
}

#[test]
fn the_desktop_sources_import_no_runtime_crate() {
    let root = workspace_root();
    let mut violations = Vec::new();
    scan_dir(&root.join(DESKTOP_SRC), &root, &mut violations);
    violations.sort();

    assert!(
        violations.is_empty(),
        "apps/augur-desktop sources reference a runtime crate directly:\n{}",
        violations.join("\n")
    );
}

/// The manifest gate must reject what it claims to reject, including the
/// rename form, which is the one a determined workaround would reach for.
#[test]
fn the_manifest_gate_detects_a_renamed_dependency() {
    let plain = "[dependencies]\nzeroclaw-runtime = { path = \"../../crates/zeroclaw-runtime\" }\n";
    assert_eq!(
        manifest_violations(plain).len(),
        1,
        "a direct dependency must be flagged"
    );

    // `kernel = { package = "zeroclaw-runtime" }` links the same crate under a
    // local alias. A gate that only reads table keys would miss it.
    let renamed = "[dependencies]\nkernel = { package = \"zeroclaw-runtime\", path = \"../x\" }\n";
    assert_eq!(
        manifest_violations(renamed).len(),
        1,
        "a renamed dependency must be flagged"
    );

    let clean = "[dependencies]\nserde = \"1.0\"\n";
    assert!(
        manifest_violations(clean).is_empty(),
        "an unrelated dependency must not be flagged"
    );
}

/// Scan the dependency sections of a manifest.
///
/// Deliberately textual and section-scoped rather than a full TOML parse: the
/// `[package]` block legitimately contains the word `augur`, and the crate's
/// own documentation comments explain the rule using the very names being
/// searched for.
fn manifest_violations(manifest: &str) -> Vec<String> {
    let mut violations = Vec::new();
    let mut in_dependency_section = false;

    for (index, raw) in manifest.lines().enumerate() {
        let line = raw.trim();
        if line.starts_with('[') {
            in_dependency_section = line.contains("dependencies");
            continue;
        }
        if !in_dependency_section || line.starts_with('#') || line.is_empty() {
            continue;
        }
        for forbidden in FORBIDDEN_DEPENDENCIES {
            if line.contains(forbidden) {
                violations.push(format!("  {}:{}: {line}", DESKTOP_MANIFEST, index + 1));
                break;
            }
        }
    }
    violations
}

fn scan_dir(dir: &Path, root: &Path, violations: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            scan_dir(&path, root, violations);
            continue;
        }
        if path.extension().is_none_or(|ext| ext != "rs") {
            continue;
        }
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        let shown = path.strip_prefix(root).unwrap_or(&path).display();
        for (index, line) in text.lines().enumerate() {
            let code = line.trim();
            // Documentation is where the rule is explained, so it names the
            // crates on purpose. Only real code counts.
            if code.starts_with("//") {
                continue;
            }
            if code.contains("zeroclaw_") || code.contains("augur_runtime") {
                violations.push(format!("  {shown}:{}: {code}", index + 1));
            }
        }
    }
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}
