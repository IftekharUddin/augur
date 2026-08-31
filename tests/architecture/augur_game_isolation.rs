//! Architecture gate: no concrete game identifier inside an Augur platform
//! crate.
//!
//! Augur's claim is that a second game arrives without platform changes. That
//! claim is cheap to make and easy to erode: one `if game_id ==
//! "hearthstone-battlegrounds"` in a shared crate, added under deadline, and
//! the seam is gone while every test still passes. This gate makes the erosion
//! fail the build the day it happens rather than at Milestone 4 when the second
//! game refuses to fit.
//!
//! The game identifiers are read from `games/*/game.yaml` rather than
//! hardcoded, so the gate covers games that do not exist yet.
//!
//! `augur-runtime` is deliberately **not** scanned: it holds the registry, and
//! naming every game exactly once, in one file, is the design. See
//! `docs/architecture/game-adapter.md`.

use std::fs;
use std::path::{Path, PathBuf};

/// Platform crates: game-agnostic by contract.
const PLATFORM_CRATES: &[&str] = &[
    "crates/augur-core/src",
    "crates/augur-game-api/src",
    "crates/augur-capture/src",
    "crates/augur-observation/src",
    "crates/augur-strategy/src",
    "crates/augur-recommendation/src",
    "crates/augur-policy/src",
    "crates/augur-voice/src",
];

#[test]
fn platform_crates_name_no_concrete_game() {
    let root = workspace_root();
    let game_ids = discover_game_ids(&root);
    assert!(
        !game_ids.is_empty(),
        "no games found under games/*/game.yaml; this gate would silently pass. \
         Either a game directory is missing or its manifest lost its `game_id` key."
    );

    let violations = scan_for_game_ids(&root, PLATFORM_CRATES, &game_ids);

    assert!(
        violations.is_empty(),
        "Concrete game identifiers found in platform crates.\n\n{}\n\n\
         Platform crates must stay game-agnostic: game-specific behavior belongs \
         in games/<id>/adapter, reached through the GameAdapter trait. The only \
         place a game may be named is the registry in augur-runtime. If you need \
         per-game behavior here, the trait is missing a method.",
        violations.join("\n")
    );
}

/// The gate is only worth having if it can fail, so prove it can.
#[test]
fn the_gate_detects_a_planted_identifier() {
    let root = workspace_root();
    let game_ids = discover_game_ids(&root);
    let planted = root.join("target/augur-arch-test-fixtures/platform-src");
    let _ = fs::remove_dir_all(&planted);
    fs::create_dir_all(&planted).expect("create fixture dir");
    let game = game_ids.first().expect("at least one game id");
    fs::write(
        planted.join("leak.rs"),
        format!("fn special_case() -> bool {{ id == \"{game}\" }}\n"),
    )
    .expect("write fixture");

    let relative = planted
        .strip_prefix(&root)
        .expect("fixture lives under the workspace root")
        .to_string_lossy()
        .to_string();
    let violations = scan_for_game_ids(&root, &[relative.as_str()], &game_ids);

    fs::remove_dir_all(&planted).ok();
    assert_eq!(
        violations.len(),
        1,
        "the scanner must flag a planted game identifier; got {violations:?}"
    );
}

fn scan_for_game_ids(root: &Path, roots: &[&str], game_ids: &[String]) -> Vec<String> {
    let mut violations = Vec::new();
    for relative in roots {
        scan_dir(&root.join(relative), root, game_ids, &mut violations);
    }
    violations.sort();
    violations
}

/// Every `game_id:` declared by a `games/*/game.yaml` manifest.
fn discover_game_ids(root: &Path) -> Vec<String> {
    let mut ids = Vec::new();
    let Ok(entries) = fs::read_dir(root.join("games")) else {
        return ids;
    };
    for entry in entries.flatten() {
        let manifest = entry.path().join("game.yaml");
        let Ok(text) = fs::read_to_string(&manifest) else {
            continue;
        };
        // Deliberately not pulling in a YAML parser for one top-level scalar:
        // the dependency would be carried by the whole workspace to read one
        // line, and the format is fixed by the manifest schema.
        for line in text.lines() {
            if let Some(value) = line.strip_prefix("game_id:") {
                let value = value.trim().trim_matches(['"', '\''].as_slice());
                if !value.is_empty() {
                    ids.push(value.to_string());
                }
                break;
            }
        }
    }
    ids.sort();
    ids.dedup();
    ids
}

fn scan_dir(dir: &Path, root: &Path, game_ids: &[String], violations: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            scan_dir(&path, root, game_ids, violations);
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
            for id in game_ids {
                if line.contains(id.as_str()) {
                    violations.push(format!("  {shown}:{}: {}", index + 1, line.trim()));
                }
            }
        }
    }
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}
