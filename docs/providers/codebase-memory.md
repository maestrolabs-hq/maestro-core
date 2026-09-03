# Codebase-Memory (CBM)

Codebase-Memory maintains one of four independent repository graphs (the
others are CGC, Graphify, and CodeGraph) — built by a single static Go binary
with vendored tree-sitter grammars and served from a local SQLite database.
Identities, indexes, scores, and results are never merged across providers.

## What it does and why Maestro uses it

Codebase-Memory parses source with tree-sitter across 162 languages, adding
Hybrid LSP semantic type resolution for a subset of them (Python, TypeScript
/ JavaScript / JSX / TSX, PHP, C#, Go, C, C++, Java, Kotlin, Rust, Perl), and
stores the result as a SQLite knowledge graph of files, symbols, and edges.
Maestro uses it as one of four structural opinions, independent of CGC's embedded
graph database, Graphify's portable JSON graph, and CodeGraph's SQLite index:
its query surface adds a git-diff-to-blast-radius tool (`detect_changes`), a
read-only Cypher-like query tool, and a persistent Architecture Decision
Record store scoped to the project, none of which the other three provide,
and none of it is merged with theirs.

## Vocabulary mapping

| Codebase-Memory term | What it means | Our term (protocol / estate) |
| --- | --- | --- |
| project | one indexed repository, keyed by its root path | indexed repository |
| index_repository | scanning code into the direct provider database | provider-native indexing |
| blast radius (`detect_changes`) | transitive callers/dependents of a git diff | impact set |
| ADR (`manage_adr`) | a per-project architecture document stored in the graph | provider-native architecture note |
| team-shared graph artifact (`.codebase-memory/graph.db.zst`) | optional compressed snapshot committed to the repo | no equivalent — not used, cache root is configured directly |
| daemon / watcher | shared background process that keeps indexes current | no equivalent — provider-internal |
| Hybrid LSP | language-server-backed type resolution layered on tree-sitter | provider-native type resolution |
| Cypher-like query (`query_graph`) | the graph query language | graph query |
| install / hooks / subagent profiles | writing MCP config and hooks into other agents' own config files | no equivalent — not used, Maestro wires MCP itself |

## Identity

| Field | Value |
| --- | --- |
| Package | `codebase-memory-mcp` **pinned 0.10.8** (single static binary, installed to `~/.local/bin`) |
| CLI | `codebase-memory-mcp` |
| Backend | SQLite knowledge graph, tree-sitter (162 languages) with Hybrid LSP for a subset |
| Cache root | `<workspace>/.maestro/state/providers/codebase-memory/` via `CBM_CACHE_DIR`; no filesystem alias is needed because, unlike CodeGraph, this cache root is configurable directly |
| Current contents | project `home-franc-workspace-MaestroLabs-maestro-core`: 362 nodes, 413 edges; labels `Section` 166, `Variable` 62, `File` 41, `Module` 41, `Function` 20, `Folder` 15, `Class` 14, `Branch`/`Decorator`/`Project` 1 each; top edge types `DEFINES` 303, `CONTAINS_FILE` 41, `CALLS` 17, `USAGE` 15; languages `TOML` 6, `Rust` 5, `YAML` 4 |

## Wiring

```json
"codebase-memory": {
  "command": "codebase-memory-mcp",
  "args": [],
  "env": {
    "CBM_CACHE_DIR": "<workspace>/.maestro/state/providers/codebase-memory"
  }
}
```

No `CBM_ALLOWED_ROOT` is set, so `index_repository` is not confined to a
containment root; the deployment relies on Maestro only ever invoking it
against workspace repositories. The server auto-discovers or creates the
project keyed by the indexed repository's root path; no separate served-graph
configuration exists.

## Skills and Pi integration

No provider-supplied Pi skill was identified in the installed distribution.
No provider-specific Pi extension is installed. Codebase-Memory ships hooks,
agent-config installers, and named subagent profiles for 43 other client
surfaces — none of that is installed here, because Maestro wires MCP itself
and the install ran with `--skip-config`. Codebase-Memory is used through its
native MCP server and native CLI only.

## CLI surface

Every MCP tool is also available as a one-shot `codebase-memory-mcp cli
<tool>` command that starts no standing process.

| Command | Purpose |
| --- | --- |
| `codebase-memory-mcp` | MCP server (stdio) |
| `codebase-memory-mcp cli <tool> [args]` | Run one MCP tool locally, then exit |
| `codebase-memory-mcp install / uninstall` | Write or remove MCP config and hooks in other agents' own config files — not used; Maestro wires MCP itself |
| `codebase-memory-mcp update` | Print the exact command to re-run the install script in place |
| `codebase-memory-mcp config list / get / set / reset` | Runtime settings (`auto_index`, `auto_watch`, `ui_enabled`, `ui_port`, …) |
| `codebase-memory-mcp daemon start / stop` | Manage the shared background daemon |

## MCP tools (15)

| Tool | Description | Tested |
| --- | --- | --- |
| `list_projects` | List indexed projects with pagination. | verified |
| `get_graph_schema` | Node/edge counts, relationship patterns, property definitions per label. | verified |
| `get_architecture` | Codebase overview: languages, packages, routes, hotspots, clusters. | verified |
| `search_graph` | Structured search by label, name pattern, file pattern, or BM25/semantic query. | verified |
| `search_code` | Grep-like text search enriched with the knowledge graph. | verified |
| `index_status` | Indexing status and coverage report for a project. | verified |
| `trace_path` | BFS traversal for callers/callees, data flow, or cross-service hops. | verified |
| `detect_changes` | Map a git diff to its transitive blast radius. | verified |
| `check_index_coverage` | Authoritative coverage metadata for exact paths or path scopes. | verified |
| `query_graph` | Execute a read-only Cypher-like query against the graph. | verified |
| `manage_adr` | Create, update, or list sections of a project's Architecture Decision Record. | verified (`sections` mode only; `update` skipped — mutating) |
| `get_code_snippet` | Read source for a symbol by qualified name. | not exercised |
| `index_repository` | Index a repository into the knowledge graph. | exercised via CLI (step 3 of provisioning), not via MCP |
| `delete_project` | Delete a project from the index. | skipped (destructive) |
| `ingest_traces` | Ingest runtime call traces to enhance the graph. | skipped (mutating) |

## Notes and limitations

`index_repository` accepts a `persistence` flag that writes a compressed team
artifact to `.codebase-memory/graph.db.zst` inside the indexed repository for
teammates to bootstrap from; this deployment never sets it, indexing writes
only to the configured `CBM_CACHE_DIR`, and no `.codebase-memory` directory
was created in `maestro-core`. `CBM_ALLOWED_ROOT` is unset, so no containment
root restricts `index_repository`, per the recorded no-containment decision
for this deployment.
