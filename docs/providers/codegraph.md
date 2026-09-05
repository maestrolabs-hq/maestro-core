# CodeGraph

CodeGraph 1.6.0 (`@colbymchenry/codegraph`) uses a native Rust/tree-sitter
extractor and per-project SQLite/FTS index. It returns grouped source and
structural context through one default-listed MCP tool.

Identities, indexes, scores and results remain provider-local and are never
merged. See [capabilities](capabilities.md) for every tool/schema disposition,
[preparation](preparation.md) for ordered source hygiene, and
[validation](validation.md) for runnable evidence and remaining gaps.

## Vocabulary mapping

| CodeGraph term | What it means | Our term (protocol / estate) |
| --- | --- | --- |
| explore | one call returning source + call path + blast radius | structural query |
| blast radius | the set of symbols affected by a change | impact set |
| staleness banner | inline warning that a returned file has pending edits | index-lag warning |
| kernel | the native Rust extraction engine | provider-native extractor |
| auto-sync / watcher | OS-level file watch that keeps the index current | provider-native indexing |
| daemon | background process shared across sessions on one project | no equivalent — provider-internal |
| bundle install (into agents) | writing MCP config into other agents' own config files | no equivalent — not used, Maestro wires MCP itself |
| PR dashboard / list / triage | GitHub pull-request tooling referenced in the CLI help | no equivalent — out of Maestro scope |

## Inspected wiring (not changed by this repair)

```json
"codegraph": {
  "command": "codegraph",
  "args": ["serve", "--mcp"],
  "cwd": "<workspace>/maestro-core"
}
```

## Scope and state

The server chooses its project from cwd or explicit `projectPath`. Always pass
`projectPath` when comparing another canonical repository. Only maestro-core's
`.codegraph` was aliased to `<workspace>/.maestro/state/providers/codegraph` in
the audit; the other seven stores were repository-local. `CODEGRAPH_DIR` accepts
one directory-name segment, not an arbitrary external path. No relocation was
performed or implied by these docs.

The server itself opens/watches the index and performs catch-up sync before
serving tools. `--no-watch` does not suppress catch-up. CLI init is not proof of
a permanent independent watcher and can install Git fallback hooks. A "read"
can therefore cause startup writes; read-only audits must not start it blindly.

## Verified native flow and failure boundaries

Scratch init -> native explicit-project MCP query -> modify/delete/rename ->
sync yielded the expected `entry calls helper`, then `renamed_entry calls
helper`, exact new source, and removed old symbols. External-file symlinks
were followed but their aliases were removed on sync; source preflight must
reject escaping links before ingestion. Newly excluded sources were removed
on refresh while unrelated symbols survived.

`codegraph sync "$REPO"` failing with `file is not a database` must stay failure.
`sync || init` is incorrect: init only sees an initialized marker and can return
0/Already initialized over corruption. The shared fanout selects init only for
an absent DB, uses explicit cwd, and checks each exit separately. Supply
`codegraphDir` from the verified MCP runtime selection, explicitly `.codegraph`
for default or e.g. `.codegraph-win`. The helper validates one directory segment
and pins CLI `CODEGRAPH_DIR` to it before testing that selected DB. It cannot
inspect the MCP process environment; confirm that identity before approval.
Default/alternate existing/absent/corrupt layouts were tested using native
directory resolution and a recording/failing CLI double, not full native sync.

An initialized empty source corpus can legitimately report
`fileCount:0,lastIndexed:null,index.state:complete`. Distinguish that from not
indexed. Byte hashes of indexed source match only the represented files;
unsupported docs/config and unresolved relationships remain coverage gaps.

## Query surfaces

Gateway name is `codegraph_codegraph_explore({query,projectPath})`.
Fanout treats formatted text as opaque returned data, never source evidence:
query echoes can forge headings and fences. Even genuine source-looking output
never yields `probe-matched`. Nonempty queries without `expectedEvidence.codegraph`
yield `returned`; evidence-requesting queries and post-index probes remain
`unconfirmed` until actual source verification outside the helper. Native
`No relevant code found for ...` remains `complete-empty`; unindexed and error
responses stay distinct. Raw text and tool metadata are preserved.
Native `codegraph_status` can be called raw but is unlisted by default, so normal
gateway discovery cannot route it. Seven narrower native tools can be listed
with `CODEGRAPH_MCP_TOOLS`; do not expand that configuration without need.

CLI `status --json "$REPO"`, `sync "$REPO"`, `init --yes "$REPO"`, `explore`,
`callers`, `callees`, `impact` and `affected` are useful native surfaces. Only
the specific structural fixture above was verified end-to-end; broader language,
framework/dynamic dispatch and impact accuracy remain untested. No model-based
inference was found in the inspected structural route; this does not authorize
an install/self-heal download if the native bundle is missing.
