//! The committed JSON Schemas must match what the Rust types generate.
//!
//! Committed generated artifacts rot. Someone adds a field to
//! `GameStateEnvelope`, the Rust side is fine, and the schema a non-Rust
//! consumer validates against silently describes the previous contract. This
//! test is the only thing standing between that and a very confusing bug
//! report.
//!
//! When it fails, the fix is to regenerate, not to edit the JSON:
//!
//! ```text
//! cargo run -p augur-core --bin augur-schema-export -- crates/augur-core/schemas
//! ```

use std::path::PathBuf;

fn schema_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("schemas")
}

#[test]
fn committed_schemas_match_the_rust_types() {
    let dir = schema_dir();
    let mut mismatches = Vec::new();

    for (name, expected) in augur_core::validate::generated_schemas() {
        let path = dir.join(name);
        match std::fs::read_to_string(&path) {
            Ok(found) if found == expected => {}
            Ok(_) => mismatches.push(format!("  {name}: committed file is stale")),
            Err(error) => mismatches.push(format!("  {name}: {error}")),
        }
    }

    assert!(
        mismatches.is_empty(),
        "Committed schemas do not match the Rust types.\n\n{}\n\n\
         The Rust types are canonical. Regenerate rather than editing the JSON:\n  \
         cargo run -p augur-core --bin augur-schema-export -- crates/augur-core/schemas",
        mismatches.join("\n")
    );
}

#[test]
fn every_generated_schema_is_committed_and_nothing_else_is() {
    // A schema left behind after a type is removed is as misleading as a stale
    // one, and neither the export binary nor the drift check above would
    // notice, because both only look at what the code currently generates.
    let dir = schema_dir();
    let expected: std::collections::BTreeSet<String> = augur_core::validate::generated_schemas()
        .into_iter()
        .map(|(name, _)| name.to_string())
        .collect();

    let found: std::collections::BTreeSet<String> = std::fs::read_dir(&dir)
        .expect("schemas directory must exist")
        .filter_map(Result::ok)
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .filter(|name| name.ends_with(".json"))
        .collect();

    assert_eq!(
        found, expected,
        "the schemas directory must contain exactly the generated schemas"
    );
}
