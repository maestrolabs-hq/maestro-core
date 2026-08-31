# Quality bar

What every repository in this organisation enforces, and what it deliberately
does not. Derived from an earlier Python governance baseline, but stated here as
our own rules rather than a comparison.

## The principle

Every claim is enforced by something that goes red. A rule nobody checks is a
comment, and a comment is not a rule.

Two consequences that shape everything below.

**One command, the same commands.** `just check` runs the gates. CI runs those
same commands, not equivalents. A gate that behaves differently in two places is
one nobody trusts.

**No gate is allowed to be inert.** A check that cannot fail is worse than no
check: it reports green while looking at nothing. When a gate has no input yet it
is removed from `check` with a note, not left in place pretending.

## The gates

```text
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo machete
cargo deny check
```

`maestro-pi-config` held a TypeScript toolchain that linted only its own
configuration files, and it was removed. The TypeScript gates exist in
`maestrolabs-hq/.github` and are attached when a repository has TypeScript,
not before -- `ts-types` over zero files reports green while looking at
nothing.

| Gate | Catches |
| --- | --- |
| `cargo fmt --check` | formatting drift; check-only in CI, `just fmt` writes |
| `cargo clippy -D warnings` | lints, with warnings as errors |
| `cargo test` | behaviour |
| `cargo machete` | dependencies declared but unused |
| `cargo deny check` | advisories, licences, bans, sources |
| `cargo test` (vocabulary) | sink names, borrowed vocabulary, retired wording |

`cargo deny` rather than `cargo audit`: it covers advisories *and* licences,
bans and sources. With an empty dependency list those sections guard against
that list changing quietly, which is the more likely failure.

## Two gates other languages need and Rust does not

**Type checking.** `mypy` and `ty` exist because Python's types are optional.
`cargo build` already fails on what they would catch.

**Architecture contracts.** Import Linter exists because any Python module can
import any other, so the contract must be re-asserted by a tool. In Cargo a
crate cannot reference a crate absent from its `[dependencies]` — it will not
compile.

That is only enforcement once crates actually depend on each other. With two
crates and no dependency between them, the build enforces nothing yet; the claim
becomes true when the first edge is real. `cargo machete` covers the other
direction — a dependency list quietly growing.

## Coverage is measured, not gated

`cargo llvm-cov --summary-only` reports; nothing fails on it.

A line-coverage floor would claim a guarantee it cannot make: `llvm-cov` reports
no branch data on this toolchain — the Branches column reads `-`. A percentage
that counts lines while implying branches is worse than no number.

## Three levels, three authorities

| Level | When | Authority |
| --- | --- | --- |
| Local hooks (prek) | every commit and merge-commit | **None.** Feedback only; a green hook cannot excuse a failed CI check |
| Fast CI | every push and pull request | **Required.** Merge blocks on it |
| Heavy CI | weekly or on request | **Evidence.** Never a required check; a stale run is not health |

Hooks are wired for `pre-commit` *and* `pre-merge-commit`. Without the second, a
merge that commits on its own brings in changes no gate ever saw.

Remote hooks are pinned to an immutable revision. Local gates run through
`repo: local`, `language: system`, `pass_filenames: false`, so they see the whole
project rather than the staged subset.

## Portability is part of the bar

No absolute path is written anywhere — not in code, configuration, task runner or
workflow. Paths derive from the operating system at run time. See
[ADR-0001](./adr/0001-paths-are-derived-never-written.md).

This is currently enforced by review and the absence of literals, not by a gate.
That is a known weakness: properly testing it means exercising Windows, macOS and
Linux.

## Not yet in place

**CI.** Both repositories are public, so rulesets and Actions are available and
free. Nothing is wired. The fast tier is close to a direct translation of
`just check`, plus secret scanning, dependency review and CodeQL default setup.

**Rulesets.** No required checks, no signed commits, no force-push protection.
Every gate today is local, and a local gate is bypassed by `--no-verify`.

**Most documentation gates.** `cargo test` now enforces vocabulary — no sink
implementation is named, no sink's vocabulary is borrowed, and wording a
decision retired cannot return. Nothing yet checks that a document's *claims*
match the code. This file saying a gate exists still does not make it exist.

## What this costs

The gates run in seconds on repositories this size, and they have already caught
six real defects: unused inter-crate dependencies, a formatter rewriting captured
configuration, a hook appending newlines that made drift unresolvable, a missing
type dependency hidden by an inert gate, a linter reading build artifacts, and a
generator dropping a tool name.

That is the argument for them. Not that they are rigorous, but that on a
repository with almost no code they were already finding things.
