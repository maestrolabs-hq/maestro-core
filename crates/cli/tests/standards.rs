//! Size, language and duplication. Mechanical limits, each verified by
//! injection in maestro-pi-config, where they were written first.

use std::collections::BTreeSet;
use std::fs;
use std::process::Command;

mod common;
use common::{repo_root, sources};

/// Governance's Python bar is 200; Rust needs more room for the same content.
/// A dumping-ground tripwire, not a design rule -- the design rules are the
/// per-function clippy lints in `Cargo.toml`.
const MAX_MODULE_LINES: usize = 250;

/// Latin-1 accented letters plus the OE ligatures, built from code points so
/// this file stays accent-free and cannot fail its own test.
const ACCENT_RANGES: &[(u32, u32)] = &[
    (0x00C0, 0x00D6),
    (0x00D8, 0x00F6),
    (0x00F8, 0x00FF),
    (0x0152, 0x0153),
];

/// Duplicate pairs looked at and kept, with the reason. An allowlist rather
/// than a threshold: structural similarity is not always duplication worth
/// removing, and a threshold alone would force bad merges.
const ACCEPTED_DUPLICATION: &[(&str, &str)] = &[(
    "no_sink_implementation_is_named <-> no_sink_vocabulary_is_borrowed",
    "Two scans over the same corpus for different banned words. Merging them \
     would hide which rule failed.",
)];

#[test]
fn no_module_becomes_a_dumping_ground() {
    let over: Vec<String> = sources()
        .iter()
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("rs"))
        .filter_map(|p| {
            let text = fs::read_to_string(p).ok()?;
            let n = text
                .lines()
                .position(|l| l.trim_start().starts_with("#[cfg(test)]"))
                .unwrap_or_else(|| text.lines().count());
            (n > MAX_MODULE_LINES).then(|| format!("  {}: {n} lines", p.display()))
        })
        .collect();
    assert!(
        over.is_empty(),
        "Module over {MAX_MODULE_LINES} lines:\n{}\n",
        over.join("\n")
    );
}

#[test]
fn all_prose_is_english() {
    let accented = |c: char| {
        ACCENT_RANGES
            .iter()
            .any(|(lo, hi)| (*lo..=*hi).contains(&(c as u32)))
    };
    let root = repo_root();
    let mut found = Vec::new();
    for path in sources() {
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        for (n, line) in text.lines().enumerate() {
            if line.chars().any(accented) {
                let rel = path.strip_prefix(&root).unwrap_or(&path);
                found.push(format!("  {}:{}: {}", rel.display(), n + 1, line.trim()));
            }
        }
    }
    assert!(
        found.is_empty(),
        "Accented characters, which usually means French:\n\n{}\n",
        found.join("\n")
    );
}

fn detected_duplication() -> BTreeSet<String> {
    let out = Command::new("similarity-rs")
        .args(["--threshold", "0.85", "crates"])
        .current_dir(repo_root())
        .output()
        .expect(
            "similarity-rs must be installed: cargo binstall similarity-rs. \
             A gate that skips when its tool is missing reports green while \
             looking at nothing.",
        );
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .filter(|l| !l.trim_start().starts_with("Classes:"))
        .filter_map(|line| {
            let (left, right) = line.split_once(" <-> ")?;
            let name = |s: &str| s.split_whitespace().last().map(str::to_owned);
            Some(format!("{} <-> {}", name(left)?, name(right)?))
        })
        .collect()
}

#[test]
fn no_duplication_is_unaccounted_for() {
    let accepted: BTreeSet<&str> = ACCEPTED_DUPLICATION.iter().map(|(p, _)| *p).collect();
    let unexplained: Vec<String> = detected_duplication()
        .into_iter()
        .filter(|p| !accepted.contains(p.as_str()))
        .collect();
    assert!(
        unexplained.is_empty(),
        "Duplication with no recorded decision:\n\n{}\n\n\
         Factor out what is shared, or record the pair with its reason.\n",
        unexplained.join("\n")
    );
}

/// An allowlist nobody prunes becomes excuses for code that no longer exists.
#[test]
fn no_accepted_duplication_has_gone_stale() {
    let found = detected_duplication();
    let stale: Vec<&str> = ACCEPTED_DUPLICATION
        .iter()
        .map(|(p, _)| *p)
        .filter(|p| !found.contains(*p))
        .collect();
    assert!(
        stale.is_empty(),
        "No longer duplicated, remove from the list:\n\n  {}\n",
        stale.join("\n  ")
    );
}
