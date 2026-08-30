//! Gates on prose.
//!
//! Every other gate here checks code. Nothing checked the documents, and they
//! drifted within hours: `protocol.md` and `ledger.md` went on describing a
//! process model that a later decision had replaced, and five green gates said
//! nothing. Markdown has no compiler, so this is the compiler.
//!
//! Two rules, both mechanical:
//!
//! 1. Maestro never names a sink implementation, and never borrows one's
//!    vocabulary. A sink you can name in the engine is a sink you cannot swap.
//! 2. Wording retired by a decision cannot come back. When a decision
//!    supersedes a phrase, the phrase joins the list below and can never
//!    silently reappear.

use std::fs;
use std::path::{Path, PathBuf};

/// Every `.rs`, `.md` and `.toml` in the repository, excluding build output.
fn sources() -> Vec<PathBuf> {
    fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        for e in entries.flatten() {
            let p = e.path();
            let name = e.file_name();
            let name = name.to_string_lossy();
            if name == "target" || name == ".git" || name == "node_modules" {
                continue;
            }
            if p.is_dir() {
                walk(&p, out);
            } else if matches!(
                p.extension().and_then(|x| x.to_str()),
                Some("rs" | "md" | "toml")
            ) {
                out.push(p);
            }
        }
    }
    let root = repo_root();
    let mut out = Vec::new();
    walk(&root, &mut out);
    out
}

/// The workspace root: this crate sits at `crates/cli`, and the rules apply to
/// every document in the repository, not just this crate's.
fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/cli has a workspace root above it")
        .to_path_buf()
}

/// Whole-word match. Substring matching flagged "knowing" and "growing" for
/// containing "wing", which is the kind of noise that gets a gate disabled.
fn contains_word(haystack: &str, needle: &str) -> bool {
    let bytes = haystack.as_bytes();
    haystack.match_indices(needle).any(|(i, _)| {
        let before = i.checked_sub(1).map(|b| bytes[b]);
        let after = bytes.get(i + needle.len()).copied();
        let boundary = |c: Option<u8>| !c.is_some_and(|c| c.is_ascii_alphanumeric() || c == b'_');
        boundary(before) && boundary(after)
    })
}

fn scan(banned: &[(&str, &str)], skip_self: bool) -> Vec<String> {
    let root = repo_root();
    let mut found = Vec::new();
    for path in sources() {
        if skip_self && path.ends_with("vocabulary.rs") {
            continue;
        }
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        let rel = path
            .strip_prefix(&root)
            .unwrap_or(&path)
            .display()
            .to_string();
        for (n, line) in text.lines().enumerate() {
            let lower = line.to_lowercase();
            for (term, why) in banned {
                if contains_word(&lower, term) {
                    found.push(format!("{rel}:{}: {why}\n    {}", n + 1, line.trim()));
                }
            }
        }
    }
    found
}

/// Maestro must not know which sink it is talking to. Naming one in the engine
/// is how a swappable dependency stops being swappable.
#[test]
fn no_sink_implementation_is_named() {
    let banned = [
        ("mempalace", "names a memory sink; say 'a memory sink'"),
        ("graphify", "names a graph sink; say 'a graph sink'"),
        ("codegraphcontext", "names a graph sink; say 'a graph sink'"),
    ];
    let found = scan(&banned, true);
    assert!(
        found.is_empty(),
        "Maestro named a sink implementation:\n\n{}\n",
        found.join("\n")
    );
}

/// Borrowing a sink's data model leaks it just as surely as naming it. "Wing"
/// and "room" are one memory engine's filing structure; Maestro's own term for
/// the same idea is the project a recall is scoped to.
#[test]
fn no_sink_vocabulary_is_borrowed() {
    let banned = [
        (
            "wing",
            "a sink's filing vocabulary; say 'scope' or 'project'",
        ),
        ("drawer", "a sink's storage vocabulary; say 'record'"),
        ("palace", "a sink's storage vocabulary; say 'store'"),
    ];
    let found = scan(&banned, true);
    assert!(
        found.is_empty(),
        "Maestro borrowed a sink's vocabulary:\n\n{}\n",
        found.join("\n")
    );
}

/// Wording a decision retired cannot return. Each entry is a phrase that was
/// true once and is now false; leaving them findable is what stops the next
/// restructure from leaving half the documents behind.
#[test]
fn no_retired_wording_survives() {
    let banned = [
        (
            "one process per exchange",
            "retired: the CLI is per-exchange, the supervisor is resident. Say which.",
        ),
        (
            "there is no daemon",
            "retired: the supervisor is resident and owns the ledger",
        ),
        (
            "spawn-per-event",
            "retired: replaced by the resident supervisor",
        ),
        (
            "delivery to consumers",
            "retired: 'consumer' became 'sink' when the crate was renamed",
        ),
    ];
    let found = scan(&banned, true);
    assert!(
        found.is_empty(),
        "A decision retired this wording, and it came back:\n\n{}\n",
        found.join("\n")
    );
}
