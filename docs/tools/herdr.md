# Herdr

Herdr is the terminal multiplexer itself — the `herdr` binary and background
server the operator installs and runs. This page documents the tool; the
separately-installed control skill that teaches an agent to drive it from
inside a pane is documented at [`docs/skills/herdr.md`](../skills/herdr.md).

## What it does and why Maestro uses it

Herdr organizes terminals into workspaces, tabs, and panes, and recognizes
coding agents running inside panes, showing each one's lifecycle state
(`idle`, `working`, `blocked`, `done`, `unknown`). A background server owns
the real terminal processes; the sidebar and CLI let a human or an agent
inspect and control them without leaving a calling pane. Maestro uses it as
the operator's terminal environment: the estate's subagent tooling focuses a
Herdr pane when a session already runs inside one, and the Herdr control
skill drives agent-to-agent coordination through it.

## Vocabulary mapping

| Herdr term | What it means | Our term (protocol / estate) |
| --- | --- | --- |
| session | a persistent background server namespace; `herdr` attaches to the default one | terminal server session |
| workspace | the project-level container, one per repo or task | terminal workspace |
| tab | a layout inside a workspace | terminal layout |
| pane | a real terminal, splittable, surviving client detach | terminal pane |
| agent | a coding-agent process Herdr recognizes inside a pane | recognized agent process |
| `HERDR_ENV` | environment marker set inside a Herdr-managed pane | session marker |

## Identity

| Field | Value |
| --- | --- |
| Upstream | `herdrdev/herdr`, `herdr.dev` |
| Distribution | a standalone binary the operator installs via the official installer (or Homebrew/mise/Nix); not an MCP server, and not itself a Pi skill |
| Installed | `herdr` 0.8.2 at `~/.local/bin/herdr` |
| Config/state root | `~/.config/herdr/` — `config.toml`, server and client logs, `session.json` (live workspace/tab/pane topology), `herdr.sock` / `herdr-client.sock`, `plugins/` |
| Environment | `HERDR_ENV=1` is set inside a Herdr-managed pane; this is both the human-visible confirmation and the control skill's own activation guard that the calling process runs inside one |

## CLI surface

`herdr --help` and each bare command group (`herdr agent`, `herdr pane`,
`herdr workspace`, `herdr tab`, and the others) are the authoritative source
for syntax; the control skill instructs an agent to read them directly rather
than work from a memorized copy. `herdr --default-config` prints the full
default configuration; `herdr status` summarizes server and client runtime
state.

## Operating conventions (Pi in Herdr)

Pi is a first-class Herdr agent: the managed extension `herdr-agent-state.ts`
(`HERDR_INTEGRATION_ID=pi`) reports lifecycle state directly, so Herdr does
not fall back to screen-scraping to tell `idle`, `working`, and `blocked`
apart, and it holds a native session reference that lets a Pi conversation
resume after a Herdr server restart rather than only after a client
detach/reattach. Do not edit that managed file; add custom hooks beside it
if needed, since reinstalling or updating the integration overwrites it.

**Topology: one workspace per repo, worktree lanes as grouped children.**
Each repo gets one Herdr workspace. Parallel branch work uses the sidebar's
worktree actions rather than a second checkout managed by hand: `New
worktree` creates the checkout under `[worktrees] directory` (for example
`~/.herdr/worktrees/<repo>/<branch-slug>`) and opens it as a new workspace
grouped under the source workspace. This maps directly onto the estate's
PR-only, topic-branch flow — one lane, one branch, one workspace.

**Two-tier delegation.** Use a Herdr agent pane for a lane that is
supervised or interactive: a long-lived conductor Pi per repo, a reviewer to
be cross-examined, a dev server watched for output. Use pi-subagents for
headless, structured fanout instead, since that work stays auditable in the
estate's spool without adding sidebar rows. Rule of thumb: if it will be
watched or talked to, it is a Herdr pane; if it is fire-and-forget with a
structured result, it is a pi-subagent.

**Sidebar as the triage surface.** `agent_panel_sort = "priority"` puts
lanes needing attention first. A lane may self-report a display-only task
label with `herdr pane report-metadata "$HERDR_PANE_ID" --source
user:pi-title --token task="..." --ttl-ms 3600000`, rendered through the
`$task` sidebar row token. Metadata is presentation only and never overrides
the integration's semantic `idle`/`working`/`blocked` state.

**Durability posture.** Detach and reattach preserve everything for free.
A server restart restores layout, and a Pi pane resumes its conversation
natively through the integration rather than needing a manual re-launch.
Update the server with `--handoff` so a best-effort live handoff keeps
running agents attached through the update. Pane screen history
(`pane_history`) stays off deliberately: it persists pane contents to disk,
a security trade-off not worth taking on panes that display tokens and other
secrets, and native session resume already covers the common restart case.

**Safety.** Background spawns use `--no-focus` so the operator's focus does
not move. Commands target `--current`, an explicit pane ID, or a unique
agent name rather than relying on whichever pane the UI happens to have
focused. IDs are parsed from JSON responses, not guessed from sidebar order.
`herdr server stop` is never run from an active session.

## Notes

- **Integrated with the estate's subagent tooling.** When a session already
  runs inside a Herdr pane, the estate's subagent inspection commands focus a
  Herdr pane instead of opening a separate window.
- **Companion skill, not a duplicate.** [`docs/skills/herdr.md`](../skills/herdr.md)
  documents the control skill: what teaches an agent to operate Herdr from
  inside a pane. This page documents Herdr itself — the binary, its install,
  and its own state. Neither page restates the other.
- **Not a Maestro provider.** No facade wraps it, no MCP registration, and no
  `<workspace>/.maestro/state` entry; its state lives at `~/.config/herdr/`.
- **Safety posture.** The control skill carries the operational rules (do not
  close a pane, tab, workspace, or session the agent did not create; never
  stop the server from an active session) — see that page rather than a
  second copy here.
