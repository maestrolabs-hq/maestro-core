# Rust Best Practices

Rust Best Practices is Apollo GraphQL's idiomatic-Rust review guide, packaged
as a `SKILL.md` contract plus nine reference chapters. It is a skill, not an
MCP server: no process to start, no `PATH` binary, no provider state. It is
advisory guidance the agent reads, not a source of truth the estate defers to.

## What it does and why Maestro uses it

The skill summarizes Apollo's [Rust Best Practices
Handbook](https://github.com/apollographql/rust-best-practices) into a
quick-reference `SKILL.md` and nine chapters covering borrowing and ownership,
Clippy discipline, performance, error handling, testing, generics and
dispatch, the type state pattern, documentation, and pointer/thread-safety
types. Maestro uses it because the estate is Rust throughout —
`maestro-core`, `maestro-project-documentation`, and `maestro-llamacpp` — so a
second, independently-authored opinion on idiomatic Rust is a useful review
aid when writing or reviewing code in any of them.

## Identity

| Field | Value |
| --- | --- |
| Upstream | `apollographql/skills` (MIT), skill path `skills/rust-best-practices` |
| Distribution | agent skill — **not** an MCP server, no `PATH` binary, no provider state |
| Installed | `~/.pi/agent/skills/rust-best-practices` via `gh skill install apollographql/skills rust-best-practices --agent pi --scope user --pin v1.2.9` |
| Skill version | `1.1.1` (the `metadata.version` field inside `SKILL.md`) |
| Reviewed pin | repository tag `v1.2.9`, commit `c288eb80629dd2309eed81f23d693f66a452d043`; this skill's directory tree SHA `6ef87177d5674b39adc1e58a46149299feefe43a` |
| Tag-vs-version note | `apollographql/skills` tags the whole monorepo on every content change, not each skill individually — there is no tag literally named `v1.1.1`. Tag `v1.2.9` is the release confirmed (by fetching its tagged content before installing) to carry `SKILL.md`'s own `metadata.version: "1.1.1"`, so pinning to `v1.2.9` pins exactly the content that version number identifies. |

## Vocabulary mapping

Omitted: the skill's terms (Clippy, `thiserror`, `anyhow`, the type state
pattern, static/dynamic dispatch) are standard Rust and tooling vocabulary,
not a sink's internal jargon needing translation into an estate term. There
is nothing here for the vocabulary gate's mapping convention to translate.

## Skill surface

| Component | Purpose |
| --- | --- |
| `SKILL.md` | Quick-reference guide and the chapter index |
| `references/chapter_01.md` | Coding Styles and Idioms — borrowing vs cloning, `Copy`, `Option`/`Result`, iterators, when to extract a function |
| `references/chapter_02.md` | Clippy and Linting — configuration, key lints, workspace lint setup |
| `references/chapter_03.md` | Performance Mindset — profiling, redundant clones, stack vs heap, zero-cost abstractions |
| `references/chapter_04.md` | Error Handling — `Result` vs panic, `thiserror` vs `anyhow`, error hierarchies |
| `references/chapter_05.md` | Automated Testing — test naming, one assertion per test, snapshot testing |
| `references/chapter_06.md` | Generics and Dispatch — static vs dynamic dispatch, trait objects |
| `references/chapter_07.md` | Type State Pattern — compile-time state safety, when to use it |
| `references/chapter_08.md` | Comments vs Documentation — when to comment, doc comments, rustdoc |
| `references/chapter_09.md` | Understanding Pointers — thread safety, `Send`/`Sync`, pointer types |

## Usage

Pi loads the skill automatically once installed (after a reload); the agent
applies it when writing, reviewing, or refactoring Rust code, choosing
between borrowing and cloning, adding error handling, or writing tests. There
is no explicit invocation command — activation is guidance-based, driven by
`SKILL.md`'s `description` frontmatter, the same as Impeccable.

## Notes

- **Advisory, not authoritative.** Where this skill's guidance disagrees with
  `maestro-core`'s own `AGENTS.md`, its `docs/adr/` entries, or an enforced
  gate, the estate's recorded decision wins. This skill informs a reviewer's
  judgment; it does not override one.
- **Concrete overlaps checked against the current estate** (`clippy.toml`,
  `Cargo.toml`'s `[workspace.lints]`, the `justfile`'s `check` recipe, and
  `crates/*/src`, `crates/*/tests`, as of this installation):
  - **`unwrap`/`expect` outside tests** — the skill bans this; the estate has
    no written rule, but zero production-code occurrences exist today, so the
    two are currently aligned in practice, not by a recorded decision.
  - **Clippy strictness** — the skill recommends
    `cargo clippy --all-targets --all-features --locked -- -D warnings`. The
    estate's `just check` runs the same without `--locked`, plus
    `clippy::pedantic`, `too_many_lines`, and `cognitive_complexity` at `warn`
    workspace-wide, and a `too-many-arguments-threshold` of 5 (`clippy.toml`)
    against Clippy's default of 7 — stricter than the skill's baseline, not
    weaker.
  - **`thiserror` vs `anyhow`** — the skill recommends `thiserror` for
    libraries and `anyhow` only for binaries. Neither crate is a dependency
    yet; the estate has not recorded an error-handling strategy. If one is
    adopted, record it as a decision rather than adopting the skill's default
    silently.
  - **`#[expect(clippy::lint)]` over `#[allow(...)]`** — the skill's
    preference; no lint suppression of either form exists in the estate yet,
    so there is nothing to reconcile.
  - **`#![deny(missing_docs)]`** — the skill suggests this for libraries. The
    estate does not enable it; instead, every `.rs` file under `crates/`
    currently opens with a `//!` module brief by convention, not by a gate.
- This skill was installed and documented only; it was not run against this
  codebase as part of this change.
- Never registered with Pi's MCP registry; it is a skill, not a server.
