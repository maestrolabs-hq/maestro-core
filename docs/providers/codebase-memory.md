# Codebase-Memory (CBM)

Codebase-Memory 0.10.8 is an independent native code graph with bundled
tree-sitter/type-resolution machinery and SQLite storage. The inspected release
source is C, not the previously documented Go implementation.

Identities, indexes, scores and results remain provider-local and are never
merged. See [capabilities](capabilities.md) for every tool/schema disposition,
[preparation](preparation.md) for ordered source hygiene, and
[validation](validation.md) for runnable evidence and remaining gaps.

## Vocabulary mapping

| Codebase-Memory term | What it means | Our term (protocol / estate) |
| --- | --- | --- |
| project | one indexed repository, keyed by its root path | indexed repository |
| index_repository | scanning code into the direct provider database | provider-native indexing |
| blast radius (`detect_changes`) | transitive callers/dependents of a git diff | impact set |
| ADR (`manage_adr`) | a per-project architecture document stored in the graph | provider-native architecture note |
| team-shared graph artifact (`.codebase-memory/graph.db.zst`) | optional compressed snapshot committed to the repo | no equivalent — not used, cache root is configured directly |
| daemon / watcher | shared background process that keeps indexes current | no equivalent — provider-internal |
| Hybrid LSP | language-server-backed type resolution layered on tree-sitter | provider-native type resolution |
| Cypher-like query (`query_graph`) | the graph query language | graph query |
| install / hooks / subagent profiles | writing MCP config and hooks into other agents' own config files | no equivalent — not used, Maestro wires MCP itself |

## Inspected wiring (not changed by this repair)

```json
"codebase-memory": {
  "command": "codebase-memory-mcp",
  "args": [],
  "env": {
    "CBM_CACHE_DIR": "<workspace>/.maestro/state/providers/codebase-memory"
  }
}
```

## Scope and isolation

`CBM_CACHE_DIR` selects the provider cache. `CBM_RUNTIME_DIR` additionally
selects its private daemon/rendezvous directory; cache isolation alone is not
sufficient. CLI operations may start a temporary daemon and write logs/runtime
state. ADR reads can migrate a legacy document into SQLite. Native read-tool
labels do not guarantee no writes. No `CBM_ALLOWED_ROOT` was set in the inspected
configuration; canonical caller preflight is required before writes.

Use the provider's `list_projects` identity, then pass `project` to scoped tools.
`index_repository` uses `repo_path`, optional explicit `name`, `mode:"fast"`,
`persistence:false`. Team artifact persistence would write a repository file.
The fast route avoids model-derived similarity; full/moderate and
`semantic_query` are distinct paths. The latter lazily uses embedded pretrained
Nomic token vectors; it is not equivalent to deterministic AST certainty and
was not exercised under the model authorization restriction.

## Verified native workflow

A tiny Python fixture passed CLI/native MCP fast index -> BM25/Cypher exact
CALLS/IMPORTS -> exact snippet/trace -> modify/delete/rename -> reindex. New
exact-file exclusions removed old symbols while preserving helper. External
symlinks returned `outside_project`, unlike CGC/CodeGraph. This does not prove
162-language completeness, every query mode, pagination or historical blast
radius. Hybrid LSP labels mean native resolution evidence, not proof that an
external language-server process ran or a measured accuracy score.

`check_index_coverage` requires `paths` or `scopes` and explicitly says
`signal:best_effort`: no recorded issue is not source completeness. Retain
freshness, skipped/partial records, recommended_action and paging metadata.
Unknown extensions can be `not_tracked`; do not remove them to improve a count.

## Important unsupported claims

- `query_decisions` does not exist among the 15 tools. `manage_adr` stores one
  explicit project document; update replaces that entire document, and sections
  retrieves its headings. The scratch whole-document write/read worked, but
  production summaries were empty despite tracked ADR files. Automatic ADR
  import is not promised; any import needs provenance and overwrite approval.
- `ingest_traces` returns accepted with "Runtime edge creation from traces not
  yet implemented". CALLS stayed unchanged. It is a stub, not enrichment.
- `search_code` uses `pattern`; `trace_path` uses `function_name`;
  `get_architecture` requires `project`, and `adr` is not an architecture aspect.
- All eight production hash tables contained graphify-out generated artifacts;
  pi-config INVENTORY/skill bytes were stale. No production reindex or cleanup
  occurred. `.cbmignore` is still absent; CGC ignores are not a substitute.

Use native missed-graph/coverage diagnostics for future targeted investigation;
actual missed-graph and committed-diff correctness are untested. Do not fuse
these records with another provider or stored memory history.
