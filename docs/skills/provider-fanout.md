# Provider fanout

Provider fanout is a skill, not an MCP server: it teaches the agent to
trigger the same operation — query, index/refresh, or status — across all
seven code-intelligence and memory providers (`docs/providers/`) in
parallel, and return their answers side by side, labeled by provider. It is
the first estate-authored skill: no upstream project defines it, because the
need is specific to running seven independent providers side by side in this
estate.

## What it does and why Maestro uses it

The estate deliberately keeps seven providers — CGC, CodeGraph,
Codebase-Memory, Graphify, Semantica, MemPalace, Docling — as independent
opinions, never merged (`docs/providers/README.md`). Without this skill,
asking the same question of all seven means seven separate manual tool
calls, no parallelism, and remembering which provider takes which argument
name for the same concept. This skill builds one equivalence mapping once
and fans a single request out to every provider's matching call
concurrently, so a caller reads seven labeled answers instead of driving
seven tools by hand.

## Vocabulary mapping

| Fanout term | What it means | Our term (protocol / estate) |
| --- | --- | --- |
| fanout | one request triggering the equivalent call on every provider at once | parallel provider dispatch |
| verb | one of query, index/refresh, status — the operation being fanned out | provider operation |
| row | one provider's result in the returned array: `{ provider, ok, data \| error \| note }` | per-provider result |
| `ok: null` row | a provider for which the verb has no equivalent, or is CLI-only | honest non-coverage, not a dropped provider |
| companion shell block | the CLI commands run alongside the mcpScript for providers with no MCP tool for a verb | CLI-only fanout half |

## Identity

| Field | Value |
| --- | --- |
| Upstream | none — estate-authored, the first estate-native skill |
| Distribution | agent skill — **not** an MCP server; it dispatches to the seven already-wired provider MCP servers documented in `docs/providers/` |
| Installed | `~/.pi/agent/skills/provider-fanout` |
| Mechanism | `mcpScript` — a single script per verb that builds an array of per-provider async calls and awaits them with `Promise.all`, each wrapped in its own `try/catch` |

## Contents

| Component | Purpose |
| --- | --- |
| `SKILL.md` | The equivalence mapping (verified tool names cross-checked against `docs/providers/*.md`) and three ready mcpScript templates: query-all, index-all, status-all. Index-all pairs the mcpScript with a companion shell block for the three CLI-only providers (CodeGraph, Graphify, Semantica); status-all pairs it with a companion shell block for CodeGraph. |

## Activation

The skill activates on phrasing such as "query all providers", "fanout a
question", "index everything", "reindex all providers", "provider status",
"compare graph opinions", or "/fanout". Before first use in a session, the
seven provider MCP servers must be connected
(`mcp({ connect: "<provider>" })` for each); a not-yet-connected server fails
its own branch of the fanout cleanly rather than blocking the other six.

## Notes

- **Never merges provider identities.** Every page in `docs/providers/`
  states this as an estate rule — CGC, CodeGraph, Codebase-Memory, Graphify,
  and Semantica each hold an independent graph, and this skill juxtaposes
  their labeled answers rather than fusing them into one result.
- **CodeGraph, Graphify, and Semantica indexing is CLI-only, not a
  workaround.** CodeGraph's MCP server exposes no indexing tool at all — a
  background watcher keeps its index current once `codegraph init` has run
  once. Graphify's MCP server is read-only by design; building or updating
  its graph is a CLI-only operation (`graphify update <path>`, adding `--force`
  after refactors that delete code). Semantica's wired `semantica-mcp` server is
  likewise read/query-only — repository ingestion is the native `semantica
  ingest --type repo` command, not an MCP tool. All three facts come from the
  provider docs, not from a limitation this skill introduces.
- **Docling reports real `n/a` semantics.** Docling holds no repository
  index at all; its cache and status calls are always scoped to one
  already-converted document, never global.
- A provider failing, timing out, or being disconnected never sinks the
  whole fanout — every template wraps each provider call in its own
  `try/catch` inside the shared `Promise.all`.
- Not registered with Pi's MCP registry; it is a skill, not a server.
