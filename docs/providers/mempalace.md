# MemPalace

MemPalace is the durable memory provider behind the `maestro memory` facade.
Maestro acknowledges writes only after SQLite commit, then the Supervisor
delivers to MemPalace asynchronously. Its governed native MCP server is also
available for provider setup and diagnostics.

## What it does and why Maestro uses it

MemPalace stores long-term memory locally: it extracts, chunks, deduplicates,
and files content into a searchable store, keeps per-session summaries, and
maintains a temporal knowledge graph whose facts can be added, invalidated,
and superseded over time. Maestro uses it because durable cross-session memory
needs exactly that shape — bounded recall at session start, capture at
lifecycle events, mining of pushed documentation — while the provider remains
replaceable: the memory protocol v1 envelope is the stable contract, and
MemPalace vocabulary never leaks past its adapter.

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
| MCP server | `mempalace-mcp` — registered in the governed Pi MCP configuration; Maestro remains the stable facade |
| Store | `<workspace>/.maestro/state/providers/mempalace` (native home; `~/.mempalace` is a filesystem alias) |
| Repository identity | `<workspace>/.maestro/state/providers/mempalace/identity.txt` (stable L0 identity per estate repository) |

## Current state

The active store has 2,904 drawers in total, including 1,458 for the `maestro-core` repository.

## CLI surface

| Command | Purpose |
| --- | --- |
| `mempalace mine <path>` | Mine documents/conversations into drawers (`--mode`, `--wing`, `--agent`, `--limit`) |
| `mempalace hooks …` | Lifecycle hook management for agent integration |
| Other commands | See `mempalace --help`; unused by Maestro, which speaks MCP to the provider |

## MCP tools (44)

| Tool | Description | Maestro equivalent | Tested |
| --- | --- | --- | --- |
| `mempalace_status` | Palace overview — total drawers, wing and room counts | `maestro memory status` — available | verified |
| `mempalace_list_wings` | List all wings with drawer counts | no facade — provider-internal | not exercised |
| `mempalace_list_rooms` | List rooms within a wing (or all rooms if no wing given) | no facade — provider-internal | not exercised |
| `mempalace_get_taxonomy` | Full taxonomy: wing → room → drawer count | no facade — provider-internal | not exercised |
| `mempalace_get_aaak_spec` | Get the AAAK dialect specification — the compressed memory format MemPalace uses. | no facade — provider-internal | not exercised |
| `mempalace_kg_query` | Query the knowledge graph for an entity's relationships. | memory protocol v1 temporal graph query — available | verified |
| `mempalace_kg_add` | Add a fact to the knowledge graph. | memory protocol v1 graph mutation — available | skipped (mutating) |
| `mempalace_kg_invalidate` | Mark a fact as no longer true. | memory protocol v1 graph mutation — available | skipped (mutating) |
| `mempalace_kg_supersede` | Atomically replace a fact with its successor at a shared boundary. | memory protocol v1 graph mutation — available | skipped (mutating) |
| `mempalace_kg_timeline` | Chronological timeline of facts. | memory protocol v1 temporal graph timeline — available | verified |
| `mempalace_kg_stats` | Knowledge graph overview: entities, triples, current vs expired facts, relationship types. | memory protocol v1 graph statistics — available | verified |
| `mempalace_traverse` | Walk the palace graph from a room. | memory protocol v1 graph traversal — available | verified |
| `mempalace_find_tunnels` | Find rooms that bridge two wings — the hallways connecting different domains. | memory protocol v1 graph traversal — available | verified |
| `mempalace_graph_stats` | Palace graph overview: total rooms, tunnel connections, edges between wings. | memory protocol v1 graph statistics — available | verified |
| `mempalace_mesh_peers` | Mesh estate snapshot (RFC 004): this replica's identity, version vector and node profile; each configured peer's reachability, last sync outcome, remo… | no facade — provider-internal | not exercised |
| `mempalace_create_tunnel` | Create a cross-wing tunnel linking two palace locations. | no facade — provider-internal | not exercised |
| `mempalace_list_tunnels` | List all explicit cross-wing tunnels. | no facade — provider-internal | not exercised |
| `mempalace_delete_tunnel` | Delete an explicit tunnel by its ID. | no facade — provider-internal | not exercised |
| `mempalace_list_hallways` | List within-wing hallway records (entity-to-entity co-occurrence links built at mine time). | no facade — provider-internal | not exercised |
| `mempalace_delete_hallway` | Delete a hallway record by its ID. | no facade — provider-internal | not exercised |
| `mempalace_follow_tunnels` | Follow tunnels from a room to see what it connects to in other wings. | memory protocol v1 graph traversal — available | verified |
| `mempalace_search` | Semantic search. | `maestro memory recall` — available | verified |
| `mempalace_check_duplicate` | Check if content already exists in the palace before filing | no facade — provider-internal | not exercised |
| `mempalace_add_drawer` | File verbatim content into the palace. | `maestro memory capture` — available | skipped (mutating) |
| `mempalace_checkpoint` | Save a whole session in one call: semantic-dedups each item, files non-duplicates as drawers, then writes one diary entry. | `maestro memory capture` — available | skipped (mutating) |
| `mempalace_delete_drawer` | Delete a drawer by ID. | no facade — provider-internal | not exercised |
| `mempalace_mine` | Mine a directory into the palace — the MCP equivalent of `mempalace mine`. | `maestro memory ingest` — available | skipped (mutating) |
| `mempalace_delete_by_source` | Bulk-delete every drawer mined from one source_file (exact match). | no facade — provider-internal | not exercised |
| `mempalace_sync` | Prune drawers whose source files are gitignored, deleted, or moved. | `maestro memory sync` — available | skipped (mutating) |
| `mempalace_get_drawer` | Fetch a single drawer by ID — returns full content and metadata. | no facade — provider-internal | not exercised |
| `mempalace_list_drawers` | List drawers with pagination. | no facade — provider-internal | not exercised |
| `mempalace_update_drawer` | Update an existing drawer's content and/or metadata (wing, room). | no facade — provider-internal | not exercised |
| `mempalace_diary_write` | Write to your personal agent diary in AAAK format. | `maestro memory capture` (session summary) — available | skipped (mutating) |
| `mempalace_diary_read` | Read your recent diary entries (in AAAK). | `maestro memory recall --startup` — available | verified |
| `mempalace_hook_settings` | Get or set hook behavior. | no facade — provider-internal | not exercised |
| `mempalace_memories_filed_away` | Check if a recent palace checkpoint was saved. | no facade — provider-internal | not exercised |
| `mempalace_reconnect` | Force reconnect to the palace database. | no facade — provider-internal | not exercised |
| `mempalace_event_append` | Append an immutable agent-coordination event to the logstream (RFC 003). | no facade — provider-internal | not exercised |
| `mempalace_event_list` | List agent-coordination events with structured filters, oldest first (append order, not timestamp order). | no facade — provider-internal | not exercised |
| `mempalace_event_wait` | Block until a matching coordination event exists or the timeout expires (default 60s, max 5 minutes). | no facade — provider-internal | not exercised |
| `mempalace_event_ack` | Acknowledge a coordination event: appends a new event.ack routed back to the original writer with the correlation id copied. | no facade — provider-internal | not exercised |
| `mempalace_artifact_put` | Store exact artifact content (unified diff patch, file, log, json, note) for agent handoffs. | no facade — provider-internal | not exercised |
| `mempalace_artifact_get` | Fetch a coordination artifact by id — exact content plus sha256 for verification. | no facade — provider-internal | not exercised |
| `mempalace_patch_submit` | Convenience: store a patch artifact and append its patch.ready event in one call. | no facade — provider-internal | not exercised |

## Notes

- Tools marked "available" are reachable today through `maestro memory` and the
  memory protocol v1 envelope; the concrete tool names above are provider
  vocabulary and stay confined to the MemPalace adapter.
- Tunnels, hallways, mesh, events, artifacts, and patch tools have no facade:
  Maestro reports such operations unavailable rather than bypassing the boundary.
