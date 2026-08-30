# ADR 0001: Paths are derived, never written

- Status: Accepted
- Date: 2026-08-30

## Context

Maestro must run on Windows, macOS and Linux, including WSL, with the same
behaviour on each. A machine is reinstalled, patched, or replaced, and the same
configuration has to come back up somewhere else without editing.

Every hardcoded path is a bet on one machine. The first version of this
repository's `justfile` already lost that bet twice in one afternoon: it built
`PATH` from `$HOME`, which does not exist on Windows, and joined entries with
`:`, which is not the Windows separator. An earlier draft anchored the toolchain
with `../../../` from the justfile, which resolved correctly only because the
repository happened to sit three directories below home.

Both looked fine on the machine they were written on. That is the failure mode:
a hardcoded path does not fail where it is authored.

## Decision

No absolute path is written down anywhere in Maestro — not in code, not in
configuration, not in a task runner, not in a workflow.

Paths are derived at run time from the operating system: the user's home
directory, the platform's configuration and data directories, the repository
root discovered from the working directory. Separators and list delimiters come
from the platform, never from an assumption about it.

Documentation uses placeholders rather than a real path from someone's machine,
so an example can never be copied into a config and appear to work.

This applies to every Maestro repository and to anything Maestro deploys.

## Consequences and risks

Configuration produced on one machine restores on another, which is the point.

Derivation is more verbose than a literal, and it can be wrong in ways a literal
cannot: a derived path that resolves to the wrong place fails on every machine
rather than one. The mitigation is that path derivation is checked, and
`just doctor` prints what actually resolved instead of what was intended.

Testing this properly means exercising all three platforms. Until then the rule
is enforced by review and by the absence of literals, not by a gate — a known
weakness rather than a claim of coverage.

## Non-goals

This does not standardise *which* directories Maestro uses on each platform.
That is a separate decision, and choosing them is not the same as refusing to
hardcode them.
