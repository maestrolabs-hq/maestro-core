# Direct provider state and Semantica

## Decision

All four providers use one HOME-derived workspace state root:

```text
<workspace>/.maestro/state/providers/
├── cgc/
├── graphify/
├── mempalace/
└── semantica/
```

The providers remain logically independent. CGC, Graphify, Semantica, and
MemPalace keep separate identities, indexes, scores, and result sets. No graph
or memory result is merged with another provider.

Each provider uses its direct active state. There is no active staging,
generation, promotion, proxy, extra hash workflow, or `semantic-context`
directory in this design.

## Provider layout

- **CGC:** use `<workspace>/.maestro/state/providers/cgc/kuzudb` directly through
  `CGC_RUNTIME_DB_TYPE=kuzudb` and `CGC_RUNTIME_DB_PATH`.
- **Graphify:** move the complete native Graphify home under
  `<workspace>/.maestro/state/providers/graphify`. Its served global graph is
  the direct file in that directory. Keep `~/.graphify` as a filesystem alias
  for native compatibility.
- **MemPalace:** move the complete native MemPalace home under
  `<workspace>/.maestro/state/providers/mempalace`. Keep `~/.mempalace` as a
  filesystem alias because MemPalace 3.8.0 has non-configurable native paths.
- **Semantica:** install `semantica==0.6.7` as-is and run its direct
  `semantica-mcp` stdio server. Set `SEMANTICA_KG_PATH` to
  `<workspace>/.maestro/state/providers/semantica/global-graph.json`. Seed the
  workspace-global graph only from `maestro-core` using Semantica's native
  repository ingestion. The wheel exposes exactly 15 MCP tools, including
  `query_graph` with `mode: "search"`; there is no `search_graph` tool.

The workspace placeholder is resolved from the runtime environment. Configured
paths use HOME interpolation and never name a machine.

## Native MCP surfaces

The direct provider MCP servers remain available as native provider surfaces.
Provider-specific tool names and counts stay in their adapters and provider
pages; Semantica's reviewed 0.6.7 wheel exposes exactly 15 tools. Direct native MCP
interfaces are the active integration surface, with no claim of merged provider
state.

## Migration sequence

1. Stop provider MCP processes and writers.
2. Verify the currently served active states before moving anything:
   - CGC: 691 graph nodes;
   - Graphify: 494 nodes and 1,132 edges;
   - MemPalace: 2,904 total drawers, including 1,458 for `maestro-core`.
3. Move the complete Graphify and MemPalace native homes into their provider
   directories and create platform-appropriate filesystem aliases at their
   original native locations. Move CGC's active KuzuDB into `cgc/kuzudb`.
4. Update the MCP configuration to use the direct provider paths and install
   the pinned Semantica package without adding a wrapper.
5. Build the initial Semantica graph from `maestro-core` through native
   repository ingestion and configure `semantica-mcp` to load its direct JSON
   file.
6. Restart the MCP client and verify all four providers. Semantica must be
   non-empty, and `query_graph` with `mode: "search"` must return a result.
7. Only after all checks pass, permanently delete the old unused provider
   directories, old `generations/` directories, `.staging` directories,
   provider backups, the unserved Graphify semantic graph, and the contents of
   `.maestro/artefacts/graph` as explicitly authorized.

The cleanup terms in step 7 describe retired files to delete; they are not
active provider architecture.

## Durability boundary

This direct phase documents Semantica's native limitations without fixing them:

- `add_entity` and `add_relationship` are not persistent across restart;
- `update_node` and `delete_node` write non-atomically when
  `SEMANTICA_KG_PATH` is configured.

The Semantica JSON file is not the Maestro ledger. No claim of transactional
provider mutation durability is made by this phase.

## Portability and rollback

The target layout is specified with HOME-derived placeholders. Symlinks are
used where supported and junctions are used on Windows; only the current WSL
execution is in scope for this migration. Native providers must continue to
resolve their aliases after the move.

Before final cleanup, moved paths and aliases can be reversed. After the
permanent cleanup in step 7, no historical rollback is promised.

## Ownership and documentation

Luna is the sole migration writer. Spark may perform later exact, bounded JSON
edits only after Luna no longer writes. The following documentation is part of
this change and must describe direct provider paths and the independent
boundary:

- `docs/providers/direct-provider-state-and-semantica.md`
- `docs/providers/semantica-evaluation.md`
- `docs/providers/mempalace.md`
- `docs/providers/cgc.md`
- `docs/providers/graphify.md`
- `docs/providers/semantica.md`

The three existing provider pages are sourced from commit `3997a2a` in the
`docs-toolchain` worktree and updated only for this state-layout decision.

## Validation

Run this sequence after implementation:

1. Validate the MCP configuration parses and all four direct commands start.
2. Query CGC and confirm 691 nodes.
3. Query Graphify and confirm 494 nodes and 1,132 edges.
4. Query MemPalace and confirm 2,904 total drawers and 1,458 in `maestro-core`.
5. Query Semantica with `get_graph_summary` and confirm a non-empty graph.
6. Query Semantica with `query_graph(mode: "search")` and confirm results.
7. Confirm the four providers resolve paths below the one physical workspace
   root, with only the documented native-home aliases outside it.
8. Confirm no old active provider path is used after cleanup.
