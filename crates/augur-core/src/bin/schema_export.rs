//! Generate the JSON Schemas for Augur's common contracts.
//!
//! The Rust types are canonical; these schemas are derived from them, never the
//! other way round. That direction matters: a hand-maintained schema drifts
//! from the code the moment someone adds a field, and the two then disagree
//! silently for as long as nobody looks.
//!
//! Usage:
//!
//! ```text
//! cargo run -p augur-core --bin augur-schema-export -- crates/augur-core/schemas
//! ```
//!
//! `crates/augur-core/tests/schema_drift.rs` runs the same generation and fails
//! if the committed files differ, so the committed schemas cannot go stale
//! without CI noticing.

use std::path::PathBuf;

use augur_core::{GameStateEnvelope, Recommendation};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir: PathBuf = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "crates/augur-core/schemas".to_string())
        .into();
    std::fs::create_dir_all(&out_dir)?;

    for (name, json) in augur_core::validate::generated_schemas() {
        let path = out_dir.join(name);
        std::fs::write(&path, &json)?;
        println!("wrote {}", path.display());
    }

    // Referenced so the imports document what is exported even as the list
    // grows; the generation itself lives beside the types.
    let _ = std::any::type_name::<(GameStateEnvelope, Recommendation)>();
    Ok(())
}
