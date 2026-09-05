# Provider capabilities matrix

Every MCP tool each of the seven providers exposes, classified by category and
by whether it is a fanout verb (the same operation called against every
provider and juxtaposed by the provider-fanout skill) or a single-provider
operation used directly against one provider. Counts are a reading taken from
the live MCP gateway; a provider adding a tool later is expected drift, not an
error in this page.

**Legend.** **query** (ask or read the graph), **index** (build or refresh the
graph), **status** (health or coverage), **validate** (visual or report
inspection), **analyze** (derived architectural analysis) are the five FANOUT
verbs the provider-fanout skill covers. **record** (durable knowledge write),
**coordinate** (multi-agent), **author** (document construction), and
**maintain** (prune, config, delete, export) are SINGLE-PROVIDER operations:
used directly against one provider, never fanned out. **out-of-scope** marks
tools that are irrelevant or unsupported on this estate. As on every page in
`docs/providers/`, results and identities are never merged across providers.

## CGC — 29 tools

| Tool | Purpose | Category | Fanout? |
| --- | --- | --- | --- |
| `add_code_to_graph` | one-time scan to add a folder's code to the graph (CLI-only on this deployment; MCP call fails opaquely for new repos) | index | index |
| `check_job_status` | status/progress of a background job | status | status |
| `list_jobs` | list background jobs | status | status |
| `find_code` | find code snippets by keyword | query | query |
| `analyze_code_relationships` | who-calls-this / class hierarchy | analyze | analyze |
| `watch_directory` | continuously watch a dir and keep the graph updated | index | index |
| `execute_cypher_query` | read-only Cypher over the code graph | query | query |
| `add_package_to_graph` | add a package to the graph | index | single-provider |
| `find_dead_code` | potentially unused functions | analyze | analyze |
| `calculate_cyclomatic_complexity` | complexity of a function | analyze | analyze |
| `find_most_complex_functions` | most complex functions | analyze | analyze |
| `list_indexed_repositories` | list indexed repositories | status | status |
| `delete_repository` | DESTRUCTIVE: permanently delete a repo from the graph | maintain | single-provider |
| `visualize_graph_query` | Neo4j visualization URL for a Cypher query (reports unsupported on this KuzuDB deployment) | validate | out-of-scope |
| `list_watched_paths` | list watched directories | status | status |
| `unwatch_directory` | stop watching a directory | maintain | single-provider |
| `load_bundle` | load a pre-indexed `.cgc` bundle | index | single-provider |
| `search_registry_bundles` | search registry bundles | query | single-provider |
| `get_repository_stats` | repository statistics | status | status |
| `discover_codegraph_contexts` | discover `.codegraphcontext` folders | status | status |
| `switch_context` | switch active graph context | maintain | single-provider |
| `list_graphs` | list graphs (reports unsupported on this backend) | status | status |
| `generate_report` | generate a codegraph report | analyze | analyze |
| `find_java_spring_endpoints` | find Spring endpoints | query | out-of-scope |
| `find_java_spring_beans` | find Spring beans | query | out-of-scope |
| `find_datasource_nodes` | query datasource nodes | query | out-of-scope |
| `simulate_metrics` | architectural metrics (coupling, cohesion, cycles, complexity, maintainability) | analyze | analyze |
| `simulate_architectural_change` | simulate service decomposition / dependency edits and compare | analyze | analyze |
| `analyze_architectural_evolution` | growth trend and technical-debt hotspots from complexity + churn | analyze | analyze |

## CodeGraph — 1 tool

| Tool | Purpose | Category | Fanout? |
| --- | --- | --- | --- |
| `codegraph_explore` | primary tool: answers almost any question over the indexed SQLite knowledge graph | query | query |

## Codebase-Memory — 15 tools

| Tool | Purpose | Category | Fanout? |
| --- | --- | --- | --- |
| `index_repository` | index a repository into the knowledge graph | index | index |
| `search_graph` | search the graph for functions/classes/symbols | query | query |
| `query_graph` | execute a Cypher query over the graph | query | query |
| `trace_path` | trace calls/callers/data-flow/cross-service paths | analyze | analyze |
| `get_code_snippet` | read source for a function/class/symbol | query | query |
| `get_graph_schema` | node/edge schema of the graph | status | status |
| `get_architecture` | high-level architecture overview | analyze | analyze |
| `search_code` | graph-augmented literal code search | query | query |
| `list_projects` | list indexed projects | status | status |
| `delete_project` | delete a project from the index | maintain | single-provider |
| `index_status` | node/edge counts and indexing status | status | status |
| `check_index_coverage` | authoritative indexing-coverage metadata | status | status |
| `detect_changes` | map a git diff to its blast radius | analyze | analyze |
| `manage_adr` | create/update Architecture Decision Records | record | single-provider |
| `ingest_traces` | ingest runtime traces to enrich the graph | index | single-provider |

## Graphify — 16 tools

| Tool | Purpose | Category | Fanout? |
| --- | --- | --- | --- |
| `query_graph` | search the graph via BFS/DFS | query | query |
| `get_node` | full details for a node by label/id | query | query |
| `get_neighbors` | direct neighbors of a node with edge info | query | query |
| `get_community` | all nodes in a community by id | query | query |
| `god_nodes` | most-connected core nodes | analyze | analyze |
| `graph_stats` | node/edge/community summary stats | status | status |
| `shortest_path` | shortest path between two concepts | analyze | analyze |
| `list_prs` | list open GitHub PRs with CI/review status | out-of-scope | out-of-scope |
| `get_pr_impact` | graph impact for a specific PR | out-of-scope | out-of-scope |
| `triage_prs` | actionable open PRs | out-of-scope | out-of-scope |
| `read_graph_report` | full GRAPH_REPORT.md | validate | analyze |
| `read_graph_stats` | node/edge/community counts and confidence | status | status |
| `read_god_nodes` | top 10 most-connected nodes | analyze | analyze |
| `read_surprising_connections` | cross-community surprising connections | analyze | analyze |
| `read_confidence_audit` | EXTRACTED/INFERRED/AMBIGUOUS edge breakdown | status | status |
| `read_suggested_questions` | suggested questions for this codebase | query | query |

## Semantica — 18 tools

| Tool | Purpose | Category | Fanout? |
| --- | --- | --- | --- |
| `extract_entities` | extract named entities from text | author | single-provider |
| `extract_relations` | extract (subject, predicate, object) relations from text | author | single-provider |
| `record_decision` | record a decision into the graph | record | single-provider |
| `query_decisions` | query recorded decisions by natural language | query | query |
| `find_precedents` | find past decisions similar to a scenario | analyze | analyze |
| `get_causal_chain` | trace causal chain upstream/downstream | analyze | analyze |
| `add_entity` | add a node/entity | record | single-provider |
| `add_relationship` | add a directed edge | record | single-provider |
| `run_reasoning` | forward-chaining IF/THEN rules over a node set | analyze | analyze |
| `get_graph_analytics` | PageRank centrality and community detection (broken on 0.6.7: PageRank crash) | analyze | analyze |
| `export_graph` | export the knowledge graph | maintain | single-provider |
| `get_graph_summary` | high-level graph summary | status | status |
| `query_graph` | get a node / traverse neighbours / keyword-search | query | query |
| `update_node` | update node properties | maintain | single-provider |
| `delete_node` | archive (soft-delete) a node | maintain | single-provider |
| `read_graph_summary` | high-level statistics | status | status |
| `read_decisions` | list all recorded decisions | query | query |
| `read_schema_info` | server info and capabilities | status | status |

## MemPalace — 44 tools

| Tool | Purpose | Category | Fanout? |
| --- | --- | --- | --- |
| `mempalace_status` | palace overview (drawers, wings, rooms) | status | status |
| `list_wings` | list wings with drawer counts | status | status |
| `list_rooms` | list rooms within a wing | status | status |
| `get_taxonomy` | full wing->room->drawer taxonomy | status | status |
| `get_aaak_spec` | the AAAK dialect specification | out-of-scope | out-of-scope |
| `kg_query` | query an entity's knowledge-graph facts | query | query |
| `kg_add` | add a fact (subject->predicate->object) | record | single-provider |
| `kg_invalidate` | mark a fact no longer true | record | single-provider |
| `kg_supersede` | atomically replace a fact with its successor | record | single-provider |
| `kg_timeline` | chronological timeline of facts | analyze | analyze |
| `kg_stats` | knowledge-graph overview (entities, triples) | status | status |
| `traverse` | walk the palace graph from a room | analyze | analyze |
| `find_tunnels` | rooms bridging two wings | analyze | analyze |
| `graph_stats` | palace graph overview (rooms, tunnels) | status | status |
| `mesh_peers` | mesh estate snapshot of replicas | coordinate | single-provider |
| `create_tunnel` | create a cross-wing tunnel | maintain | single-provider |
| `list_tunnels` | list explicit cross-wing tunnels | status | status |
| `delete_tunnel` | delete a tunnel by id | maintain | single-provider |
| `list_hallways` | list within-wing hallway records | status | status |
| `delete_hallway` | delete a hallway record by id | maintain | single-provider |
| `follow_tunnels` | follow tunnels from a room | analyze | analyze |
| `search` | semantic search returning verbatim drawer content | query | query |
| `check_duplicate` | check if content already exists | maintain | single-provider |
| `add_drawer` | file verbatim content into the palace | record | single-provider |
| `checkpoint` | save a whole session in one call | record | single-provider |
| `delete_drawer` | delete a drawer by id (irreversible) | maintain | single-provider |
| `mine` | mine a directory into the palace | index | index |
| `delete_by_source` | bulk-delete drawers from one source | maintain | single-provider |
| `sync` | prune drawers whose sources are gitignored/removed | maintain | single-provider |
| `get_drawer` | fetch a single drawer by id | query | query |
| `list_drawers` | list drawers with pagination | status | query |
| `update_drawer` | update a drawer's content/metadata | maintain | single-provider |
| `diary_write` | write to the agent diary in AAAK | record | single-provider |
| `diary_read` | read recent diary entries | query | single-provider |
| `hook_settings` | get/set hook behavior | maintain | single-provider |
| `memories_filed_away` | check if a recent checkpoint was saved | status | status |
| `reconnect` | force reconnect to the palace database | maintain | single-provider |
| `event_append` | append an immutable coordination event | coordinate | single-provider |
| `event_list` | list coordination events | coordinate | single-provider |
| `event_wait` | block until a matching event exists | coordinate | single-provider |
| `event_ack` | acknowledge a coordination event | coordinate | single-provider |
| `artifact_put` | store exact artifact content | coordinate | single-provider |
| `artifact_get` | fetch a coordination artifact by id | coordinate | single-provider |
| `patch_submit` | store a patch artifact and append an event | coordinate | single-provider |

## Docling — 19 tools

| Tool | Purpose | Category | Fanout? |
| --- | --- | --- | --- |
| `is_document_in_local_cache` | check if a document is already converted/cached | status | status |
| `convert_document_into_docling_document` | convert one document (URL or file) to a cached DoclingDocument | index | index |
| `convert_directory_files_into_docling_document` | convert all files in a local directory | index | index |
| `create_new_docling_document` | create a new empty document from a prompt string | author | single-provider |
| `export_docling_document_to_markdown` | export a cached document to Markdown | query | single-provider |
| `save_docling_document` | save a cached document to disk as Markdown/JSON | maintain | single-provider |
| `page_thumbnail` | generate a thumbnail image for a page | validate | single-provider |
| `add_title_to_docling_document` | add/update a document title | author | single-provider |
| `add_section_heading_to_docling_document` | add a section heading | author | single-provider |
| `add_paragraph_to_docling_document` | add a paragraph | author | single-provider |
| `open_list_in_docling_document` | open a new list group | author | single-provider |
| `close_list_in_docling_document` | close a list group | author | single-provider |
| `add_list_items_to_list_in_docling_document` | add list items to an open list | author | single-provider |
| `add_table_in_html_format_to_docling_document` | add an HTML-formatted table | author | single-provider |
| `get_overview_of_document_anchors` | structured overview of a document's anchors | query | query |
| `search_for_text_in_document_anchors` | search text within one cached document | query | query |
| `get_text_of_document_item_at_anchor` | read text of a document item at an anchor | query | query |
| `update_text_of_document_item_at_anchor` | update text at an anchor | author | single-provider |
| `delete_document_items_at_anchors` | delete document items by anchor | author | single-provider |

## Fanout coverage

The provider-fanout skill's templates cover the five fanout verbs (query,
index, status, validate, analyze); single-provider and out-of-scope tools are
listed here for completeness and are used directly against the provider,
never fanned out.

## Indexing hygiene

Every provider must be indexed only from a **live repo root** — a directory
holding `.git`. Never index `<workspace>/.maestro/state/.staging` (built
snapshots), `<workspace>/.superpowers/worktrees` (in-progress worktrees), or the
workspace container directory itself. The container is not a git repository and
the provider ignore files do not list `.maestro`/`.superpowers`, so a
container-level index silently sweeps a built snapshot whose code is not on
`main` — which makes CGC/Semantica/Codebase-Memory confidently describe software
that does not exist. The provider-fanout skill's index template rejects these
paths; keep provider ignore files (`.cgcignore`, `.cbmignore`) carrying
`.maestro/`, `.superpowers/`, `.worktrees/`, `graphify-out/` as a second layer.

## Validation notes (verified 2026-09-05)

A live pass exercised every non-destructive tool; see `validation.md`. Corrections
and defects found:

- CGC `execute_cypher_query` takes `cypher_query` (not `query`).
- Codebase-Memory `search_code` takes `pattern`; `trace_path` takes
  `function_name`; `check_index_coverage` requires `paths` or `scopes`.
- Graphify `query_graph` takes `question`; its stats report `links`, not `edges`.
- Semantica `get_graph_analytics` is broken on 0.6.7 (PageRank crash);
  `extract_entities` / `extract_relations` are naive; the graph is
  containment-only (no derived semantic relations).
