# Rust MCP Server Generator

Rust MCP Server Generator is a scaffolding skill packaged as a single
`SKILL.md`. It is a skill, not an MCP server: no process to start, no `PATH`
binary, no provider state. It is a **generator** — it activates when asked to
scaffold a new Rust MCP server, not when reviewing existing code — and its
output is advisory: a starting point, not a source of truth the estate
defers to.

## What it does and why Maestro uses it

The skill generates a complete Rust MCP server project — `Cargo.toml`,
`.gitignore`, `README.md`, a `main.rs`/`handler.rs`/`state.rs` skeleton, and
`tools/`, `prompts/`, and `resources/` modules with an integration test —
built on the official `rmcp` SDK, supporting stdio, SSE, and HTTP transports.
Maestro **consumes** MCP servers today (seven providers wired in the governed
MCP configuration) but does not yet **author** one in Rust. This skill is
kept installed as a deferred, dormant capability for the day Maestro decides
to expose a Rust MCP server, rather than installed at that later point under
time pressure. It was not exercised against this codebase as part of this
change.

## Vocabulary mapping

Omitted: the skill's terms (`rmcp`, stdio/SSE/HTTP transport, tool/prompt/
resource modules) are standard MCP and Rust tooling vocabulary, not a sink's
internal jargon needing translation into an estate term. There is nothing
here for the vocabulary gate's mapping convention to translate.

## Identity

| Field | Value |
| --- | --- |
| Upstream | `github/awesome-copilot` (skill path `skills/rust-mcp-server-generator`) |
| License | MIT (Copyright GitHub, Inc.) — the repository's top-level `LICENSE`; the skill's own frontmatter declares no separate license |
| Distribution | agent skill — **not** an MCP server, no `PATH` binary, no provider state |
| Installed | `~/.pi/agent/skills/rust-mcp-server-generator`, by direct copy of the single `SKILL.md` file (no `references/` subdirectory exists upstream) |
| Skill version | none declared — the frontmatter carries only `name` and `description`, no `version` field |
| Pinned to | commit `2ba72cd14253500bbb747b5f01e72dd03fbafcb0` (branch `main`, 2026-09-03) |
| Pin rationale | No per-skill tag exists for this path inside the `awesome-copilot` monorepo, so it pins to a commit SHA — the same practice `maestro-pi-config` already uses for its own tagless git packages (`maestro-pi-config/config/README.md:84-92`) and the same method used for the `rust-skills` install. |
| Install method | Direct copy, not `gh skill` or `npx skills add`. The prior `rust-skills` install showed `npx skills add` defaults to **project scope**, writing into the repository's own working tree; direct copy into the global Pi skills directory avoids that failure mode entirely. |

## Skill surface

| Component | Purpose |
| --- | --- |
| `SKILL.md` | The entire skill: a generation contract that asks for a project name, description, transport type, and tool list, then emits `Cargo.toml`, `.gitignore`, `README.md`, and a full `src/` + `tests/` skeleton wired to the `rmcp` SDK |

## Usage

In Pi, after a reload, the skill activates when asked to scaffold a Rust MCP
server (description-matched), or can be invoked explicitly:

```text
/rust-mcp-server-generator
```

## Notes

- **Advisory, not authoritative — and its output needs pruning.** This skill
  generates a full multi-module project structure (separate `tools/`,
  `prompts/`, `resources/`, `state.rs`, and an integration test) regardless
  of how small the intended server is. `maestro-core` deleted four crates
  because "a seam is only real when something varies across it"
  (`Cargo.toml:7-9`), and `AGENTS.md` calls for surgical changes and no
  speculative structure. Generated output is a starting point: prune it to
  what the task actually needs, and bring it into line with the estate's
  `clippy.toml` lints and `AGENTS.md` before it is treated as real code.
- **Edition mismatch.** The skill's generated `Cargo.toml` template pins
  `edition = "2021"`; the estate pins the 2024 edition elsewhere
  (`rust-toolchain.toml`, and `rust-skills`' own coverage is current for
  1.96 / 2024). Update the generated edition before using any scaffold this
  skill produces.
- **Dependency footprint.** The generated `Cargo.toml` template pulls in
  `tokio`, `axum`, `schemars`, `async-trait`, and others by default. The
  estate's `just check` runs `cargo machete`, which will flag any of these
  left unused after pruning — expected, and the pruning step above is what
  keeps that gate meaningful rather than a chore to silence.
- Never registered with Pi's MCP registry; it is a skill, not a server.
- This skill was installed and documented only; it was not run against this
  codebase as part of this change.
