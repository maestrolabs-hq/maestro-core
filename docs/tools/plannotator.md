# Plannotator

Plannotator is the human review surface for agent work: plans, documents, and
code diffs open in a local browser (or in the terminal through Herdr), get
annotated, and the feedback returns to the agent as its next message.

## What it does and why Maestro uses it

Plannotator plugs into coding agents through hooks and commands. When an agent
proposes a plan or finishes writing code, the work opens for markup; comments
and annotations are sent back to the agent to act on. The estate uses it for
human-in-the-loop review — the one link in the chain that must stay outside
every facade, because it is where a person overrides the machine. It is an
operator tool, not a Maestro provider: no facade wraps it and none is planned.

## Vocabulary mapping

| Plannotator term | What it means | Our term (protocol / estate) |
| --- | --- | --- |
| review | annotate a git diff or PR in the browser | code review |
| annotate | mark up a document, URL, or folder | document review |
| annotate-last / last | reopen the most recent agent reply or review | review recall |
| guide | a saved, exportable, shareable review walkthrough | review artifact |
| gate (`--gate`) | block until the human approves | human approval gate |
| data directory | local store of plans, annotations, drafts, preferences | operator state (outside `.maestro`) |
| Herdr Annotate | the same review flow inside the Herdr terminal | terminal review surface |

## Identity

| Field | Value |
| --- | --- |
| Upstream | `backnotprop/plannotator` (TypeScript, plannotator.ai) |
| Binary | `plannotator` 0.27.11 at `~/.local/bin/plannotator`, installed by the official `install.sh` (checksum-verified from GitHub Releases) |
| Pi integration | `npm:@plannotator/pi-extension` declared in `~/.pi/agent/settings.json` |
| Herdr integration | `annotate` plugin, `github:plannotator/herdr-annotate` pinned at commit `bccf884b…`, config `~/.config/herdr/plugins/config/annotate` |
| Release verification | per-target `.sha256` plus `gh attestation verify` (SLSA provenance and CycloneDX SBOM predicates) |
| Governance | not yet recorded in `maestro-pi-config` (the provision manifest has a `pi` kind for the extension; pending decision) |

## CLI surface

| Command | Purpose |
| --- | --- |
| `plannotator review [--git \| --gitbutler] [PR_URL]` | Review a diff or pull request in the browser |
| `plannotator annotate <file \| url \| folder>` | Annotate documents; `--gate` blocks until approval, `--hook`/`--json` for agent integration |
| `plannotator annotate-last` / `last` | Reopen the latest agent reply or review |
| `plannotator guide list \| export \| share \| unshare` | Manage saved review guides |
| `plannotator sessions` / `archive` | Session history and archival |
| `plannotator setup-goal` | Structured goal interview |
| `plannotator uninstall [--purge]` | Clean removal |
| bare `plannotator` | Hook mode — expects JSON on stdin (used by agent integrations) |

## Agent commands (through the Pi extension)

| Command | Effect |
| --- | --- |
| `$plannotator-review` | Open the current plan or diff for review |
| `$plannotator-annotate <file\|url\|folder>` | Open a document for annotation |
| `$plannotator-last` | Reopen the most recent review |

## Usage

- In Pi (after a reload): propose a plan, run `$plannotator-review`, mark it up
  in the browser, send; the feedback arrives as the agent's next message.
- In Herdr: annotate terminal text, whole Markdown documents, or agent replies
  with the `annotate` plugin; annotations share the same data directory as the
  browser app, so both surfaces compound.
- Update by rerunning the official installer; Pi updates directly with
  `pi install npm:@plannotator/pi-extension`.

## Notes

- Everything is local: the browser UI and the data directory live on this
  machine; `guide share` is the only explicitly opt-in publishing path.
- Not a Maestro provider: no facade, no MCP registration, no `.maestro` state.
  It sits at the boundary where the human reviews what the agents produced.
