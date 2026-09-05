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
MemPalace hallways and tunnels. Semantica's two apparent defects resolved as
one wiring gap (missing spaCy model, since installed) and one genuine 0.6.7
bug pair in `get_graph_analytics` (two local hotfixes applied;
upstream-reportable).

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

### Semantica — wired and enriched; analytics fixed by two local hotfixes
Validated: `query_graph`, `query_decisions`, `find_precedents`,
`get_graph_summary`, `read_graph_summary`, `read_decisions`, `read_schema_info`,
`extract_entities`, `extract_relations`, `run_reasoning`, `get_causal_chain`,
`get_graph_analytics` (after the hotfixes below).
- WIRING GAP (fixed): `extract_entities` / `extract_relations` returned naive
  output (`UNKNOWN` labels, generic `related_to`) because no spaCy model was
  installed — the extractor silently falls back to a pattern stub. With
  `en_core_web_md` + `en_core_web_sm` installed in the tool venv, extraction is
  verified producing real labels (PERSON/ORG/GPE) and dependency-based
  predicate relations. See `semantica.md`, "Extraction wiring".
- ENRICHED: the graph was rebuilt through the extraction pipeline — 1,329
  nodes (8 repository, 251 file, 1,070 entity) and 4,570 edges (`contains`
  251, `mentions` 2,371, `related_to` 1,903, plus real predicates such as
  `located_in`, `use`, `maintain`, `wire`). Entities are queryable via
  `query_graph` search (for example `entity:NORP:herdr`).
- `get_graph_analytics` took TWO local hotfixes: bug #1,
  `_filter_nodes_by_labels` calls `graph.nodes()` but `ContextGraph.nodes` is
  a dict ("'dict' object is not callable"); bug #2, the MCP handler sorts the
  `{'centrality', 'rankings'}` result wrapper instead of the inner
  `{node: score}` map ("'<' not supported between instances of 'dict' and
  'list'"). With both applied the tool returns centrality end-to-end. Both are
  site-packages patches a reinstall overwrites — upstream-reportable.
- QUALITY: spaCy's newswire NER on markdown mislabels syntax (`Herdr` as
  `NORP`, `##` as `MONEY`) and most relations are generic `related_to`. A
  markdown-strip pre-pass or `method="llm"` through the local llama.cpp
  endpoint would raise precision.
- `run_reasoning` needs `facts` + `rules`; `get_causal_chain` needs a
  `decision_id` (none recorded).

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
