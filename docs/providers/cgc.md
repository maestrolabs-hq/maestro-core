# CodeGraphContext (CGC)

CGC 0.6.8 parses source with tree-sitter into an embedded structural graph.
The inspected backend is KuzuDB; optional vector resolution is a separate
model-loading path, not part of the verified deterministic fixture.

Identities, indexes, scores and results remain provider-local and are never
merged. See [capabilities](capabilities.md) for every tool/schema disposition,
[preparation](preparation.md) for ordered source hygiene, and
[validation](validation.md) for runnable evidence and remaining gaps.

## Vocabulary mapping

| CGC term | What it means | Our term (protocol / estate) |
| --- | --- | --- |
| context | one graph database instance | workspace-context source |
| repository (indexed) | a scanned code snapshot in the database | indexed repository |
| index / update / watch | scanning code into the direct provider database | provider-native indexing |
| job | background indexing task | no equivalent — provider-internal |
| bundle | pre-indexed portable graph package | no equivalent — not used |
| Cypher | the graph query language | graph query |
| god nodes / hotspots | most-connected or riskiest entities | architectural report |

## Inspected wiring (not changed by this repair)

```json
"cgc": {
  "command": "cgc",
  "args": ["mcp", "start"],
  "env": {
    "CGC_RUNTIME_DB_TYPE": "kuzudb",
    "CGC_RUNTIME_DB_PATH": "<workspace>/.maestro/state/providers/cgc/kuzudb"
  }
}
```

## Scope and successful native workflow

Direct DB: `<workspace>/.maestro/state/providers/cgc/kuzudb`. A tiny Python
fixture independently produced `entry CALLS helper`, expected imports and exact
source over native CLI and native stdio MCP. Ordinary update/delete/rename,
watch/edit/query/unwatch and new exact-file exclusion removal worked in scratch.
The 26-language catalog is availability, not measured language coverage.

`add_code_to_graph({repo_path})` DOES index a new native MCP repository. A job id
was followed by the exact new function source. An existing repository returns
`success:false`/already indexed: a no-op, not refresh. Both populated job status
handlers fail JSON serialization of `last_update_time`; a guarded 0.6.8 patch
passes all native job-state regressions in scratch, **not deployed**.
The earlier claim that only CLI works was a historical gateway failure, not a
native limitation or a diagnosed gateway root cause. Live gateway remains
untested; preserve original `error.data`, not just "Tool execution error".

## Native operations and constraints

- `cgc --db kuzudb --path "$DB" index "$REPO" --summarize` creates an index;
  `update "$REPO"` refreshes. Global flags precede the subcommand. Runtime
  `CGC_RUNTIME_DB_TYPE` / `CGC_RUNTIME_DB_PATH` also select the CLI database.
- `find_code` and most analyses accept `repo_path`; stats uses `repo_path`, not
  the unsupported schema spelling `path`. Cypher uses `cypher_query`.
- The tracked fanout's `cgcMode: "refresh-cli"` requires the selected existing
  `cgcDbPath`, matching the follow-up MCP DB, and confirmed `cgcOwnerState`
  (`unowned` or independently `released`). It runs update then a scoped source
  probe; default `index-new` is not refresh. Helper ordering is mock-tested.
- Kuzu's owner lock can block CLI while MCP holds it. Report blocked and seek
  explicit maintenance permission; never automatically stop/restart services.
- Root validation does not confine symlink targets. Native CGC indexed an
  external file then left its outside-path symbol after removing the alias.
  Shared preflight now rejects escapes before new writes; historical orphan
  cleanup remains unimplemented and production refresh unapproved.
- `.cgcignore` is provider-specific; it does not protect CBM or CodeGraph.
  Versioning/deploying all eight untracked ignore files remains outstanding.
- `generate_report` writes a file (default cwd/CGC_REPORT.md); tested content
  was suggested Cypher, not a validated architecture report. Kuzu visualization
  returns a localhost URL; browser/server correctness is untested.
- Spring beans/datasource queries failed binder checks on a Python corpus.
  Positive Java, dead-code validity, historical evolution, simulation quality,
  bundles, package ingestion, context switching and other backends are untested.

No provider-specific Pi extension is required. Use native CLI/MCP only after
permission checks; installing parsers or returning counts does not certify
current-source completeness.
