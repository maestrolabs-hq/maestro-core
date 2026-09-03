# CodeGraph

CodeGraph maintains a third independent repository graph — alongside CGC,
Graphify, and Codebase-Memory — built by a native Rust kernel and served from
an in-repo SQLite database. Identities, indexes, scores, and results are
never merged across providers.

## What it does and why Maestro uses it

CodeGraph parses source with tree-sitter grammars compiled into a Rust kernel
(20 languages compiled in, remaining languages and per-file fallbacks handled
by the same extraction logic), storing symbols, edges, and files in a local
SQLite database with FTS5 full-text search. A file watcher keeps the index
current on every save, so a query never triggers its own indexing pass.
Maestro uses it as one of four structural opinions, independent of CGC's
embedded graph database, Graphify's portable JSON graph, and Codebase-Memory's
SQLite index: CodeGraph's single MCP tool returns verbatim source grouped by
file plus the call path and blast radius in one call, which is a different
query shape from the other providers and is not merged with any of them.

## Vocabulary mapping

| CodeGraph term | What it means | Our term (protocol / estate) |
| --- | --- | --- |
| explore | one call returning source + call path + blast radius | structural query |
| blast radius | the set of symbols affected by a change | impact set |
| staleness banner | inline warning that a returned file has pending edits | index-lag warning |
| kernel | the native Rust extraction engine | provider-native extractor |
| auto-sync / watcher | OS-level file watch that keeps the index current | provider-native indexing |
| daemon | background process shared across sessions on one project | no equivalent — provider-internal |
| bundle install (into agents) | writing MCP config into other agents' own config files | no equivalent — not used, Maestro wires MCP itself |
| PR dashboard / list / triage | GitHub pull-request tooling referenced in the CLI help | no equivalent — out of Maestro scope |

## Identity

| Field | Value |
| --- | --- |
| Package | `@colbymchenry/codegraph` **pinned 1.6.0** (npm, global install) |
| CLI | `codegraph` |
| Backend | SQLite + FTS5, native Rust kernel (tree-sitter) |
| Database | `<workspace>/.maestro/state/providers/codegraph/codegraph.db`; the in-repo `.codegraph` directory is a filesystem symlink alias to this path, because the native index location is not configurable |
| Current contents | 9 files, 31 nodes, 47 edges (`file`: 5, `function`: 15, `import`: 8, `variable`: 3); languages `rust`, `yaml` |

## Wiring

```json
"codegraph": {
  "command": "codegraph",
  "args": ["serve", "--mcp"],
  "cwd": "<workspace>/maestro-core"
}
```

The server resolves the project's `.codegraph/` index from `cwd` (or from a
`projectPath` argument passed with a tool call); there is no separate
served-graph configuration. The index is not auto-refreshed by the MCP
server itself — the file watcher that keeps it current is provider-native and
runs independently once the project has been initialized with `codegraph
init`.

## Skills and Pi integration

No provider-supplied Pi skill was identified in the installed distribution.
No provider-specific Pi extension is installed. CodeGraph ships its own agent
guidance — a long usage-instructions string returned in the MCP `initialize`
response, not a Pi skill file — and Claude-Code-specific skill files under its
own repository that are not part of this installation. CodeGraph is used
through its native MCP server and native CLI only.

## CLI surface

| Command | Purpose |
| --- | --- |
| `codegraph init [--yes]` | Build the initial per-project index |
| `codegraph sync` | Sync changes since the last index |
| `codegraph status [--json]` | Index health: file/node/edge counts, pending changes |
| `codegraph query <search>` | Symbol search by name |
| `codegraph explore <query...>` | Same output as the `codegraph_explore` MCP tool |
| `codegraph node / callers / callees / impact / files` | Structural queries; CLI equivalents of the unlisted MCP tools |
| `codegraph serve --mcp` | MCP server (stdio) |
| `codegraph uninit` | Remove the project's `.codegraph/` directory |
| `codegraph install` / `uninstall` | Write or remove MCP config in other agents' own config files — not used; Maestro wires MCP itself |
| `codegraph upgrade [--check]` | Update the CLI |

## MCP tools (8; 1 listed by default)

Only `codegraph_explore` is listed by default — the provider's own guidance
states that one strong tool steers an agent better than a menu of narrower
ones. The other seven stay fully functional but unlisted unless the server is
started with `CODEGRAPH_MCP_TOOLS=explore,node,search,callers,callees,impact,files,status`;
everything they return already arrives inline on `codegraph_explore`. Maestro
does not currently set that environment variable, so only the row below is
reachable through the configured server.

| Tool | Description | Tested |
| --- | --- | --- |
| `codegraph_explore` | Verbatim source of the relevant symbols grouped by file, plus the call path among them and a blast-radius summary, in one call. | verified |
| `codegraph_search` (unlisted by default) | Quick symbol search by name; locations only, no code. | not exercised |
| `codegraph_node` (unlisted by default) | Read a file with line numbers, or one symbol's source plus caller/callee trail. | not exercised |
| `codegraph_callers` (unlisted by default) | List functions that call a given symbol. | not exercised |
| `codegraph_callees` (unlisted by default) | List functions that a given symbol calls. | not exercised |
| `codegraph_impact` (unlisted by default) | List symbols affected by changing a given symbol. | not exercised |
| `codegraph_files` (unlisted by default) | Indexed file tree with language and symbol counts. | not exercised |
| `codegraph_status` (unlisted by default) | Index health check: file/node/edge counts. | not exercised |

## Notes and limitations

The index lags file writes by roughly one second through the watcher; a tool
response can carry a staleness banner naming files edited since the last
sync. This deployment is a single small repository (5 Rust source files plus
CI/config files), so the measured index above is proportionate to that
corpus, not a partial scan.
