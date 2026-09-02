# Graphify

Graphify maintains the repository-graph — one of two independent repository
graphs (the other is CGC). It also performs optional LLM semantic extraction
of documents through local OpenAI-compatible endpoints.

## What it does and why Maestro uses it

Graphify builds a portable whole-repository graph (one JSON file): AST
extraction for code, Cargo introspection for crate dependencies, and — when
asked — LLM semantic extraction that turns prose documents into typed nodes
and relationships with per-edge provenance. Maestro uses it as the
repository-graph source for impact analysis, architectural hubs, and
cross-repository analysis; its graph is a single portable artifact that fits
the direct provider-state model, and it stays independent from CGC so the two
sources can disagree visibly instead of contaminating each other.

## Vocabulary mapping

| Graphify term | What it means | Our term (protocol / estate) |
| --- | --- | --- |
| corpus | the scanned source tree | repository source |
| graphify-out/ | output directory holding graph.json | provider output |
| global graph | the graph file served by MCP | direct served graph |
| community | clustered group of related nodes | module cluster |
| god node | most-connected node | architectural hub |
| EXTRACTED / INFERRED / AMBIGUOUS | edge provenance classes | edge provenance |
| semantic extraction | LLM pass turning documents into nodes/edges | document semantic pass (local models only) |
| PR dashboard (list/triage/impact) | GitHub pull-request tooling | no equivalent — out of Maestro scope |

## Identity

| Field | Value |
| --- | --- |
| Package | `graphifyy` **pinned 0.9.53** with extras `[mcp,openai]` (uv tool) |
| CLIs | `graphify`, `graphify-mcp` |
| Structural graph | `<workspace>/.maestro/state/providers/graphify/graph.json` |
| Structural contents | 429 nodes, 1,133 edges; SHA-256 `a903d1df…5fdca5` |
| Served global graph | `<workspace>/.maestro/state/providers/graphify/global-graph.json` — 494 nodes, 1,132 edges (stub nodes for dangling endpoints; one self-loop omitted); `~/.graphify` is a filesystem alias |
| Semantic model | `qwen38-semantic` router preset (`reasoning-effort = low`) at `http://127.0.0.1:8080/v1` |

## Wiring

```json
"graphify": {
  "command": "graphify-mcp",
  "args": ["global-graph.json"],
  "cwd": "<workspace>/.maestro/state/providers/graphify"
}
```

The global graph is served directly from the configured path. Use Graphify’s native
commands to update it when the repository changes; it is not auto-refreshed. Calling MCP tools with a
`project_path` argument switches to `<project>/graphify-out/graph.json`;
omit it to query the served global graph.

## Skills and Maestro equivalents

Graphify ships an official Pi skill, installed at
`~/.pi/agent/skills/graphify`, matching `graphifyy` 0.9.53. The skill documents
the direct Graphify workflow for building and querying provider graphs. The
installed `maestro-cli` skill is the future stable Maestro graph facade, but
`maestro graph` commands are not implemented yet. The Graphify MCP server and
CLI are provider interfaces; neither is a Pi skill, and `maestro-cli` is the
planned Maestro equivalent.

## CLI surface

| Command | Purpose |
| --- | --- |
| `graphify extract <path>` | Full extraction (AST + optional semantic LLM); used for direct provider extraction |
| `graphify query / path / explain / affected / god-nodes` | Graph queries over a `graph.json` |
| `graphify global add / remove / list` | Maintain the served global graph |
| `graphify diagnose multigraph` | Edge-collapse diagnostics |
| `graphify benchmark / export …` | Reporting and export formats |

## MCP tools (10)

| Tool | Description | Maestro equivalent | Tested |
| --- | --- | --- | --- |
| `query_graph` | Search the knowledge graph using BFS or DFS. | planned `maestro graph query` (source: repository-graph) | not exercised |
| `get_node` | Get full details for a specific node by label or ID. | planned `maestro graph query` (source: repository-graph) | not exercised |
| `get_neighbors` | Get all direct neighbors of a node with edge details. | planned `maestro graph query` (source: repository-graph) | not exercised |
| `get_community` | Get all nodes in a community by community ID. | planned `maestro graph query` (source: repository-graph) | not exercised |
| `god_nodes` | Return the most connected nodes - the core abstractions of the knowledge graph. | planned `maestro graph report` (source: repository-graph) | verified |
| `graph_stats` | Return summary statistics: node count, edge count, communities, confidence breakdown. | planned `maestro graph status` (source: repository-graph) | verified |
| `shortest_path` | Find the shortest path between two concepts in the knowledge graph. | planned `maestro graph query` (source: repository-graph) | not exercised |
| `list_prs` | List open GitHub PRs with CI status, review state, and graph impact (which communities each PR touches, blast radius). | no facade — PR dashboard is out of Maestro scope | not exercised |
| `get_pr_impact` | Get detailed graph impact for a specific PR: which files it changes, which knowledge-graph communities are affected, and how many nodes are touched. | no facade — PR dashboard is out of Maestro scope | not exercised |
| `triage_prs` | Return all actionable open PRs (correct base, not stale) with full graph impact data so you can reason about review priority, merge order, and conflic… | no facade — PR dashboard is out of Maestro scope | not exercised |
