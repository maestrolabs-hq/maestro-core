//! Shared by the gates. Kept here so they do not each carry their own copy of
//! the walk -- which the duplication gate would rightly flag.

use std::fs;
use std::path::{Path, PathBuf};

/// The workspace root: this crate sits at `crates/cli`, and the rules apply to
/// every document in the repository, not just this crate's.
pub fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/cli has a workspace root above it")
        .to_path_buf()
}

/// Every `.rs`, `.md` and `.toml` in the repository, excluding build output.
pub fn sources() -> Vec<PathBuf> {
    fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        for e in entries.flatten() {
            let p = e.path();
            let name = e.file_name();
            let name = name.to_string_lossy();
            if name == "target"
                || name == ".git"
                || name == "node_modules"
                || name == ".worktrees"
                || name == ".superpowers"
            {
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
    let mut out = Vec::new();
    walk(&repo_root(), &mut out);
    out
}
