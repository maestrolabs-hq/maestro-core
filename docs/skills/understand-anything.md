# Understand Anything

Understand Anything is a pack of slash-command skills, not an MCP server: its
multi-agent pipeline runs on the invoking agent's own LLM, reading the
repository and writing an LLM-generated knowledge graph plus guided tours to
a project-local data directory.

## What it does and why Maestro uses it

`/understand` orchestrates a chain of sub-agents that scan a codebase,
extract files, functions, classes, and dependencies, identify architectural
layers, and generate guided learning tours, saving the result as a
project-local JSON knowledge graph with a local interactive dashboard.
Companion skills extend this to git diffs, onboarding guides, deep-dive
explanations, business-domain flows, and even non-code sources (Figma files,
LLM wiki knowledge bases). This is explicitly not a fifth structural
repository graph alongside CGC, Graphify, CodeGraph, and Codebase-Memory:
those four are deterministic parser output over source text, verifiable and
reproducible byte-for-byte; Understand Anything's graph is LLM-generated
prose and structure, produced by the agent's own reasoning rather than a
fixed extractor. Maestro records it here as a distinct capability — a
teaching-oriented, human-explorable graph with tours and a dashboard — not as
a fifth entry in the structural-graph set.

## Vocabulary mapping

| Understand Anything term | What it means | Our term (protocol / estate) |
| --- | --- | --- |
| `/understand` | full-codebase analysis pipeline invoked as a skill | LLM-generated repository graph (agent-run, not provider-run) |
| tour | a generated guided walkthrough of the graph | no equivalent — not used |
| domain (`/understand-domain`) | extracted business flows and process steps | no equivalent — not used |
| knowledge graph node summary | plain-English description attached to a graph node | LLM-authored node summary |
| `.ua/` (legacy `.understand-anything/`) | project-local data directory holding the graph and config | skill output directory (project-local, not `.maestro/state`) |
| dashboard / viewer | local web UI rendering the committed graph read-only | no equivalent — not used |
| incremental update | re-analyzing only files changed since the last run | no equivalent — not used |
| multi-agent pipeline | the chain of sub-agents `/understand` invokes | agent-run analysis pipeline |

## Identity

| Field | Value |
| --- | --- |
| Upstream | `Egonex-AI/Understand-Anything` (MIT) |
| Distribution | agent skill pack — **not** a PATH executable and **not** an MCP server |
| Installed | `~/.agents/skills/{understand,understand-chat,understand-dashboard,understand-diff,understand-domain,understand-explain,understand-figma,understand-knowledge,understand-onboard}` (symlinks) via `curl -fsSL https://raw.githubusercontent.com/Egonex-AI/Understand-Anything/main/install.sh \| bash -s pi` |
| Clone | `~/.understand-anything/repo`, commit `ba450c4` (2026-08-26); plugin manifest reports version `2.9.4` |
| Reviewed pin | **none.** Unlike Archify, there is no reviewed-pin entry in `maestro-pi-config`; `install.sh --update` runs `git pull --ff-only` against upstream `main`, so an update pulls whatever `main` holds at that moment |

## Skill surface

| Skill (invoked as `/understand-*`) | Purpose |
| --- | --- |
| `understand` | Full-codebase analysis pipeline; writes `knowledge-graph.json` |
| `understand-chat` | Ask questions about a codebase using its knowledge graph |
| `understand-dashboard` | Launch the local interactive web dashboard |
| `understand-diff` | Analyze a git diff or pull request: what changed, affected components, risks |
| `understand-domain` | Extract business-domain flows into an interactive domain graph |
| `understand-explain` | Deep-dive explanation of a specific file, function, or module |
| `understand-figma` | Analyze a Figma file into a design knowledge graph |
| `understand-knowledge` | Analyze a Karpathy-pattern LLM wiki knowledge base into a knowledge graph |
| `understand-onboard` | Generate an onboarding guide for new team members |

`/understand`'s pipeline runs these sub-agents in sequence: `project-scanner`,
`file-analyzer`, `architecture-analyzer`, `tour-builder`, `graph-reviewer`,
and `assemble-reviewer`; `/understand-domain` adds `domain-analyzer`,
`/understand-knowledge` adds `article-analyzer`, and a `design-analyzer` and
`knowledge-graph-guide` agent back the Figma and chat skills respectively.

## Usage

In Pi, after a reload:

```text
/understand
```

Writes `knowledge-graph.json` (and supporting files) to the project's data
directory: `.ua/` for a fresh project, or the legacy `.understand-anything/`
if that directory already exists. Once a graph exists and is committed,
anyone with Node.js >= 18 can view it without the skill or an LLM:

```shell
npx https://github.com/Egonex-AI/Understand-Anything/releases/latest/download/understand-anything-viewer.tgz /path/to/analyzed/project
```

## Notes

- Understand Anything is never registered with any MCP registry; it is a
  skill pack, not a server, and holds no state under
  `<workspace>/.maestro/state/providers/`.
- The initial `/understand` run on a large repository can consume a
  significant number of tokens; subsequent runs are incremental (only
  changed files re-analyzed).
- Running `/understand` writes `.ua/` into the analyzed repository. A
  repository that runs it must then decide whether to commit or ignore that
  directory. `maestro-core` has not run the pipeline, so nothing was added to
  `.gitignore` here.
- The pipeline was not exercised as part of this installation — no
  `knowledge-graph.json` was generated and no output is reported here.
- No reviewed pin exists for this skill; `install.sh --update` tracks
  upstream `main` directly.
