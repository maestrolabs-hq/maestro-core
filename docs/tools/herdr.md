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

**Explicit defaults we rely on.** Three defaults matter enough to name
rather than leave implicit. `session.resume_agents_on_restore` stays at its
default `true` — it is load-bearing for the durability posture above, not an
incidental setting. `advanced.scrollback_limit_bytes` stays at its default
10 MB per pane; a busy lane's rendered output can exceed that, but Pi's own
session file is unaffected, so this is revisited only if a lane shows
truncated terminal output, not preemptively. Pi's own
`subagents.watchdog.enabled` stays `false` in `~/.pi/agent/settings.json` —
left off deliberately for now, revisited as the concurrent-lane count grows
and a frozen subagent becomes more likely to go unnoticed.

## Installed integrations

The Herdr–Pi surface currently in use, all verified live:

| Integration | Mechanism | What it provides |
| --- | --- | --- |
| Pi lifecycle + session integration | managed Pi extension `herdr-agent-state.ts` (`HERDR_INTEGRATION_ID=pi`), reporting `pane.report_agent` and `pane.report_agent_session` over the Herdr socket | authoritative `idle`/`working`/`blocked` state without screen-scraping, plus a native session reference so a Pi conversation resumes after a Herdr server restart |
| Pi task reporter | user Pi extension `herdr-task-title.ts` beside the managed file (source `user:pi-task`), reporting `pane.report_metadata` tokens | the first line of each submitted prompt becomes a display-only `task` token, rendered by the `$task` sidebar row — the agent panel reads as a task board |
| Annotate plugin | Herdr plugin `plannotator/herdr-annotate` (pinned commit) with actions, popup panes, and a Markdown link handler | annotate terminal selections, review an agent's last message, and send feedback back to the agent — see [`docs/tools/plannotator.md`](./plannotator.md) |
| Herdr Sidebar plugin | Herdr plugin `alexarthurs/herdr-sidebar` (pinned commit `4faeea73`), a VS Code-style dockable pane | file explorer and git source control in one pane — syntax-highlighted previews, diffs, staging, and an experimental inline editor; toggled with `prefix+b` |
| Herdr control skill | Pi skill driving the `herdr` CLI, pinned to the installed binary version | lets a Pi session inspect and control panes, tabs, workspaces, and sibling agents from inside a pane — see [`docs/skills/herdr.md`](../skills/herdr.md) |
| Config surface | `~/.config/herdr/config.toml` | worktree checkout root, priority agent-panel sort, symbol status indicators, terminal-delivered toast notifications for background `blocked`/`done` with sound notifications explicitly disabled, the `$task` sidebar row layout, the annotate and herdr-sidebar plugin keybindings, and agent-cycling keys (`next_agent`/`previous_agent`/`focus_agent`) rebound around the two plugins' key collisions with Herdr's own defaults |

## Potential integrations

Mechanisms Herdr exposes that the estate does not use yet, recorded so a
future decision starts from the full list rather than a shortlist. Ordered
roughly by expected payoff.

1. **Socket event subscription → audit spool bridge.** The socket API
   supports long-lived event subscriptions; a small subscriber could record
   every lane's state transitions and session references into the estate's
   spool, extending "every delegation is recorded and auditable" from
   pi-subagents to Herdr-level lanes. Cost: a daemon to own.
2. **Estate plugin with a "new lane" action.** A `herdr-plugin.toml` action
   that creates the worktree, splits a pane, starts a named Pi, and prompts
   it as one keybound step, replacing the manual five-step flow.
3. **Plugin event hooks.** The same plugin can react to session events such
   as worktree creation to bootstrap a lane automatically; pairs with the
   action above rather than standing alone.
4. **Workspace-level status tokens.** The metadata mechanism already used
   for `$task`, at workspace scope: a periodic reporter could surface
   `governance plan` drift or ahead/behind counts per repo in the Spaces
   sidebar, turning it into an estate health board.
5. **Richer per-lane metadata.** `pane.report_metadata` also carries
   `state_labels` and `display_agent`, so a lane could show
   `working="reviewing PR"` instead of the bare state word; same mechanism
   as the task token, purely presentational.
6. **Tab-bar command widgets.** `tab_bar_right` runs a script on an
   interval; a blocked-lane count or drift flag could stay permanently
   visible. Trivial to add.
7. **Custom command keybindings.** Popup scratch terminal, a lazygit popup
   once lazygit is installed, and `plugin_action` bindings for the estate
   plugin's actions.
8. **Direct attach for remote check-ins.** `herdr agent attach <name>` from
   any terminal, including over SSH, opens one lane without the full UI.
   A habit rather than a setup.
9. **Named test sessions.** Isolated scratch servers for experiments that
   must not touch the live session; already part of the control skill's
   safety rules, unused as a workflow.
10. **Pane screen history — considered and declined.** It would replay pane
    contents after a restart but persists terminal output (including
    secrets) to disk; native Pi session resume already covers the restart
    case, so the trade-off is not worth taking.

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
