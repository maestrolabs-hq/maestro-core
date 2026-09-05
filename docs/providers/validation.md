# Provider validation

A live exercise of every provider's non-destructive query, analysis, derivation,
and status tools, recorded so "green" means observed, not assumed. Mutating and
destructive tools were not exercised — firing them to validate would corrupt
state; their contracts stay in `capabilities.md`. Verified 2026-09-05.

## Verdict

The query system is live across all seven providers; each returned real answers
from the live-only indexes. Correlations are automatic wherever the provider
supports them: CGC / CodeGraph / Codebase-Memory structural edges,
Codebase-Memory semantic edges, Graphify communities and inferred edges, and
MemPalace hallways and tunnels. Two defects remain, both in Semantica and both
provider-side.

## Per provider

### CGC — OK
Validated: `find_code`, `analyze_code_relationships`, `execute_cypher_query`,
`find_dead_code`, `find_most_complex_functions`, `analyze_architectural_evolution`,
`get_repository_stats`, `list_indexed_repositories`, `list_jobs`,
`list_watched_paths`. `cgc doctor`: 26 parsers, 8/8 probes OK.
- `execute_cypher_query` takes `cypher_query`, not `query`.
- `visualize_graph_query` is unsupported on the KuzuDB backend.
- CLI `stats`/`list` block while the MCP server holds the KuzuDB single-writer
  lock (expected; query via MCP while the server runs).

### CodeGraph — OK
Validated: `codegraph_codegraph_explore`. All eight repos processed.
`maestro-herdr-config` and `maestro-manifests` hold no CodeGraph-indexable
source (docs/config only; the Rust in `maestro-manifests` is designed, not
built), so they report `fileCount 0` / `lastIndexed null` / `state complete` —
"indexed, nothing indexable", not "never indexed".

### Codebase-Memory — OK
Validated: `search_graph`, `query_graph` (Cypher), `search_code`, `trace_path`,
`get_graph_schema`, `get_architecture`, `index_status`, `list_projects`,
`query_decisions`. Auto-derives `SEMANTICALLY_RELATED`, `SIMILAR_TO`,
`FILE_CHANGES_WITH` edges.
- `search_code` takes `pattern`; `trace_path` takes `function_name`;
  `check_index_coverage` requires `paths` or `scopes`; all require `project`.

### Graphify — OK
Validated: `query_graph`, `god_nodes`, `graph_stats`, `read_confidence_audit`,
`read_surprising_connections`, `read_god_nodes`, `read_suggested_questions`,
`get_node`, `shortest_path`.
- `query_graph` takes `question`, not `query`; stats report `links`, not `edges`.
- `get_node` / `shortest_path` need a disambiguating path/id or undirected search
  for common labels. Global graph rebuilt from eight live sources; zero phantom
  symbols.

### Semantica — query OK, two defects
Validated: `query_graph`, `query_decisions`, `find_precedents`,
`get_graph_summary`, `read_graph_summary`, `read_decisions`, `read_schema_info`,
`extract_entities`, `extract_relations`, `run_reasoning`, `get_causal_chain`.
- DEFECT: `get_graph_analytics` crashes ("PageRank calculation failed: 'dict'
  object is not callable") — no centrality/community analytics on 0.6.7.
- LIMITATION: `extract_entities` / `extract_relations` are naive (label content
  `UNKNOWN`, emit generic `related_to`); not useful correlations.
- The graph carries containment edges only (repository to file); no derived
  semantic relations. `run_reasoning` needs `facts` + `rules`; `get_causal_chain`
  needs a `decision_id` (none recorded).

### MemPalace — OK (all memory kept)
Validated: `status`, `search`, `kg_query`, `kg_stats`, `kg_timeline`,
`graph_stats`, `list_wings`, `list_rooms`, `list_tunnels`, `sync`. Auto-builds
within-wing hallways and cross-wing tunnels on `mine`.

### Docling — OK (convert)
Validated: `convert_directory_files_into_docling_document`. Per-anchor read tools
(`get_overview`, `search`, `get_text`, `export`, `thumbnail`) operate on a cached
`document_key` and were not re-exercised; authoring/save/delete are mutating and
were not exercised.

## Not exercised by policy

Every mutating or destructive tool: `delete_*`, `add_*`, `record_decision`,
`update_node`, `checkpoint`, `kg_add`/`kg_supersede`/`kg_invalidate`,
`create_tunnel`/`delete_tunnel`/`delete_hallway`, `event_*`/`artifact_*`/
`patch_submit`, Docling authoring/save/delete, `watch_directory`/
`unwatch_directory`, `switch_context`, `load_bundle`. Their contracts are in
`capabilities.md`.
