# Semantica

Semantica 0.6.7 supplies native graph composition, decision history, literal
retrieval and deterministic reasoning. Automatic trustworthy repository semantic
extraction is **not** established by the current integration.

Identities, indexes, scores and results remain provider-local and are never
merged. See [capabilities](capabilities.md) for every tool/schema disposition,
[preparation](preparation.md) for ordered source hygiene, and
[validation](validation.md) for runnable evidence and remaining gaps.

## Vocabulary mapping

| Semantica term | What it means | Our term (protocol / estate) |
| --- | --- | --- |
| ContextGraph | Semantic graph held by the provider | semantic graph |
| entity | A typed graph node | semantic node |
| relationship | A typed directed graph edge | semantic edge |
| decision | A recorded decision with supporting context | decision record |
| precedent | A related earlier decision or case | precedent |
| causal chain | A sequence of causal relationships | causal path |
| provenance | Source and extraction metadata | source evidence |
| query_graph | Search or traverse the semantic graph | semantic graph query |

## Inspected wiring (not changed by this repair)

```json
"semantica": {
  "command": "semantica-mcp",
  "env": {
    "SEMANTICA_KG_PATH": "<workspace>/.maestro/state/providers/semantica/global-graph.json"
  }
}
```

## Direct state and demonstrated native components

One active file: `<workspace>/.maestro/state/providers/semantica/global-graph.json`.
The server loads it only at startup. No proxy, alternate provider identity,
generation/promotion architecture or live replacement was added. Fifteen native
tools plus three resource aliases are exposed; no tool named search_graph.

The audit found eight repository nodes, 251 file nodes and 680 entities (939
nodes/2099 edges). All edge properties were empty; 430/457 semantic edges were
generic related_to. Those shape counts are NOT quality evidence. Every named
file existed, with 229 exact contents and 22 empty-file ID fallbacks, not 22
proven stale documents. Entity labels such as PERSON/Maestro, NORP/herdr and
GPE/Rust are not source-established architecture identities.

A real no-model native composition worked in scratch:
`extract_code_files` -> explicit text parse -> regex entity occurrences/pattern
relation -> `GraphBuilder(extract=False,merge_entities=False,resolve_conflicts=False)`
-> ContextGraph save/fresh reload, yielding the expected two nodes/one edge.
This is a deterministic component fixture, not production extraction accuracy.
Markdown autodetection fails; `DocumentParser.parse(...,file_type="text")`
retains Markdown bytes but does not interpret its structure. Use the verified
MarkdownIt original line maps described in [preparation](preparation.md).

Native forward reasoning derived Thing(Aster) from supplied facts/rules.
Structured decision/precedent queries worked on two explicit scratch decisions;
precedent similarity is lexical, not demonstrated embedding retrieval.
`query_graph(mode:"search")` is case-insensitive contiguous substring matching
on id/content, in graph iteration order with a limit. No embeddings are involved.

## Reproducible guarded repair, not live deployment

`maestro-pi-config/config/tools/semantica/semantica-0.6.7.patch` accepts only
known exact official/locally-hotfixed source preimages. Scratch regressions fix:

- PageRank dict-node adaptation, correct filtered-neighbor normalization and
  dangling mass, real edge/community counts and empty-graph output. Existing
  two crash hotfixes alone did not fix these values.
- Relation subject/predicate/object serialization with confidence/context/metadata;
  native Decision.to_dict causal-chain JSON; add_entity label -> content mapping;
  explicit MCP isError for business errors.
- KB builds require a wired explicit extraction pipeline, unwrap successful
  output, validate its shape and avoid implicit graph extraction. Empty valid
  extraction differs from unwired success. Requested GraphBuilder import/storage/
  processing errors now propagate through top-level ProcessingError rather than
  returning a placeholder with success_rate=1. Explicit graph=False still omits
  the graph. This does not construct a new pipeline.
- Relation chunks retain explicit routing kwargs; entity/relation/triplet result
  caching is disabled until an approved nonsecret account/cache scope exists.
  Same model/text/endpoint can belong to different accounts; never log/cache keys.

After separately approved deployment, verify patch output hashes and rerun
native analytics tests against that runtime. The tracked fanout then supports
`semanticaAnalyticsVerified: true` plus explicit analyze/startup permission;
this caller attestation is not automatic deployment detection. The opt-in's
dispatch was mock-tested and is disabled here. Analytics remains whole-graph,
not a repository-specific quality certificate.

See the patch recipe/tests and [validation](validation.md). The patch does NOT
wire an authorized model transport, repair every extractor/splitter, replace
the active graph or make native mutations crash-safe. Edgeless community
fallback still logs a native warning although tested counts are correct.

## Extraction admission and unresolved production policy

Only `openai-codex/gpt-6-astra` is authorized for inference in this repair.
Semantica's OpenAI SDK uses Chat Completions/API-key auth; Pi Codex uses OAuth
Responses transport. Do not paste an OAuth token into OPENAI_API_KEY or treat
an OpenAI-compatible base URL as subscription integration. No such bridge was
built. The selected-source pilot below consumes an already-produced host-agent
artifact; admission does not run inference. Optional spaCy 3.8.0 models/libraries being installed is
not permission or quality; regex does not require them. MCP extraction routing
still does not expose/forward the necessary authorized transport options.

Bypass destructive Markdown stripping: it removed inline subjects/objects,
flattened tables and lost negation/provenance. Do not pre-deduplicate occurrence
spans or cap entities before relations without visible truncation. Require exact
original quotes/spans, repo/path/observed revision, polarity, modality, temporal
bounds, method/model/config and confidence meaning on each accepted assertion.
Dependency extraction can ignore negation, native LLM spans can default to zero,
and implicit fallback can invent adjacency. The narrow admission below checks
schema and exact source bytes; general entailment and labeled holdout extraction
quality remain unimplemented, not silently repaired.

### Selected-source artifact pilot (scratch only)

`maestro-pi-config/config/tools/semantica/admit.py` accepts only schema_version1:
`model`, `repository`, `sources[{path,sha256}]`, and
`facts[{subject,predicate,object,polarity,status,evidence:{path,start_byte,end_byte,quote}}]`.
The model header must be `openai-codex/gpt-6-astra`; verify actual host dispatch
identity and `xhigh` separately. Header text is not runtime attestation.
The recovered host extraction's original harness exit1 (incorrect no-repo-edits
acceptance check) remains recorded, not relabeled as success or regenerated.

The host selects reviewed nonsecret sources explicitly, compares **every** fact
with source context, and rejects inaccuracies before admission. Fresh canonical
preflight, exact selected-set equality, SHA256 and strict UTF-8 exclusive-end
byte slices reject stale/missing/unselected/unsafe references and malformed or
duplicate records. Every selected source must have evidence; a source with no
facts is unsupported by this narrow schema. Admission does not detect omitted
assertions or prove entailment. References/hashes are attribution at observation,
not race-proof capabilities or a lock on later source changes.

Each distinct fact occurrence becomes a native FACT node. Searchable content
includes `[status; polarity]`, subject/predicate/object, exact quote, path/span
and hash; native node metadata retains the original fields. No positive semantic
edges, invented confidence, embeddings or planned-to-implemented inference.
A SOURCE_SCOPE node lists only the selected sources and observed checkout.
Each run builds a fresh ContextGraph, saves/reloads using native serialization,
then publishes a **new path only** via a same-directory hard link. Existing
outputs are never replaced; invalid input publishes nothing. Requires a local
filesystem supporting hard links; no shared-write/crash-durable replacement
contract. Changed/deleted/renamed sources require a newly reviewed artifact and
selection; old snapshots remain separate, never silently reused or merged.

Verified pilot: **only README.md, docs/supervisor.md, crates/cli/src/main.rs**
in maestro-core, with10 source-checked facts. Not complete-repository coverage.
Native stdio search/node queries retain neither-is-built, planned supervisor,
not-yet-specified contract, and the implemented `eprintln!`/`exit(1)` distinction.

Repeat from maestro-pi-config with inspected existing `SEMANTICA_PYTHON`, the
patched scratch `PATCHED_PACKAGES` from the repair recipe, and the unchanged
reviewed host `ARTIFACT` (no inference/client/production startup):

```bash
WORKSPACE="$(dirname "$(pwd -P)")"
SCRATCH="$(mktemp -d)"
PYTHONPATH="$PATCHED_PACKAGES" "$SEMANTICA_PYTHON" tests/semantica_admission.py
PYTHONPATH="$PATCHED_PACKAGES" "$SEMANTICA_PYTHON" tests/semantica_admission.py \
  --pilot "$ARTIFACT" "$WORKSPACE/maestro-core" "$WORKSPACE" "$SCRATCH/pilot.json"
```

`--pilot` runs the actual admission CLI with `--artifact`, `--repo`, `--workspace`,
repeated `--source` for those3 files, and `--output-new`; then native save/load,
MCP stdio `query_graph` search and full node reads for every fact. It prints raw
request/response evidence, denies network/spaCy model loading, owns scratch
HOME/cache/model directories and closes stdin for EOF cleanup. No extraction
mocks are used. The separate existing semantica_native.py extraction tests still
use typed doubles and do not certify this model step.

Automatic fanout **index remains separately unconnected/blocked**; this manual
pilot does not enable a readiness flag, deploy patches or replace the served KG.
The removed skill companion upserted source files, retained deletions/legacy
nodes, and reported upserts as additions. No semantic refresh happened there.
Production reconciliation needs approved source ownership/archival/decision
retention, multi-writer policy, backup, validated scratch evidence, atomic save
and fresh reload before direct active-file replacement.

Native add_entity/add_relationship/record_decision mutate memory without saving;
update_node/archive save non-atomically. Archived nodes remain searchable.
No persistence contract was silently changed by this patch. Keep production
writes/restarts blocked until that separate decision and quality check.
