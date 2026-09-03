# Rust Skills

Rust Skills is a 265-rule idiomatic-Rust review guide packaged as a single
`SKILL.md` plus a `rules/` tree, current for Rust 1.96 (the 2024 edition). It
is a skill, not an MCP server: no process to start, no `PATH` binary, no
provider state. It is advisory guidance the agent reads, not a source of
truth the estate defers to.

## What it does and why Maestro uses it

The skill indexes 265 rules across 26 categories — ownership, error handling,
async, concurrency, unsafe code, API design, memory and numeric safety,
conversions, serde, pattern matching, macros, closures, observability,
testing, and common anti-patterns — each as its own file under `rules/`,
loaded progressively as the agent needs a specific category rather than all
at once. Maestro uses it because the estate is Rust throughout —
`maestro-core`, `maestro-project-documentation`, and `maestro-llamacpp` — so a
second, independently-authored opinion on idiomatic Rust is a useful review
aid when writing or reviewing code in any of them.

This skill replaces `apollographql/skills`' `rust-best-practices`, installed
and documented here earlier the same week. See Notes for why.

## Vocabulary mapping

Omitted: the skill's terms (ownership, ADR-style rule prefixes such as
`anti-`, `err-`, `opt-`, `conc-`, Clippy, `unsafe`, serde) are standard Rust
and tooling vocabulary, not a sink's internal jargon needing translation into
an estate term. There is nothing here for the vocabulary gate's mapping
convention to translate.

## Identity

| Field | Value |
| --- | --- |
| Upstream | `leonardomso/rust-skills` (MIT) |
| Distribution | agent skill — **not** an MCP server, no `PATH` binary, no provider state |
| Installed | `~/.pi/agent/skills/rust-skills`, by direct copy of the pinned commit's tree (no `.git`) — see Notes for why neither wrapper CLI installs this repo cleanly |
| Skill version | `1.5.1` (`SKILL.md`'s `metadata.version` field) |
| Pinned to | commit `fd2a861ab0406a4ac536a55274d14ea6fd1ca9c9` (branch `master`, 2026-06-14) |
| Pin rationale | Upstream has **0 tagged releases and 0 tags** (verified via `gh api repos/leonardomso/rust-skills/releases,tags`). The estate's own rule for a git package with no tags is to pin to a SHA, exactly as `maestro-pi-config` already does for its own tagless git packages (`maestro-pi-config/config/README.md:84-92`): "Every git package is pinned to a ref... the other two have no tags at their installed commit, so they pin to a SHA." Pinnability was never actually blocked by the lack of a tag. |
| Provenance | **1** contributor via `gh api repos/leonardomso/rust-skills/contributors`; **0** tagged releases; last commit 2026-06-14. Stated plainly, not hidden: this is a smaller, less-institutional upstream than Apollo GraphQL's. |

## Skill surface

| Component | Purpose |
| --- | --- |
| `SKILL.md` | 499-line quick-reference index across the 26 rule categories, current for Rust 1.96 / 2024 edition. ~8.2x the byte size of the Apollo `SKILL.md` it replaces (37.8 KB vs 4.6 KB) — the always-resident cost at activation; the 265 `rules/` files load progressively, not all at once. |
| `rules/` (265 files) | One markdown file per rule, each with a rationale and a compile-checked example |
| `checks/` | leonardomso's own example-verification tooling (a Cargo project plus `analyze.py` / `validate.py` / `check.sh` / `baseline.txt`). **Not part of the published skill surface** — it compile-checks the guide's own code examples in its upstream CI, not the estate's code, and adds no gate here. |

## Usage

In Pi, after a reload, the skill activates automatically when writing,
reviewing, or refactoring Rust code (description-matched, the same as
Impeccable), or can be invoked explicitly:

```text
/rust-skills
```

## Notes

- **Advisory, not authoritative.** Where this skill's guidance disagrees with
  `maestro-core`'s own `AGENTS.md`, its `docs/adr/` entries, or an enforced
  gate, the estate's recorded decision wins. This skill informs a reviewer's
  judgment; it does not override one.
- **Known divergences, checked against the current estate** (`clippy.toml`,
  `Cargo.toml`'s `[workspace.lints]`, and every `.rs` file under `crates/`, as
  of this installation):
  - **`panic = "abort"`** appears seven times across
    `rules/opt-codegen-units.md`, `rules/opt-lto-release.md` (×3),
    `rules/perf-release-profile.md` (×2), and `SKILL.md` — always framed as a
    release-profile optimization tradeoff, never as a mandate. It is dormant
    here: the estate has **zero** `#[should_panic]` tests across every `.rs`
    file, so there is nothing for it to conflict with today.
  - **`#[allow(clippy::…)]` vs `#[expect(clippy::…)]`** — this skill uses
    `#[allow(clippy::…)]` exclusively (5 rule files; it never mentions
    `#[expect(`), always paired with a justification comment. The estate has
    **zero** lint suppressions of either form today, so this is latent, not
    active — but if one is ever needed, `#[expect(...)]` is the more current
    idiom (it starts failing the moment the suppression stops being
    necessary), and worth preferring over what this guide shows.
  - **Clippy strictness** — the estate's `just check` runs `clippy::pedantic`,
    `too_many_lines`, and `cognitive_complexity` at `warn` workspace-wide, and
    a `too-many-arguments-threshold` of 5 (`clippy.toml`) against Clippy's
    default of 7 — stricter than this guide's baseline recommendations, not
    weaker.
- **Install method.** Neither wrapper CLI installs this repository cleanly.
  `gh skill install leonardomso/rust-skills` refuses it outright: "no skills
  found... this repository may be a curated list rather than a skills
  publisher" — `gh skill` only discovers `skills/*/SKILL.md`-style layouts,
  and this repository's `SKILL.md` sits at the repository root. `npx skills
  add leonardomso/rust-skills --agent pi --copy` installed successfully but
  defaulted to **project scope**, writing `.pi/skills/rust-skills/` and a
  `skills-lock.json` into the `maestro-core` working tree itself — both were
  removed before this change was committed. The skill was installed instead
  by copying the pinned commit's tree (everything except `.git`) directly
  into `~/.pi/agent/skills/rust-skills/`.
- **Why this replaces Apollo.** Apollo's `chapter_04.md:43-50` recommends an
  `if let Ok(..) else { ... }` construct that does not compile, and Apollo is
  silent on the Rust 2024 edition the estate pins. This skill's examples are
  compile-checked in its own upstream CI (`checks/`) and are explicitly
  current for Rust 1.96 / the 2024 edition. The earlier decision to keep
  Apollo rested on treating "no upstream tags" as disqualifying; the estate's
  actual pin rule explicitly allows a SHA pin for a tagless package, so that
  was never a real blocker.
- This skill was installed and documented only; it was not run against this
  codebase as part of this change.
- Never registered with Pi's MCP registry; it is a skill, not a server.
- `wshobson/agents`' `rust-async-patterns` skill, evaluated separately, is
  redundant with this skill's own async coverage and was not installed.
