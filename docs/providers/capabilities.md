# Provider capabilities and checked contracts

Inspected 2026-09-05: CGC 0.6.8, CodeGraph 1.6.0, Codebase-Memory 0.10.8,
Graphify 0.9.53, Semantica 0.6.7, MemPalace 3.8.0, cached Docling MCP 3.2.0.
This is an evidence ledger, not a promise that every tool is healthy.
Identities, graphs, scores and results remain separate.

**All live Pi gateway workflows are untested.** The gateway catalog was cached;
no disconnected production server was started. Native stdio, native handlers,
immutable storage inspection and deterministic mocks are different boundaries.
The code-provider native stdio and Graphify/Docling stdio fixtures really ran;
MemPalace used isolated native handlers/storage, not stdio. Semantica native
handler/JSON tests ran with no model loads. Counts and empty responses do not
certify complete workflows or quality. See [validation](validation.md).

E = **verified end-to-end**, only the named synthetic/native scope; S =
**schema/source-only**; F = **failed**; B = **blocked**; N =
**not-applicable**; U = **untested**. `!` means required, `?` optional.
All production MemPalace calls with possible writes/model loads are blocked,
including operations whose native names suggest reads. Source references
`M/` below name the installed mempalace package, never production data.

These tables describe the audited installed runtime. Guarded patch artifacts
now pass selected scratch regressions, but are **not deployed**: CGC job JSON,
Graphify exact-ID/error signaling plus global ownership/community/type safety,
Docling guarded Markdown snapshots, Semantica analytics/serialization/KB/routing,
and adapter branch deadlines. Remaining failures are not erased by those fixes.



## CGC - 29 native tools

| Tool | Input schema surface | Evidence classification / remaining gap |
|---|---|---|
| add_code_to_graph | repo_path; is_dependency?, graph_name? | verified end-to-end new native ingest + exact source; already-indexed no-op; job status failed; production gateway untested |
| check_job_status | job_id | failed: last_update_time serialization after real job |
| list_jobs | no args | failed with populated jobs; empty list alone insufficient |
| find_code | query; fuzzy_search?, edit_distance?, repo_path?, graph_name? | verified exact synthetic symbol/source and deleted-symbol absence; fuzzy/semantic quality untested |
| analyze_code_relationships | query_type, target; context?, depth?, repo_path?, graph_name? | verified find_callees entry->helper; other 14 query_type variants schema/source-only |
| watch_directory | repo_path | verified scratch watch/edit/query; server lifecycle production blocked |
| execute_cypher_query | cypher_query; params?, graph_name? | verified CALLS/IMPORTS/file queries and write refusal; full Cypher dialect not tested |
| add_package_to_graph | package_name, language; is_dependency?, graph_name? | schema/source-only; package resolution/indexing untested, no install |
| find_dead_code | exclude_decorated_with?, repo_path?, graph_name? | schema/source-only: empty result with dynamic/entrypoint caveat; dead-code quality untested |
| calculate_cyclomatic_complexity | function_name; path?, repo_path?, graph_name? | verified straight-line fixture complexity1; branches/languages untested |
| find_most_complex_functions | limit?, repo_path?, graph_name? | verified fixture complexity1 rows; includes module pseudo-functions; ranking quality beyond fixture untested |
| list_indexed_repositories | graph_name? | verified scratch repository identity inventory; production native DB read separately |
| delete_repository | repo_path; graph_name? | verified scratch watched repo removal; external-path orphan defect separately failed |
| visualize_graph_query | cypher_query; graph_name? | schema/source-only: returns Kuzu localhost URL; browser/server untested |
| list_watched_paths | no args | verified empty -> watched path -> empty |
| unwatch_directory | repo_path | verified removes owned scratch watcher |
| load_bundle | bundle_name; clear_existing?, graph_name? | schema/source-only; download/import/destructive replacement untested |
| search_registry_bundles | query?, unique_only? | schema/source-only; external registry not probed |
| get_repository_stats | repo_path?, graph_name? | verified repo-scoped fixture counts only; `path` is not advertised schema although native compatibility alias exists |
| discover_codegraph_contexts | repo_path?, max_depth? | schema/source-only: no_contexts_found on known no-context fixture; real discovery/switch workflow untested |
| switch_context | context_path; save? | schema/source-only; mutating state/persisted selection untested |
| list_graphs | no args | not-applicable on Kuzu: success:true with unsupported message, not a successful multigraph inventory |
| generate_report | output_path?, include_java?, god_node_limit?, complexity_limit?, cross_module_limit? | verified scratch file creation; analytical report quality schema/source-only (suggestions-only report) |
| find_java_spring_endpoints | http_method?, path_pattern?, repo_path?, graph_name? | not-applicable Python fixture; empty response, Java-positive workflow untested |
| find_java_spring_beans | stereotype?, repo_path?, graph_name? | failed binder missing spring_stereotype; Java-positive workflow untested |
| find_datasource_nodes | kind?, name?, include_columns?, graph_name? | failed binder variable d not in scope |
| simulate_metrics | repo_path?, context? | schema/source-only: metrics returned (score84.1) but domain validity untested; inferred labels are not truth |
| simulate_architectural_change | changes; repo_path?, context? | schema/source-only: remove_node helper changed reported score84.1->82.0/nodes14->13; architectural interpretation untested |
| analyze_architectural_evolution | repo_path?, commits?, context? | schema/source-only: empty history/hotspots on unborn repo; historical correctness untested |

`analyze_code_relationships.query_type` enum: `find_callers`, `find_callees`, `find_all_callers`, `find_all_callees`, `find_importers`, `who_modifies`, `class_hierarchy`, `overrides`, `dead_code`, `call_chain`, `module_deps`, `variable_scope`, `find_complexity`, `find_functions_by_argument`, `find_functions_by_decorator`. Architectural changes enum: `decompose`, `remove_dependency`, `add_dependency`, `remove_node`; objects require type and support mapping/source/target/rel_type/node_id.

## CodeGraph - one listed gateway tool

| Tool | Input / route | Classification |
|---|---|---|
| codegraph_explore | query:string; maxFiles?:number default12; projectPath?:string | verified native scratch MCP source query, explicit project, empty and not-indexed guidance; gateway doubled prefix cached-schema-only |
| codegraph_status (unlisted) | projectPath used in probe | verified native raw MCP; unlisted through normal gateway discovery |
| codegraph_search / node / callers / callees / impact / files (unlisted) | native source/CLI equivalents exist | schema/source-only; not runtime tested individually |

No indexing tool was listed. `CODEGRAPH_MCP_TOOLS` exposes narrower read tools; no need to add more tools merely to make a fanout work. Useful CLI capabilities include `index` (explicit rebuild), `affected` (tests affected by files), callers/callees/impact and context; none requires fusing provider outputs. Impact/affected are native analysis capabilities, so blanket “no derived-analysis surface” needs qualifying as the currently listed gateway surface.

## Codebase-Memory - 15 native tools

| Tool | Required / optional schema | Classification |
|---|---|---|
| index_repository | repo_path; mode?, target_projects?, name?, persistence? | verified CLI + native MCP fast ingest/refresh, rename/delete removal; moderate/full semantic paths blocked; persistence/cross-repo-intelligence untested |
| search_graph | project; query?, label?, name_pattern?, qn_pattern?, file_pattern?, relationship?, min_degree?, max_degree?, exclude_entry_points?, include_connected?, semantic_query?:string[], limit?, offset?, format?, fields?, detail? | verified BM25 entry query; other filters/paging/semantic modes schema/source-only/blocked |
| query_graph | query, project; graph?, max_rows? | verified CALLS/IMPORTS + write refusal; graph=missed and full dialect untested |
| trace_path | function_name, project; direction?, depth?, limit?, cursor?, mode?, parameter_name?, edge_types?, risk_labels?, include_tests?, format?, include_evidence? | verified outbound hop+resolver evidence; data_flow/cross_service/paging/test filters untested |
| get_code_snippet | qualified_name, project; include_neighbors? | verified exact named function source |
| get_graph_schema | project | verified emitted fixture labels/properties/edge types; not proof extraction complete |
| get_architecture | project; path?, aspects? | verified fixture main->lib boundary/leaf/entry overview; cycles and other analytical aspects untested; missing-project call failed |
| search_code | pattern, project; file_pattern?, path_filter?, mode?, context?, regex?, debug?, limit? | verified full literal matches on known helper lines; bad query arg correctly failed; other modes untested |
| list_projects | offset?, limit?, include_details?, metadata_only? | verified scratch project names/roots and deletion visibility; paging untested |
| delete_project | project | verified explicit scratch project removed; production deletion prohibited |
| index_status | project; verbose? | verified ready/empty fixture metadata and excluded paths; not source-completeness proof |
| check_index_coverage | project; paths?, scopes?, scope_limit?, scope_offset? | verified paths/scopes plus exclusions/outside_project/not_tracked; paths or scopes required at runtime; paging untested |
| detect_changes | project; scope?, direction?, depth?, limit?, base_branch?, since?, format? | schema/source-only: unborn repo returns untracked set; real committed diff/blast radius untested |
| manage_adr | project; mode?, content? | verified sections/update whole-document semantics; native default get supported by source; automatic repo ADR discovery not promised |
| ingest_traces | traces:[{caller?,callee?,count?}], project | failed promised enrichment: accepted stub, source and runtime agree |

`get_architecture.aspects`: all, overview, structure, dependencies, routes, languages, packages, entry_points, hotspots, boundaries, layers, file_tree, clusters, cycles. `adr` is not an aspect. The default/get mode of manage_adr is the native ADR retrieval path. `query_decisions` is absent, not a sixteenth CBM tool.

## Graphify - 10 native tools and 6 resource aliases

| Tool | Contract | Status and boundary |
| --- | --- | --- |
| `query_graph` | `question! string`, `mode? string(bfs,dfs)`, `depth? integer`, `token_budget? integer`, `context_filter? array[string]`, `project_path? string` | E — Native stdio copy; wrong input error-envelope F |
| `get_node` | `label! string`, `project_path? string` | E — Exact fully qualified source ID, native stdio |
| `get_neighbors` | `label! string`, `relation_filter? string`, `token_budget? integer`, `project_path? string` | E — Exact ID and grounded call, native stdio |
| `get_community` | `community_id! integer`, `token_budget? integer`, `project_path? string` | F — Returns members, but global numeric communities collide |
| `god_nodes` | `top_n? integer`, `project_path? string` | E — Native degree ranking only; design/doc hubs are not runtime architecture |
| `graph_stats` | `project_path? string` | E — Counts/category percentages, native stdio; missing-project envelope F |
| `shortest_path` | `source! string`, `target! string`, `max_hops? integer`, `undirected? boolean`, `project_path? string` | F — Exact-ID one-hop query resolves unrelated node |
| `list_prs` | `base? string`, `repo? string`, `project_path? string` | N — GitHub PR management outside assigned role; no network call |
| `get_pr_impact` | `pr_number! integer`, `repo? string`, `project_path? string` | N — GitHub PR management outside assigned role; no network call |
| `triage_prs` | `base? string`, `repo? string`, `project_path? string` | N — GitHub PR management outside assigned role; no network call |

| Gateway-style name / actual resource URI | Contract | Disposition |
| --- | --- | --- |
| `read_graph_report` / `graphify://report` | Native resource URI only; default graph, no project arguments | F: missing report returns normal text |
| `read_graph_stats` / `graphify://stats` | Same | E: native resource counts |
| `read_god_nodes` / `graphify://god-nodes` | Same | E: degree ranking |
| `read_surprising_connections` / `graphify://surprises` | Same | F for globally meaningful community analysis; native read itself ran |
| `read_confidence_audit` / `graphify://audit` | Same | E: exact category counts; not precision |
| `read_suggested_questions` / `graphify://questions` | Same | F for reliable global community framing; native generated questions ran |

Resource-as-tool alias names are S from estate docs and U at the actual gateway; native resources and tools were independently enumerated.

## Semantica - 15 native tools and 3 resource aliases



| Tool / resource | Native audit classification | Actual exercised scope / gap |
|---|---|---|
| extract_entities | verified end-to-end deterministic regex composition; failed typed-span contract with mock; ML/LLM blocked | Exact schemas read; no native model extraction or extraction accuracy measured. |
| extract_relations | failed | Internal real Relation serialization nulls (mocked extraction); authored parse loses negation; native pattern component can produce founded_by. No real ML/LLM quality test. |
| record_decision | verified end-to-end scratch handler; durability failed/source-only | Two nonempty native decisions recorded and found; no save by this tool. |
| query_decisions | verified end-to-end scratch handler | Structured category returned the two known decisions; production has no decisions. Not a native language-understanding benchmark. |
| find_precedents | verified end-to-end scratch handler | Same-scenario fixture returned two known decisions; source shows word/bigram Jaccard, optional structural contribution, not demonstrated embedding retrieval (`context_graph:5028-5098,4298-4328`). |
| get_causal_chain | failed | Nonempty known CAUSES chain cannot serialize Decision objects; empty graph would hide it. |
| add_entity | failed content/durability expectation | Stored content becomes id rather than supplied label; in-memory only. |
| add_relationship | verified end-to-end native graph insertion; persistence untested separately/source-only | Identical edge dedup verified; implicit missing endpoints are native behavior, not an established bug. Handler always says added. |
| run_reasoning | verified end-to-end scratch handler | facts Component(Aster), rule IF Component(?x) THEN Thing(?x) -> Thing(Aster). This proves forward chaining over supplied facts, not factual validity of extracted graph. |
| get_graph_analytics | failed | Known connected/disconnected/isolated/empty graphs expose counts/mass/errors above. |
| export_graph | verified end-to-end JSON handler + json.loads | Synthetic 6 entities / 6 relationships exported after native decision recording; Turtle/TTL/NT/XML/JSON-LD untested. |
| get_graph_summary | verified end-to-end scratch handler; production source-only | Two-node known graph returned node_count=2, decision_count=0, graph_ready=true. Not a successful-load/quality/freshness certificate. |
| query_graph | verified end-to-end scratch node/neighbors/substring handler | a node detail, one outgoing neighbor, literal substring hit; whole question search quality unsupported. |
| update_node | verified end-to-end scratch handler -> file -> fresh load | status=done persisted. Atomicity/concurrent writers/crash recovery untested. |
| delete_node | verified end-to-end scratch archive -> file -> fresh load | status=archived persisted, node retained and searchable. Hard delete not applicable to this tool. |
| read_graph_summary | verified native resource handler; gateway alias schema/source-only | semantica://graph/summary delegates summary. |
| read_decisions | verified native resource handler; gateway alias schema/source-only | semantica://decisions/list delegates list with limit=50; not all decisions for arbitrary graph size. |
| read_schema_info | verified native resource handler; gateway alias schema/source-only | semantica://schema/info lists version/15 tools/3 resources. 18 gateway surfaces are not 18 native tools. |


## MemPalace - 44 native tools

| Tool | Exact declared arguments | Disposition / native handler source |
| --- | --- | --- |
| `mempalace_status` | none | S — contract inspected; native runtime U, no production call; `M/mcp_server.py:2074` |
| `mempalace_list_wings` | none | S — contract inspected; native runtime U, no production call; `M/mcp_server.py:2217` |
| `mempalace_list_rooms` | `wing? string` | S — contract inspected; native runtime U, no production call; `M/mcp_server.py:2260` |
| `mempalace_get_taxonomy` | none | S — contract inspected; native runtime U, no production call; `M/mcp_server.py:2315` |
| `mempalace_get_aaak_spec` | none | N — dialect specification not a repository-quality operation; schema inspected; `M/mcp_server.py:2508` |
| `mempalace_kg_query` | `entity! string`, `as_of? string`, `direction? string` | E — native isolated handler/storage chain; production B; `M/mcp_server.py:3822` |
| `mempalace_kg_add` | `subject! string`, `predicate! string`, `object! string`, `valid_from? string`, `valid_to? string`, `source_closet? string`, `source_file? string`, `source_drawer_id? string` | E — native isolated handler/storage chain; production B; `M/mcp_server.py:3837` |
| `mempalace_kg_invalidate` | `subject! string`, `predicate! string`, `object! string`, `ended? string` | E — native isolated handler/storage chain; production B; `M/mcp_server.py:3894` |
| `mempalace_kg_supersede` | `subject! string`, `predicate! string`, `old_object! string`, `new_object! string`, `at? string` | E — native isolated handler/storage chain; production B; `M/mcp_server.py:3932` |
| `mempalace_kg_timeline` | `entity? string` | E — native isolated handler/storage chain; production B; `M/mcp_server.py:3983` |
| `mempalace_kg_stats` | none | E — native isolated handler/storage chain; production B; `M/mcp_server.py:3994` |
| `mempalace_traverse` | `start_room! string`, `max_hops? integer` | S — contract inspected; native runtime U, no production call; `M/mcp_server.py:2513` |
| `mempalace_find_tunnels` | `wing_a? string`, `wing_b? string` | S — contract inspected; native runtime U, no production call; `M/mcp_server.py:2527` |
| `mempalace_graph_stats` | none | S — contract inspected; native runtime U, no production call; `M/mcp_server.py:2542` |
| `mempalace_mesh_peers` | none | S — contract inspected; native runtime U, no production call; `M/mcp_server.py:2556` |
| `mempalace_create_tunnel` | `source_wing! string`, `source_room! string`, `target_wing! string`, `target_room! string`, `label? string`, `source_drawer_id? string`, `target_drawer_id? string` | S — contract inspected; native runtime U, no production call; `M/mcp_server.py:2564` |
| `mempalace_list_tunnels` | `wing? string` | S — contract inspected; native runtime U, no production call; `M/mcp_server.py:2602` |
| `mempalace_delete_tunnel` | `tunnel_id! string` | S — contract inspected; native runtime U, no production call; `M/mcp_server.py:2611` |
| `mempalace_list_hallways` | `wing? string` | S — contract inspected; native runtime U, no production call; `M/mcp_server.py:2618` |
| `mempalace_delete_hallway` | `hallway_id! string` | S — contract inspected; native runtime U, no production call; `M/mcp_server.py:2627` |
| `mempalace_follow_tunnels` | `wing! string`, `room! string` | S — contract inspected; native runtime U, no production call; `M/mcp_server.py:2634` |
| `mempalace_search` | `query! string`, `limit? integer`, `wing? string`, `room? string`, `source_file? string`, `since? string`, `before? string`, `max_distance? number`, `context? string` | B — embedding/model or production-write flow not authorized; schema inspected; `M/mcp_server.py:2369` |
| `mempalace_check_duplicate` | `content! string`, `threshold? number` | B — embedding/model or production-write flow not authorized; schema inspected; `M/mcp_server.py:2453` |
| `mempalace_add_drawer` | `wing! string`, `room! string`, `content! string`, `source_file? string`, `added_by? string` | B — embedding/model or production-write flow not authorized; schema inspected; `M/mcp_server.py:2989` |
| `mempalace_checkpoint` | `items! array[object]`, `diary? object`, `dedup_threshold? number`, `added_by? string` | B — embedding/model or production-write flow not authorized; schema inspected; `M/mcp_server.py:4453` |
| `mempalace_delete_drawer` | `drawer_id! string` | S — contract inspected; native runtime U, no production call; `M/mcp_server.py:3127` |
| `mempalace_mine` | `source! string`, `mode? string(projects,convos,extract)`, `wing? string`, `agent? string`, `limit? integer`, `dry_run? boolean`, `extract? string(exchange,general)` | B — embedding/model or production-write flow not authorized; schema inspected; `M/mcp_server.py:3259` |
| `mempalace_delete_by_source` | `source_file! string`, `dry_run? boolean` | S — contract inspected; native runtime U, no production call; `M/mcp_server.py:3450` |
| `mempalace_sync` | `project_dir? string`, `wing? string`, `apply? boolean` | S — contract inspected; native runtime U, no production call; `M/mcp_server.py:3567` |
| `mempalace_get_drawer` | `drawer_id! string` | S — contract inspected; native runtime U, no production call; `M/mcp_server.py:3606` |
| `mempalace_list_drawers` | `wing? string`, `room? string`, `since? string`, `before? string`, `limit? integer`, `offset? integer` | S — contract inspected; native runtime U, no production call; `M/mcp_server.py:3621` |
| `mempalace_update_drawer` | `drawer_id! string`, `content? string`, `wing? string`, `room? string` | B — embedding/model or production-write flow not authorized; schema inspected; `M/mcp_server.py:3711` |
| `mempalace_diary_write` | `agent_name! string`, `entry? string`, `topic? string`, `wing? string`, `content? string` | B — embedding/model or production-write flow not authorized; schema inspected; `M/mcp_server.py:4002` |
| `mempalace_diary_read` | `agent_name! string`, `last_n? integer`, `wing? string` | S — contract inspected; native runtime U, no production call; `M/mcp_server.py:4148` |
| `mempalace_hook_settings` | `silent_save? boolean`, `desktop_toast? boolean` | E — native isolated handler/storage chain; production B; `M/mcp_server.py:4219` |
| `mempalace_memories_filed_away` | none | E — scratch marker deletion proven; unsafe as read-only status, production B; `M/mcp_server.py:4263` |
| `mempalace_reconnect` | none | S — contract inspected; native runtime U, no production call; `M/mcp_server.py:4297` |
| `mempalace_event_append` | `type! string`, `stream! string`, `room! string`, `from_agent! string`, `to_agent? string`, `correlation_id? string`, `branch? string`, `base_commit? string`, `status? string`, `body? string`, `metadata? object`, `artifact_ids? array[string]` | E — native isolated handler/storage chain; production B; `M/mcp_server.py:4551` |
| `mempalace_event_list` | `stream? string`, `room? string`, `type? string`, `to_agent? string`, `from_agent? string`, `correlation_id? string`, `status? string`, `since_event_id? string`, `since_created_at? string`, `limit? integer`, `preview? boolean` | E — native isolated handler/storage chain; production B; `M/mcp_server.py:4611` |
| `mempalace_event_wait` | `stream? string`, `room? string`, `type? string`, `to_agent? string`, `from_agent? string`, `correlation_id? string`, `status? string`, `since_event_id? string`, `since_created_at? string`, `timeout_ms? integer`, `limit? integer` | E — native isolated handler/storage chain; production B; `M/mcp_server.py:4654` |
| `mempalace_event_ack` | `event_id! string`, `from_agent! string`, `status? string`, `body? string` | E — native isolated handler/storage chain; production B; `M/mcp_server.py:4695` |
| `mempalace_artifact_put` | `kind! string`, `content! string`, `created_by! string`, `metadata? object` | E — native isolated handler/storage chain; production B; `M/mcp_server.py:4706` |
| `mempalace_artifact_get` | `artifact_id! string` | E — native isolated handler/storage chain; production B; `M/mcp_server.py:4719` |
| `mempalace_patch_submit` | `content! string`, `from_agent! string`, `stream! string`, `room? string`, `to_agent? string`, `correlation_id? string`, `branch? string`, `base_commit? string`, `body? string`, `metadata? object` | E — native isolated handler/storage chain; production B; `M/mcp_server.py:4730` |

Declared schema nuances: `diary_write` accepts `content` as dispatch alias for `entry`; the handler signature itself requires `entry`. `event_* status` is described as `open, claimed, ready, applied, blocked, failed, superseded` but is not encoded as an enum in the tool schema; native validation still matters. `sync.apply` defaults false and `delete_by_source.dry_run` defaults true, but neither was invoked in production. `kg_supersede.at` uses a shared temporal boundary; `kg_invalidate.ended` date-only differs from an exact UTC instant.

## Docling - 19 installed native tools; one additional guarded scratch tool

| Tool | Exact declared arguments | Disposition |
| --- | --- | --- |
| `is_document_in_local_cache` | `document_key! string` | E — real conversion/key/anchor chain over native stdio |
| `convert_document_into_docling_document` | `source! string` | E — earlier curated fixture only; fanout B, broader conversion requires separate authorization |
| `convert_markdown_snapshot_into_docling_document` (patch only) | `repository_root! string`, `source_path! string`, `source_sha256! string`, `content_base64! string` | E — real scratch MD-only stdio and actual-worker key/search/anchor chain; hash-checked bytes, no source fetch/model loaders; caller-attributed reference, not independent filesystem freshness; deployed U |
| `convert_directory_files_into_docling_document` | `source! string` | F — all-unsupported silently returns [] over native stdio; supported-file function chain E |
| `create_new_docling_document` | `prompt! string` | E — real native isolated authoring/function chain; native stdio call U |
| `export_docling_document_to_markdown` | `document_key! string`, `max_size? integer/null` | E — real conversion/key/anchor chain over native stdio |
| `save_docling_document` | `document_key! string` | E — real native isolated authoring/function chain; native stdio call U |
| `page_thumbnail` | `document_key! string`, `page_no? integer`, `size? integer` | E — synthetic PIL page only, disk isolation proven; real document/PDF image and browser U; falsely readOnlyHint |
| `add_title_to_docling_document` | `document_key! string`, `title! string` | E — real native isolated authoring/function chain; native stdio call U |
| `add_section_heading_to_docling_document` | `document_key! string`, `section_heading! string`, `section_level! integer` | E — real native isolated authoring/function chain; native stdio call U |
| `add_paragraph_to_docling_document` | `document_key! string`, `paragraph! string` | E — real native isolated authoring/function chain; native stdio call U |
| `open_list_in_docling_document` | `document_key! string` | E — real native isolated authoring/function chain; native stdio call U |
| `close_list_in_docling_document` | `document_key! string` | E — real native isolated authoring/function chain; native stdio call U |
| `add_list_items_to_list_in_docling_document` | `document_key! string`, `list_items! array[ListItem]` | E — real native isolated authoring/function chain; native stdio call U |
| `add_table_in_html_format_to_docling_document` | `document_key! string`, `html_table! string`, `table_captions? array/null`, `table_footnotes? array/null` | E — real native isolated authoring/function chain; native stdio call U |
| `get_overview_of_document_anchors` | `document_key! string` | E — real conversion/key/anchor chain over native stdio |
| `search_for_text_in_document_anchors` | `document_key! string`, `text! string` | E — real conversion/key/anchor chain over native stdio |
| `get_text_of_document_item_at_anchor` | `document_key! string`, `document_anchor! string` | E — real conversion/key/anchor chain over native stdio |
| `update_text_of_document_item_at_anchor` | `document_key! string`, `document_anchor! string`, `updated_text! string` | E — real native isolated authoring/function chain; native stdio call U |
| `delete_document_items_at_anchors` | `document_key! string`, `document_anchors! array[string]` | E — real native isolated authoring/function chain; native stdio call U |

Nested exact contracts: `list_items` is an array of objects requiring string `list_item_text` and string `list_marker_text`; `section_level` is integer 1..100; anchors are strings such as `#/texts/1`, not `[anchor:...]` wrappers or bare indices. Broad conversion result is `{from_cache:boolean,document_key:string}`; guarded snapshot additionally returns `{repository_root,source_path,source_sha256}`; overview `{structure:string}`; search `{result:string}`; item lookup `{text:string}`; updates `{document_key:string}`; export `{document_key,markdown}`; save `{md_file,json_file}`. An empty/no-match result is not a parsed document. Thumbnail returns MCP image content, not a graph.

## Fanout and safety boundary

Query/status/analyze juxtapose explicitly scoped operations. Index/refresh
requires provider-specific write permission, canonical preflight and native
exclusions. Record/coordinate/author/maintain tools are targeted operations,
not implicit fanout setup. `memories_filed_away` deletes a checkpoint marker;
CGC reports and Docling thumbnails write files. Tool categories/readOnlyHint
are not permission or proof of no writes.

Use the tracked `provider-fanout/fanout.js` rather than stale copied snippets.
CGC stats uses `repo_path`; CodeGraph uses `projectPath`; Graphify repository
tools use `project_path`; CBM architecture requires `project`; Docling anchor
search uses `text`. Semantica search is contiguous case-insensitive substring
matching over id/content, not embeddings. Graphify resource aliases have no
repository argument: global community-derived results remain degraded.

Before sanitizing, read [preparation](preparation.md). Before claiming ready,
read [validation](validation.md). Installed provider tool count 142 includes nine
resource aliases, not 142 native functions, and does not imply coverage. The new
Docling snapshot tool exists only in guarded scratch copies, not that live count.
