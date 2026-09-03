# MemPalace

MemPalace is the durable memory provider used through native MCP and CLI surfaces.
Its governed native MCP server is also available for provider setup and
diagnostics.

## What it does and why Maestro uses it

MemPalace stores long-term memory locally: it extracts, chunks, deduplicates,
and files content into a searchable store, keeps per-session summaries, and
maintains a temporal knowledge graph whose facts can be added, invalidated,
and superseded over time. Maestro uses it because durable cross-session memory
needs exactly that shape — bounded recall at session start, capture at
lifecycle events, mining of pushed documentation — while the provider remains
replaceable: MemPalace vocabulary never leaks past this documented boundary.

## Vocabulary mapping

| MemPalace term | What it means | Our term (protocol / estate) |
| --- | --- | --- |
| palace | the on-disk memory store | memory store |
| wing | per-repository memory scope | repository identity |
| room / taxonomy | classification bucket inside a wing | category |
| drawer | one stored memory record | memory record |
| filing | writing a record into the store | capture |
| mine | bulk-extract memories from files or conversations | ingest |
| diary | per-session summary entries | session summary (startup recall) |
| temporal KG fact / invalidate / supersede | time-versioned knowledge assertions | temporal facts and versions |
| tunnels / hallways | cross-memory links between records | no equivalent — not exposed |
| mesh / peers | multi-store synchronization | no equivalent — not exposed |

## Identity

| Field | Value |
| --- | --- |
| Package | `mempalace` v3.8.0 (uv tool) |
| CLI | `mempalace` |
| MCP server | `mempalace-mcp` — registered in the governed shared MCP configuration |
| Store | `<workspace>/.maestro/state/providers/mempalace` (native home; `~/.mempalace` is a filesystem alias) |
| Repository identity | `<workspace>/.maestro/state/providers/mempalace/identity.txt` (stable L0 identity per estate repository) |

## Current state

The active store has 2,904 drawers in total, including 1,458 for the `maestro-core` repository.

## CLI surface

| Command | Purpose |
| --- | --- |
| `mempalace mine <path>` | Mine documents/conversations into drawers (`--mode`, `--wing`, `--agent`, `--limit`) |
| `mempalace hooks …` | Lifecycle hook management for agent integration |
| Other commands | See `mempalace --help`; direct provider access remains current |

## Skills and Pi integration

No provider-supplied Pi skill was identified. `mempalace-autosave` and
`mempalace-derive-relations` are installed as temporary Pi extensions, not
skills. The MemPalace MCP and CLI remain the current direct interfaces.

## MCP tools (44)

| Tool | Description | Tested |
| --- | --- | --- |
| `mempalace_status` | Palace overview — total drawers, wing and room counts | verified |
| `mempalace_list_wings` | List all wings with drawer counts | not exercised |
| `mempalace_list_rooms` | List rooms within a wing (or all rooms if no wing given) | not exercised |
| `mempalace_get_taxonomy` | Full taxonomy: wing → room → drawer count | not exercised |
| `mempalace_get_aaak_spec` | Get the AAAK dialect specification — the compressed memory format MemPalace uses. | not exercised |
| `mempalace_kg_query` | Query the knowledge graph for an entity's relationships. | verified |
| `mempalace_kg_add` | Add a fact to the knowledge graph. | skipped (mutating) |
| `mempalace_kg_invalidate` | Mark a fact as no longer true. | skipped (mutating) |
| `mempalace_kg_supersede` | Atomically replace a fact with its successor at a shared boundary. | skipped (mutating) |
| `mempalace_kg_timeline` | Chronological timeline of facts. | verified |
| `mempalace_kg_stats` | Knowledge graph overview: entities, triples, current vs expired facts, relationship types. | verified |
| `mempalace_traverse` | Walk the palace graph from a room. | verified |
| `mempalace_find_tunnels` | Find rooms that bridge two wings — the hallways connecting different domains. | verified |
| `mempalace_graph_stats` | Palace graph overview: total rooms, tunnel connections, edges between wings. | verified |
| `mempalace_mesh_peers` | Mesh estate snapshot (RFC 004): this replica's identity, version vector and node profile; each configured peer's reachability, last sync outcome, remo… | not exercised |
| `mempalace_create_tunnel` | Create a cross-wing tunnel linking two palace locations. | not exercised |
| `mempalace_list_tunnels` | List all explicit cross-wing tunnels. | not exercised |
| `mempalace_delete_tunnel` | Delete an explicit tunnel by its ID. | not exercised |
| `mempalace_list_hallways` | List within-wing hallway records (entity-to-entity co-occurrence links built at mine time). | not exercised |
| `mempalace_delete_hallway` | Delete a hallway record by its ID. | not exercised |
| `mempalace_follow_tunnels` | Follow tunnels from a room to see what it connects to in other wings. | verified |
| `mempalace_search` | Semantic search. | verified |
| `mempalace_check_duplicate` | Check if content already exists in the palace before filing | not exercised |
| `mempalace_add_drawer` | File verbatim content into the palace. | skipped (mutating) |
| `mempalace_checkpoint` | Save a whole session in one call: semantic-dedups each item, files non-duplicates as drawers, then writes one diary entry. | skipped (mutating) |
| `mempalace_delete_drawer` | Delete a drawer by ID. | not exercised |
| `mempalace_mine` | Mine a directory into the palace — the MCP equivalent of `mempalace mine`. | skipped (mutating) |
| `mempalace_delete_by_source` | Bulk-delete every drawer mined from one source_file (exact match). | not exercised |
| `mempalace_sync` | Prune drawers whose source files are gitignored, deleted, or moved. | skipped (mutating) |
| `mempalace_get_drawer` | Fetch a single drawer by ID — returns full content and metadata. | not exercised |
| `mempalace_list_drawers` | List drawers with pagination. | not exercised |
| `mempalace_update_drawer` | Update an existing drawer's content and/or metadata (wing, room). | not exercised |
| `mempalace_diary_write` | Write to your personal agent diary in AAAK format. | skipped (mutating) |
| `mempalace_diary_read` | Read your recent diary entries (in AAAK). | verified |
| `mempalace_hook_settings` | Get or set hook behavior. | not exercised |
| `mempalace_memories_filed_away` | Check if a recent palace checkpoint was saved. | not exercised |
| `mempalace_reconnect` | Force reconnect to the palace database. | not exercised |
| `mempalace_event_append` | Append an immutable agent-coordination event to the logstream (RFC 003). | not exercised |
| `mempalace_event_list` | List agent-coordination events with structured filters, oldest first (append order, not timestamp order). | not exercised |
| `mempalace_event_wait` | Block until a matching coordination event exists or the timeout expires (default 60s, max 5 minutes). | not exercised |
| `mempalace_event_ack` | Acknowledge a coordination event: appends a new event.ack routed back to the original writer with the correlation id copied. | not exercised |
| `mempalace_artifact_put` | Store exact artifact content (unified diff patch, file, log, json, note) for agent handoffs. | not exercised |
| `mempalace_artifact_get` | Fetch a coordination artifact by id — exact content plus sha256 for verification. | not exercised |
| `mempalace_patch_submit` | Convenience: store a patch artifact and append its patch.ready event in one call. | not exercised |

## Notes

- Verification reflects native provider interfaces in this direct MCP phase.
- Tunnels, hallways, mesh, events, artifacts, and patch tools are not represented in the current native-direct documentation boundaries.
