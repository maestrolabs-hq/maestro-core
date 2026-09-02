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

mod common;
use common::{repo_root, sources};

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

fn scan(banned: &[(&str, &str)], skip_self: bool, allow_provider_docs: bool) -> Vec<String> {
    let root = repo_root();
    let allowed_provider_docs = root.join("docs").join("providers");
    let mut found = Vec::new();
    for path in sources() {
        if skip_self && path.ends_with("vocabulary.rs") {
            continue;
        }
        // Provider-specific documentation is the explicit adapter boundary.
        if allow_provider_docs && path.starts_with(&allowed_provider_docs) {
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
    let found = scan(&banned, true, true);
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
    let found = scan(&banned, true, true);
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
    let found = scan(&banned, true, false);
    assert!(
        found.is_empty(),
        "A decision retired this wording, and it came back:\n\n{}\n",
        found.join("\n")
    );
}

/// A document naming a crate that does not exist is the drift the vocabulary
/// rules cannot see: it is not a banned word, it is a false claim. Deleting
/// three crates left `supervisor.md` still listing them in a table headed
/// "Crates", and every gate stayed green.
#[test]
fn no_document_names_a_crate_that_does_not_exist() {
    let root = repo_root();
    let existing: Vec<String> = fs::read_dir(root.join("crates"))
        .expect("crates/")
        .flatten()
        .filter(|e| e.path().is_dir())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();

    let mut found = Vec::new();
    for path in sources() {
        if path.extension().and_then(|x| x.to_str()) != Some("md") {
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
            for token in line.split("crates/").skip(1) {
                let name: String = token
                    .chars()
                    .take_while(|c| c.is_alphanumeric() || *c == '-')
                    .collect();
                if !name.is_empty() && !existing.contains(&name) {
                    found.push(format!(
                        "{rel}:{}: names crates/{name}, which does not exist\n    {}",
                        n + 1,
                        line.trim()
                    ));
                }
            }
        }
    }
    assert!(
        found.is_empty(),
        "A document names a crate that is not there:\n\n{}\n",
        found.join("\n")
    );
}
