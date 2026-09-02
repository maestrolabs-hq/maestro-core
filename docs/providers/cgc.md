# CodeGraphContext (CGC)

CGC maintains the workspace-context code graph — one of two independent
repository graphs (the other is Graphify). Identities, indexes, scores, and
results are never merged.

## What it does and why Maestro uses it

CGC parses source with tree-sitter (26 languages) into an embedded graph
database and answers precise structural questions: who calls this function,
which functions are dead, what is the cyclomatic complexity, what does a raw
graph query return. Maestro uses it as the workspace-context source because
those answers come from the AST itself — deterministic, no model involved —
and keeping it independent from Graphify gives two graph opinions that are
never merged, so each can be trusted or replaced on its own.

## Vocabulary mapping

| CGC term | What it means | Our term (protocol / estate) |
| --- | --- | --- |
| context | one graph database instance | workspace-context source |
| repository (indexed) | a scanned code snapshot in the database | indexed repository |
| index / update / watch | scanning code into the direct provider database | provider-native indexing |
| job | background indexing task | no equivalent — provider-internal |
| bundle | pre-indexed portable graph package | no equivalent — not used |
| Cypher | the graph query language | graph query (planned `maestro graph query`) |
| god nodes / hotspots | most-connected or riskiest entities | architectural report (planned `maestro graph report`) |

## Identity

| Field | Value |
| --- | --- |
| Package | `codegraphcontext` v0.6.8 (uv tool) |
| CLIs | `cgc`, `codegraphcontext` |
| Backend | Embedded KuzuDB (26 tree-sitter languages) |
| Database | `<workspace>/.maestro/state/providers/cgc/kuzudb` |
| Current contents | 40 files, 193 functions, 31 structs, 21 enums, 133 modules (691 graph nodes) |

## Wiring

The MCP server takes no CLI options; database selection uses environment
variables with highest precedence:

```json
"codegraph": {
  "command": "cgc",
  "args": ["mcp", "start"],
  "env": {
    "CGC_RUNTIME_DB_TYPE": "kuzudb",
    "CGC_RUNTIME_DB_PATH": "<workspace>/.maestro/state/providers/cgc/kuzudb"
  }
}
```

Caveats: a long-running client caches the spawned server — restart the client
(Pi `/reload`) after changing this wiring. Tool results arrive wrapped as JSON
text in `result.content[0].text`; decode before reading fields. Tools that re-read source files from disk require the original repository path to remain available.

## CLI surface

| Command | Purpose |
| --- | --- |
| `cgc doctor` | Diagnostics (backend, parsers, permissions) |
| `cgc --db kuzudb --db-path <p> stats` | Node counts by kind |
| `cgc --db kuzudb --db-path <p> list` | Indexed repositories |
| `cgc index / update / clean / delete` | Mutating provider-native operations |
| `cgc report / diagram / visualize` | Reporting and rendering |
| `cgc mcp start` | MCP server (stdio) |

## MCP tools (29)

| Tool | Description | Maestro equivalent | Tested |
| --- | --- | --- | --- |
| `add_code_to_graph` | Performs a one-time scan of a local folder to add its code to the graph. | provider-native direct state; no Maestro facade | skipped (mutating) |
| `check_job_status` | Check the status and progress of a background job. | no facade — provider-internal job control | verified |
| `list_jobs` | List all background jobs and their current status. | no facade — provider-internal job control | verified |
| `find_code` | Find relevant code snippets related to a keyword (e.g., function name, class name, or content). | planned `maestro graph query` (source: workspace-context) | not exercised |
| `analyze_code_relationships` | Analyze code relationships like 'who calls this function' or 'class hierarchy'. | planned `maestro graph query` (source: workspace-context) | not exercised |
| `watch_directory` | Continuously monitors a directory and keeps graph updated. | provider-native direct state; no Maestro facade | skipped (mutating) |
| `execute_cypher_query` | Run a read-only Cypher query against the code graph. | planned `maestro graph query` (source: workspace-context) | not exercised |
| `add_package_to_graph` | Add a package to the graph. | provider-native direct state; no Maestro facade | skipped (mutating) |
| `find_dead_code` | Find potentially unused functions. | planned `maestro graph query` (source: workspace-context) | not exercised |
| `calculate_cyclomatic_complexity` | Calculate complexity of a function. | planned `maestro graph query` (source: workspace-context) | not exercised |
| `find_most_complex_functions` | Find most complex functions. | planned `maestro graph query` (source: workspace-context) | not exercised |
| `list_indexed_repositories` | List all indexed repositories. | planned `maestro graph status` (source: workspace-context) | not exercised |
| `delete_repository` | DESTRUCTIVE AND IRREVERSIBLE. | no facade — destructive direct-state operation | skipped (destructive) |
| `visualize_graph_query` | Generate a Neo4j visualization URL for a Cypher query. | planned `maestro graph query` (source: workspace-context) | not exercised |
| `list_watched_paths` | List all watched directories. | no facade — provider-internal | verified |
| `unwatch_directory` | Stop watching a directory. | provider-native direct state; no Maestro facade | skipped (mutating) |
| `load_bundle` | Load a pre-indexed graph bundle (.cgc) into the database. | provider-native direct state; no Maestro facade | skipped (mutating) |
| `search_registry_bundles` | Search registry bundles. | no facade — provider-internal registry | verified |
| `get_repository_stats` | Get repository statistics. | planned `maestro graph status` (source: workspace-context) | not exercised |
| `discover_codegraph_contexts` | Discover .codegraphcontext folders. | no facade — provider-internal discovery | verified |
| `switch_context` | Switch active graph context. | provider-native direct state; no Maestro facade | skipped (mutating) |
| `list_graphs` | List all available graphs in the FalkorDB instance. | no facade — FalkorDB-specific; inert on KuzuDB | verified (reports unsupported) |
| `generate_report` | Generate codegraph report. | planned `maestro graph report` (source: workspace-context) | not exercised |
| `find_java_spring_endpoints` | Find Spring endpoints. | no facade — Java-stack specific | verified (empty on Rust corpus) |
| `find_java_spring_beans` | Find Spring beans. | no facade — Java-stack specific | fails on non-Java corpus |
| `find_datasource_nodes` | Query datasource nodes. | no facade — Java-stack specific | fails on non-Java corpus |
| `simulate_metrics` | Calculate repository architectural metrics (coupling, cohesion, circular dependencies, complexity, and maintainability). | planned `maestro graph query` (source: workspace-context) | not exercised |
| `simulate_architectural_change` | Simulate architectural modifications (service decomposition, adding/removing dependencies, deleting nodes) and compare metrics against the baseline. | planned `maestro graph query` (source: workspace-context) | not exercised |
| `analyze_architectural_evolution` | Analyze repository growth trend and identify Technical Debt Hotspots (combining code complexity and Git commit churn). | planned `maestro graph query` (source: workspace-context) | not exercised |
