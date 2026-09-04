# Herdr

Herdr is a skill, not an MCP server: it teaches the agent to drive the
separately-installed `herdr` CLI, a terminal multiplexer for coding agents.
The skill only activates when the user explicitly mentions Herdr and the
agent is running inside a Herdr-managed pane.

## What it does and why Maestro uses it

Herdr organizes terminals into workspaces, tabs, and panes, recognizes coding
agents running inside panes, and exposes the current session through the
`herdr` CLI. Maestro uses it to inspect and coordinate neighboring agent and
command panes from inside a Herdr-managed session — starting an agent in a
sibling pane, prompting it, reading its output, and waiting on its lifecycle
state, without leaving the calling terminal. Unlike the dormant skills
installed earlier this week (Rust MCP Server Generator), Herdr is actually
present and running in this environment: the `herdr` binary is on `PATH` at
the same version this skill is pinned to, and `HERDR_ENV=1` is set — so this
skill is live whenever the agent runs inside a Herdr pane, not dormant.

## Vocabulary mapping

| Herdr term | What it means | Our term (protocol / estate) |
| --- | --- | --- |
| workspace / tab / pane | the three-level terminal topology (`w1`, `w1:t1`, `w1:p1`) | terminal layout hierarchy |
| agent lifecycle state | `idle`, `working`, `blocked`, `done`, `unknown` — Herdr's classification of a recognized agent | agent readiness state |
| `--current` | targets the pane the calling process is running in | self-targeting flag |
| `HERDR_ENV` | environment marker confirming the agent runs inside a Herdr-managed pane | skill activation guard |
| `agent start` / `agent prompt` | launches or drives a recognized coding agent in an existing pane | sibling-agent coordination |
| `pane run` / `pane read` | sends a command to an ordinary (non-agent) pane and reads its output | raw terminal control |

## Identity

| Field | Value |
| --- | --- |
| Upstream | `herdrdev/herdr` (skill path `skills/herdr`), Apache License 2.0 |
| Distribution | agent skill — **not** an MCP server; the estate does not wire the `herdr` binary itself, which is installed and managed outside this skill |
| Installed | `~/.pi/agent/skills/herdr`, by direct copy (CLI installers avoided — see Notes) |
| Pinned to | tag `v0.8.2` → commit `34ba52cc6ff3b723e6fc0130485ec24582dbe205` (a lightweight tag resolving directly to the commit) |
| Pin rationale | Unlike Rust Skills' upstream, `herdrdev/herdr` publishes tagged releases, so this pins to the tag itself — the same pin style as Superpowers and Apollo in `maestro-pi-config`, rather than a bare commit SHA |
| Environment | `herdr` binary present on `PATH` at `~/.local/bin/herdr`, reporting `herdr 0.8.2` — the same version this skill is pinned to; `HERDR_ENV=1` is set in this session, so the skill's own activation guard is satisfied here |

## Skill surface

| Component | Purpose |
| --- | --- |
| `SKILL.md` | Single-file contract (no `references/` tree at this pin) covering pane/tab/workspace topology, starting and prompting agents in sibling panes, running ordinary commands, and reading pane output. Teaches the `agent`, `pane`, `workspace`, `tab`, `worktree`, `terminal`, `notification`, `integration`, and `session` command groups; instructs the agent to run each group bare (e.g. `herdr agent`) to read the installed CLI's own authoritative syntax rather than assuming it. |

## Usage

In Pi, after a reload, the skill activates only when the user explicitly
mentions Herdr and the agent is running inside a Herdr-managed pane:

```text
Use Herdr to start a codex agent in a pane to the right and ask it to review the current diff.
```

The skill's own first instruction is to verify `HERDR_ENV=1` before issuing
any control command, and to stop and say so if that check fails.

## Notes

- **Built-in activation guard.** The skill checks `test "${HERDR_ENV:-}" = 1`
  before any control command and instructs the agent to stop if it fails —
  the skill will not attempt to inspect or control a Herdr session from
  outside Herdr.
- **Safety rules the skill carries.** Do not close a workspace, tab, pane, or
  session the agent did not create unless the user explicitly asked. Never
  run `herdr server stop` from an active session unless the user explicitly
  intends to stop the server. Never kill the main Herdr process; use named
  test sessions for experiments that need an isolated server.
- **Advisory, not authoritative.** Where this skill's guidance disagrees with
  `maestro-core`'s own `AGENTS.md` or an enforced gate, the estate's recorded
  decision wins.
- **Install method.** `gh skill` and `npx skills add` were avoided per the
  lesson from the Rust Skills install: both can misplace a skill into project
  scope (writing into the repository's own working tree) rather than the
  global skills directory. `skills/herdr/SKILL.md` was fetched directly at
  the pinned tag and copied into `~/.pi/agent/skills/herdr/`; nothing was
  written under `maestro-core/.pi/`.
- Not registered with Pi's MCP registry; it is a skill, not a server.
- This installation confirmed the `herdr` binary, its version, and
  `HERDR_ENV` in this environment. No Herdr control command (starting a pane,
  reading agent state, and so on) was exercised as part of this change.
