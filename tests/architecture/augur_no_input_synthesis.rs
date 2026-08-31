//! Architecture gate: Augur synthesizes no input. Ever.
//!
//! "A coach, never a bot" is the product's central promise, and it is the one
//! claim that cannot be walked back after the fact. A single call that moves a
//! cursor or presses a key turns a coaching tool into automation software, with
//! everything that implies for the game's terms of service and for the people
//! who trust it.
//!
//! Policy documents do not enforce anything. This does: it greps every Augur
//! source file for the platform symbols that make input synthesis possible and
//! fails the build if one appears. A contributor who genuinely needs one of
//! these has to change this file, in a pull request, where the change is
//! visible and reviewable, which is exactly the conversation that should
//! happen.
//!
//! Scope is Augur code only. Inherited `zeroclaw-*` crates are upstream's
//! surface; the coaching agent's access to them is constrained separately by
//! the tool registry (`SecurityPolicy.allowed_tools`, sealed by
//! `ScopedToolRegistry::assemble`).

use std::fs;
use std::path::{Path, PathBuf};

/// Every Augur-owned source root.
const AUGUR_ROOTS: &[&str] = &[
    "crates/augur-core/src",
    "crates/augur-game-api/src",
    "crates/augur-capture/src",
    "crates/augur-observation/src",
    "crates/augur-strategy/src",
    "crates/augur-recommendation/src",
    "crates/augur-policy/src",
    "crates/augur-voice/src",
    "crates/augur-runtime/src",
    "apps/augur-desktop/src",
    "games",
];

/// Platform symbols that synthesize input, plus the crates that wrap them.
///
/// Grouped by the mechanism they represent so that a failure message tells a
/// contributor what the gate thinks they were doing.
const INPUT_SYNTHESIS_SYMBOLS: &[(&str, &str)] = &[
    // macOS
    ("CGEventPost", "macOS Quartz event injection"),
    ("CGEventCreateKeyboardEvent", "macOS synthetic key events"),
    ("CGEventCreateMouseEvent", "macOS synthetic mouse events"),
    ("CGWarpMouseCursorPosition", "macOS cursor warping"),
    // Windows
    ("SendInput", "Windows synthetic input"),
    ("keybd_event", "Windows legacy key injection"),
    ("mouse_event", "Windows legacy mouse injection"),
    (
        "PostMessageW",
        "Windows message injection into another window",
    ),
    ("SetCursorPos", "Windows cursor positioning"),
    // X11 / Linux
    ("XTestFakeKeyEvent", "X11 XTEST key injection"),
    ("XTestFakeButtonEvent", "X11 XTEST button injection"),
    ("uinput", "Linux uinput virtual device"),
    // Cross-platform crates whose entire purpose is input synthesis
    ("enigo", "the enigo input-automation crate"),
    ("rdev", "the rdev input-simulation crate"),
    ("autopilot", "the autopilot automation crate"),
    // Process memory access, banned for the same reason
    ("ReadProcessMemory", "reading another process's memory"),
    ("vm_read_overwrite", "macOS task memory reads"),
    ("process_vm_readv", "Linux process memory reads"),
];

#[test]
fn augur_crates_contain_no_input_synthesis() {
    let root = workspace_root();
    let violations = scan(&root, AUGUR_ROOTS);

    assert!(
        violations.is_empty(),
        "Input-synthesis or process-memory symbols found in Augur code.\n\n{}\n\n\
         Augur is a coach, not a bot: it never sends input to a game, never reads \
         game memory, and never automates play. If you have a legitimate need for \
         one of these symbols, changing this gate is part of the change, and the \
         product rule in docs/architecture/security-and-privacy.md is what you are \
         arguing against.",
        violations.join("\n")
    );
}

/// A gate that cannot fail is decoration. This proves it can.
#[test]
fn the_gate_detects_planted_input_synthesis() {
    let root = workspace_root();
    let planted = root.join("target/augur-arch-test-fixtures/input-synthesis");
    let _ = fs::remove_dir_all(&planted);
    fs::create_dir_all(&planted).expect("create fixture dir");
    fs::write(
        planted.join("bot.rs"),
        "unsafe fn press() { SendInput(1, &input, size); }\n",
    )
    .expect("write fixture");

    let relative = planted
        .strip_prefix(&root)
        .expect("fixture lives under the workspace root")
        .to_string_lossy()
        .to_string();
    let violations = scan(&root, &[relative.as_str()]);

    fs::remove_dir_all(&planted).ok();
    assert_eq!(
        violations.len(),
        1,
        "the scanner must flag a planted input-synthesis call; got {violations:?}"
    );
}

fn scan(root: &Path, roots: &[&str]) -> Vec<String> {
    let mut violations = Vec::new();
    for relative in roots {
        scan_dir(&root.join(relative), root, &mut violations);
    }
    violations.sort();
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
        // Manifests count: pulling in `enigo` is the violation, whether or not
        // anything calls it yet.
        let is_source = path.extension().is_some_and(|ext| ext == "rs");
        let is_manifest = path.file_name().is_some_and(|name| name == "Cargo.toml");
        if !is_source && !is_manifest {
            continue;
        }
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        let shown = path.strip_prefix(root).unwrap_or(&path).display();
        for (index, line) in text.lines().enumerate() {
            for (symbol, mechanism) in INPUT_SYNTHESIS_SYMBOLS {
                if line.contains(symbol) {
                    violations.push(format!(
                        "  {shown}:{}: {} ({mechanism})",
                        index + 1,
                        line.trim()
                    ));
                }
            }
        }
    }
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}
