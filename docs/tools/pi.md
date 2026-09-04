# Pi

Pi is the coding-agent runtime this estate operates on. The operator runs
`pi`; every provider, skill, and subagent this repository's `docs/providers/`
and `docs/skills/` pages describe loads through it. This page documents Pi's
role in the estate and its state on this machine, not Pi's own feature set,
which lives upstream.

## What it does and why Maestro uses it

Pi is the terminal coding-agent client: it loads skills, wires MCP servers,
and dispatches subagents according to a per-machine configuration. Maestro
does not wrap or replace it — the estate's providers, skills, and tools are
components this runtime loads, and this repository's own `docs/providers/`,
`docs/skills/`, and `docs/tools/` pages describe what is loaded, not how
loading works. That mechanism, and how a machine's Pi configuration is
captured and restored, belongs to `maestro-pi-config`.

No vocabulary mapping table follows: Pi's own terms — agent, skill, provider,
subagent — are already the estate's vocabulary. There is nothing foreign to
translate, unlike a tool such as Herdr with its own workspace/tab/pane model.

## Identity

| Field | Value |
| --- | --- |
| Upstream | `earendil-works/pi` (MIT), published to npm as `@earendil-works/pi-coding-agent` |
| Installed | `0.84.4`, global npm install; the `pi` binary resolves through the active Node installation |
| Config/state root | `~/.pi/agent/` — `settings.json` (default model and provider, subagent model policy, installed packages), `skills/` (this estate's installed skills), `npm/node_modules/` (bundled dependencies for packages and skills) |
| Per-project scratch | `<project>/.pi/` (the package's own `piConfig.configDir`); **not** gitignored by default in this repository — an installer that targets project scope instead of the global skills directory writes here, which is why this estate's skill installs use a direct copy into `~/.pi/agent/skills/` rather than a project-scoped CLI |

## Relationship to the rest of the estate

- **Providers** (`docs/providers/`) are MCP servers wired into
  `~/.config/mcp/mcp.json`, which Pi's MCP client connects to.
- **Skills** (`docs/skills/`) are `SKILL.md` contracts Pi loads from
  `~/.pi/agent/skills/`; `settings.json`'s `packages` list is how git- and
  npm-sourced skills and extensions are declared.
- **Subagents** — including `scout`, `delegate`, `worker`, `researcher`,
  `oracle`, and `reviewer` — each have a model and fallback-model policy
  recorded in `settings.json`'s `subagents.agentOverrides`, not in this
  repository.
- **Configuration capture.** `maestro-pi-config` is the separate repository
  that captures this machine's Pi configuration into git and can restore it
  onto another machine; it defines its own vocabulary (capture, plan, apply,
  provision) for that mechanism, and that vocabulary is not restated here.

## Notes

- Not itself an MCP server or a `SKILL.md`; it is the runtime both load into.
- Full CLI and configuration reference: the upstream repository
  (`earendil-works/pi`) is the primary source; this page does not duplicate
  it.
