# Graphify

Graphify 0.9.53 (`graphifyy[mcp,openai]`) produces portable repository graphs
from AST/config extraction and optional separately authorized semantic passes.

Identities, indexes, scores and results remain provider-local and are never
merged. See [capabilities](capabilities.md) for every tool/schema disposition,
[preparation](preparation.md) for ordered source hygiene, and
[validation](validation.md) for runnable evidence and remaining gaps.

## Vocabulary mapping

| Graphify term | What it means | Our term (protocol / estate) |
| --- | --- | --- |
| corpus | the scanned source tree | repository source |
| graphify-out/ | output directory holding graph.json | provider output |
| global graph | the graph file served by MCP | direct served graph |
| community | clustered group of related nodes | module cluster |
| god node | most-connected node | architectural hub |
| EXTRACTED / INFERRED / AMBIGUOUS | edge provenance classes | edge provenance |
| semantic extraction | LLM pass turning documents into nodes/edges | document semantic pass (requires exact model authorization) |
| PR dashboard (list/triage/impact) | GitHub pull-request tooling | no equivalent — out of Maestro scope |

## Inspected wiring (not changed by this repair)

```json
"graphify": {
  "command": "graphify-mcp",
  "args": ["global-graph.json"],
  "cwd": "<workspace>/.maestro/state/providers/graphify"
}
```

## Selected graph and attribution

`project_path` selects `<repo>/graphify-out/graph.json`. Omission selects the
configured served global graph. Native `graphify update "$REPO"` is AST-only
and refreshes the local graph, **not** that global artifact. The skill now
queries the same project scope it updated. `--force` allows validated shrink
on delete/rename; it is not an automatic override of every safety check.

Native local rebuild/delete and separately selected native stdio reads worked
in scratch. Counts/text stats use `Edges:`; the JSON file's field is `links`.
Query logging is conditional and off by default; inspect inherited environment
or explicitly disable it for isolated tests. Some cache reads initialize dirs.

The audited sources all had `_origin:"ast"`, no nonempty semantic_hash and no
model attribution on edges. `INFERRED` can mean deterministic cross-file
heuristics, not an LLM. Historical generating-model identity is unknown; the
previous qwen preset attribution was unsupported. Optional semantic/model,
labeling and LLM dedup paths remain blocked under the exact-model restriction.

## Guarded fixes, not deployed

The 0.9.53 source patch reuses `_resolve_single_node` for shortest paths, so an
exact source/target ID produces its actual one-hop edge instead of fuzzy-matching
an unrelated hub. It preserves ambiguity reporting. Native stdio exceptions now
raise ToolError/isError rather than ordinary "Error executing" success text.
Both regressions passed against an isolated package copy. Live gateway and
installed provider remain unchanged.

The fanout command explicitly sets `GRAPHIFY_FORCE=0`, then adds `--force` only
with approved graph shrink. A native CLI fixture under inherited force=1 records
false/true at `_rebuild_code` for unapproved/approved requests, with that rebuild
mocked and zero graph writes. Positive probes use NODE/EDGE records, not query
headers. This does not validate a real refresh lifecycle.

## Global graph remains degraded

The installed native global add still merges source-less external nodes by
label and can delete B's relations when removing their first owner A; unchanged
re-add skips repair. Repo-local community numbers also collide. Production
surprising-connections/questions/community interpretation remains degraded.

The extended guarded 0.9.53 patch removes that label dedup/remap, retaining
repo-qualified identities for **all** nodes and their incident edges/attributes.
It uses existing `community_offset` to allocate disjoint numeric IDs, not global
reclustering. Simple undirected graphs retain self-loops; directed/multigraph
imports are rejected before writes rather than flattened. No cross-repository
shared-ownership registry or cross-provider merging was added.

Scratch native add A/add B/remove A/unchanged re-add B/update/delete/re-add
regressions preserve B's reference edge and self-call and isolate same-label
externals/community IDs. The earlier shortest-path/error repairs still pass.
Manifest version2 and graph `global_schema_version:2` form an explicit safety
boundary: legacy/incomplete state is refused **before unchanged-hash skips and
writes**. Do not just change version markers. Rebuild separately from validated
original per-repo graphs in empty scratch state, then verify exact typed edges,
source/manifest hashes and served reload before any separately approved rollout.

These are patch-artifact/scratch results, not deployed results. No production
global add/remove/rebuild automation is approved or shipped in the fanout, and
no readiness flag was flipped. Current production sources declare simple
undirected graphs; the directed/multigraph fixture was an unsupported-input
regression, not evidence of those particular edge losses in production.

A separate Rust heuristic incorrectly linked prelude `drop(...)` to a
`Drop::drop` method. Built-in/receiver-aware resolver repair remains deferred;
provenance categories and unchanged edge counts are not precision metrics.

## Useful bounded capabilities

Exact node/neighbors, local graph query, degree-ranked hubs, known source CALLS
and implements edges are independently useful. Native detect/hash/manifest
checks distinguish source freshness from aggregate-artifact hashes. The
provider-supplied Pi skill is installed separately; follow this estate's
permission/model/source boundaries before any extraction. Ten native tools
plus six resource aliases do not mean sixteen native tools; resources use the
default graph and cannot receive `project_path`. PR/network and browser workflows
are untested/not applicable here. HTML existence is not browser validation.

## Generated report archive after refresh

Keep `graphify-out/graph.json` and other graph artifacts in place for native
`project_path` discovery. Version0.9.53 has no report-only output setting:
its output-directory selection also relocates graph JSON. Do not redirect the
whole graph directory, change source exclusions or weaken a repository gate.

After an explicitly approved report-producing refresh, pause that producer,
inspect each report's SHA256, then run the checkout's small report-only helper.
It requires a **new external directory on the same filesystem**, validates all
selected report preimages before moving anything, and preserves original relative
paths, bytes, modes and provenance. Existing archival generations never overwrite.
A failure may leave a partial archive; reconcile its provenance before resuming,
not a blind retry. Retained reports are historical output, not eligible source.

From maestro-pi-config, with the approved canonical `REPO_PATH`, inspected
`REPORT_SHA256`, and a new generation name `ARCHIVE_ID`:

```bash
WORKSPACE="$(dirname "$(pwd -P)")"
ARCHIVE_PARENT="$WORKSPACE/.maestro/state/providers/graphify/reports/$(basename "$REPO_PATH")"
mkdir -p -m 700 "$ARCHIVE_PARENT"
python3 config/tools/graphify/archive_reports.py --repo "$REPO_PATH" \
  --archive-new "$ARCHIVE_PARENT/$ARCHIVE_ID" \
  --report graphify-out/GRAPH_REPORT.md "$REPORT_SHA256"
```

Repeat `--report <original-relative-path> <inspected-sha256>` for a dated report
in the same generation. Verify originals are absent, archival hashes match, and
unchanged `just check` passes. Consumers open the archived relative path explicitly;
`callflow-html --report` accepts that path. Native `graphify://report` only checks
beside its selected default graph and cannot discover an external report archive.
Do not put a report symlink back into the checkout to mimic the old lookup. Graph
queries still use the unchanged local JSON path; report placement proves neither
current source freshness nor graph quality.
