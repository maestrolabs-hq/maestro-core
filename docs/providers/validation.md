# Provider validation: evidence, not blanket green

Inspected 2026-09-05. Four read-only audits exercised isolated native fixtures
and inspected current source/contracts. Only the authorized GPT-6 agent ran;
no additional inference/model loading, production provider start/reindex or
memory maintenance occurred. This supersedes the earlier unsupported claim
that every non-destructive tool was exercised live.

## Boundary and coverage matrix

All live Pi MCP gateway calls remain **untested**; catalog metadata was cached
with servers disconnected. E below means **verified end-to-end only for the
named native scratch workflow**. See [capabilities](capabilities.md) for every
tool's failed, blocked, schema/source-only, not-applicable or untested scope.

| Provider | Verified useful native evidence | Failures / remaining scope |
| --- | --- | --- |
| CGC | CLI/native stdio index -> exact `entry CALLS helper` and imports/source; watch/edit/unwatch; ordinary rename/delete and new exclusion removal | Populated job status datetime failed; guarded repair independently verified installed. Escaping symlink source survived alias removal; preflight prevents new escape, historical cleanup deferred. Java-positive, dead-code, simulation/history, language breadth and browser untested. |
| CodeGraph | Init -> explicit-project native stdio source -> modify/rename/delete -> sync; empty complete corpus distinguished from uninitialized | Outside symlinks followed; startup catch-up/watch/hooks can write. Corrupt sync exits1, fallback init incorrectly exits0/Already initialized; helper removes that fallback. Wider resolution/impact quality untested. |
| Codebase-Memory | Fast CLI/native stdio index -> BM25/CALLS/IMPORTS/snippet/trace -> reindex/remove old source; new exclusion removal; whole-document ADR update/sections | Trace input accepted but runtime edges not implemented. No query_decisions tool. Coverage is best effort; full/moderate/token-vector paths blocked. Production generated-file contamination/stale pi-config bytes not cleaned. |
| Graphify | Real local AST rebuild -> delete -> scoped native stdio query/node/neighbors; degree ranking and source hash checks | Local update does not refresh global. Ownership/community/type repairs and legacy rebuild refusal independently verified installed; published global composes eight preserved input artifacts, not source freshness or global reclustering. Exact-ID/error fixes retained. Rust drop heuristic, semantic quality/browser still untested or failed. |
| Semantica | Explicit text/regex/pattern -> GraphBuilder -> ContextGraph -> save/fresh reload: 2 nodes/1 known edge; native decision/precedent, literal query, forward reasoning | Default KB empty-success, analytics and serializers/routing repairs independently verified installed. A real host-Astra source-admission pilot covers only 3 selected files/10 facts; no full-corpus extraction transport, safe shared publication or quality holdout result. Defaults/fallbacks/span/negation/splitter defects remain. |
| MemPalace | Synthetic HOME/--palace native handlers/storage: temporal add/supersede/as_of/timeline/invalidate; exact artifact/event/ack/patch-submit | No production native calls; history unchanged. Native stdio, Chroma retrieval, embedding model, distributed replication/hallway/tunnel quality untested/blocked. Status acknowledgement deletes marker; reads can initialize storage. |
| Docling | New guarded inventory-snapshot/MD-only native stdio and actual-worker conversion/key/search/item, denied fetch/model loaders; earlier curated cache/overview/export; native authoring/HTML tables/save; synthetic PIL thumbnail | Empty/all-unsupported directory returns [] success; not readiness. Anchors lack original Markdown spans; fence language lost, adjacent code/text context needed. PDF/OCR/images/remote/browser untested/blocked. |

Installed dependencies, counts, argument errors, empty/no-op responses and
inferred labels do not prove quality. Existing counts describe materialized
shape only. Real edge/source/temporal fixtures outrank previous assistant prose.

## Reproducible fixes versus installed runtime

Patch artifacts and their application recipe are tracked under
`maestro-pi-config/config/tools/semantica/`. The authorized rollout was
independently verified on 2026-09-05: all 11 installed postimages match five
manifests, and all three live skill files match source. Installed native,
31 helper and 7 adapter checks passed; persistent Docling uses exact
MCP3.2.0/slim2.125.0/core2.95.0 pins. Open Pi sessions still require manual
`/reload` and gateway permission/connect checks.

The patcher validates all SHA256 preimages/output hashes first,
rejects unknown versions/source states and is idempotent; simulated replacement
failure rolls back earlier replaced files. Whole-set crash atomicity and
concurrent live writers are NOT guaranteed: offline application/backup is a
precondition, not a new live package manager.

| Versioned regression | Observed red -> green in isolated copies | What it does not establish |
| --- | --- | --- |
| tests/semantica_native.py | 5 tests pass: initial 8 failed assertions and follow-up 3 swallowed graph errors reproduced red -> green; native PageRank/KB/JSON plus typed extraction doubles. Graph errors propagate; graph=False and normal/empty/repeat builds remain valid | Actual model quality, upstream fallback correctness, all split/span invariants, production atomic save or MCP transport. Native edgeless-community fallback/deprecation warnings remain. |
| tests/cgc_native.py | Updated pending/running/completed/failed job status/list: 8 JSON failures -> native handler round-trips pass | Live gateway/new job terminal observation after deployment; historical orphan cleanup. |
| tests/graphify_native.py | 5 native tests: observed loss/type/legacy failures -> repo-qualified externals/edge attributes/loops, disjoint offset communities, add/remove/unchanged/update/re-add and rebuild refusal pass; original exact-ID/stdio repairs retained | Production rebuild/reload, global reclustering or language-wide precision. |
| tests/docling_native.py | Real new native stdio schema/MD-only key/search/item/export, exact raw CRLF/negation/inline code/table/code, base64/hash/unsupported source/content failures, changed cache keys, zero fetch/model-loader entries and unknown-runtime rejection after observed red | Live filesystem freshness, independent reference attestation, PDF/OCR/image conversion, live gateway or full Windows/macOS runtimes. |
| tests/provider_preflight.py | 2 tests: canonical Git/.git-file/topic/dirty/quote/nested exclusion/symlink checks plus fail-first selected-only Markdown byte/hash inventory snapshot pass | Secret scan, native ignore deployment or full Windows/macOS execution. |
| tests/provider_patch.py | 2 tests pass: unknown-preimage refusal, failure rollback, all real manifests' output-hash guards and repeated scratch application | Crash-consistent multi-file transactions, concurrent writers or live installs. |
| tests/provider_fanout.mjs | Old skill marked 6 business errors ok; unpatched actual worker cannot bound branches. Patched actual worker passes seven rows, exact args, envelopes, permissions, one hang/late reply, deadline bounds and sandbox restrictions | Real host approval/connect/dispatch lifetime, all provider indexes, live tool-routing and quality. Host cutoff is simulated by terminating only the owned scratch worker, not a production server. |
| tests/provider_review.mjs | 31 tests pass after observed red: prior30 actual-worker native-envelope/no-match/CodeGraph opaque-text checks plus real Docling snapshot stdio chain, invalid/uninventoried snapshot rejection without fallback, inherited Graphify force through native CLI with mocked rebuild, native CodeGraph directory selection with CLI doubles, timeout/abort/business distinctions | Arbitrary conversion (blocked); real CodeGraph init/sync, Graphify rebuild or live gateway transports. CBM uses source-grounded BM25 rows; no native CBM query-echo defect is claimed. |
| tests/adapter_native.mjs | 7 tests pass after observed red: native executeCall -> mcpScript -> worker, typed SDK/legacy timeout/abort/closed transport, call/job/provider identity, single write submission and ordinary error text | Only the connected client callTool is doubled; actual host approval/connect/remote cancellation remains untested. Node24 experimental TypeScript-transform warning remains. |
| provision::tests::captured_provider_requirements_render_reproducible_install_steps | Missing Semantica and now Docling requirements red -> pinned uv steps/manual native prerequisites tested | Actual installs, transitive lockfile/model provisioning, provider aliases or ignore files. |

Run stdlib checks from maestro-pi-config:

```bash
python3 tests/provider_preflight.py
PROVIDER_BASELINES="$SCRATCH_BASELINES" python3 tests/provider_patch.py
node tests/provider_fanout.mjs "$PATCHED_ADAPTER_WORKER"
# Export GRAPHIFY_PYTHON, DOCLING_PYTHON, PYTHONPATH and CODEGRAPH_DIRECTORY_JS per patch README.
node tests/provider_review.mjs "$PATCHED_ADAPTER_WORKER"
node --experimental-transform-types tests/adapter_native.mjs "$PATCHED_ADAPTER_ROOT"
PYTHONDONTWRITEBYTECODE=1 PYTHONPATH="$SCRATCH_PACKAGES" "$SEMANTICA_PYTHON" tests/semantica_native.py
PYTHONDONTWRITEBYTECODE=1 PYTHONPATH="$SCRATCH_PACKAGES" "$CGC_PYTHON" tests/cgc_native.py
PYTHONDONTWRITEBYTECODE=1 PYTHONPATH="$SCRATCH_PACKAGES" "$GRAPHIFY_PYTHON" tests/graphify_native.py
PYTHONDONTWRITEBYTECODE=1 PYTHONPATH="$SCRATCH_PACKAGES" "$DOCLING_PYTHON" tests/docling_native.py
just check
```

Interpreter paths must select the inspected installed environments; copy source
packages into unique scratch first and apply the guarded patches there. Never
import native providers against production HOME/state for these tests. See the
patch README for complete copying/application commands. Mock extraction is
labeled honestly; no native model output is claimed from it.

## Required completion evidence

For each provider separately, preserve inputs/source revision+dirty bytes,
selected state/scope, exact args, native terminal state, expected/actual output,
errors/partial/truncation/skipped files, and permission/model attribution.
Prove known source/anchor, expected typed/directed edges, negative/no-fact cases,
delete/rename/new exclusion removal and persistence/reload where applicable.
No-op/accepted/pending/complete-empty/unconfirmed are distinct outcomes.

The bounded fanout emits settled rows independently. Timeout means **outcome
unknown**, not cancelled: queued/approval/connect work might dispatch later.
Do not retry writes or stop shared services. Native request deadlines alone
are applied after connection/approval; the captured worker call-deadline patch
bounds waiting, not remote execution. Long approved index work can use an
explicit 300s branch/330s enclosing budget; max branch budget is 600s with a
larger enclosing timeout and sufficient native requestTimeoutMs. None was
changed in production. A whole-script cutoff can leave only emitted partial
rows plus the host incomplete-call trace, not a final seven-row array. Typed
native request timeouts are also `timed_out_unknown`; abort/transport closure
is `aborted_unknown`, never confirmed cancellation. Legacy call_failed/-32001
error envelopes are conservative unknown outcomes, not ordinary business errors.

Positive checks use provider records/item text, never query metadata, no-match
echoes or anchor IDs. Docling exact search `{result}` is followed by
`get_text_of_document_item_at_anchor({document_key,document_anchor})` and `{text}`
validation. The one restored conversion route requires the exact selected
preflight Markdown byte snapshot/reference/hash and the new native MD-only guarded
tool. Native SHA256/format/backend constraints are enforced, not extension/OCR
attestation; no path fetch or fallback exists. Caller retains original bytes and
rechecks inventory: references are attributed metadata, not independently verified
live filesystem truth. Earlier curated Markdown/HTML evidence is not arbitrary
format authorization. See [Docling](docling.md) for the verified installed version boundary.

Preparation is not repair-by-cleaner. The full [stage inventory](preparation.md)
records native tools used, unsafe/bypassed defaults and unavailable/untested
branches. Original inline identifiers, negation, code, paragraph/table structure,
all occurrence spans and meaningful nulls must survive. Explicit span equality
and predicate/polarity/modality checks precede graph insertion; validation
scores/counts do not prove entailment. Native splitter offsets/zero overlap,
ExtractionValidator zero spans and dependency negation remain unfixed.

## Production and quality decisions still blocked

- Authorized Semantica extraction producer/transport, no-fallback admission,
  reviewed source labels/holdout and measured entity/relation precision/recall.
  No new model/endpoint/API/OAuth bridge or token handling is implied by pins.
- Source-scoped semantic refresh/deletion/rename/legacy ownership, decision and
  manual annotation retention; approved backup/quiescence, validated scratch
  quality, atomic direct-file persistence and fresh served reload before repair.
- Graphify current-source refresh across all eight roots and served reload
  verification. The published composition is not a source refresh; legacy artifacts
  are refused, not silently repaired by unchanged re-add.
- Eight-root provider ignore versioning/deployment, historical generated-output
  cleanup, complete native refresh-removal matrix and live gateway verification.
- Active Pi gateway verification after manual `/reload`, any further service
  restart/install/model load, browser rendering/interaction and cross-platform
  script execution.
- Production MemPalace writes remain prohibited, not a pending cleanup task.

Earlier follow-up pi-config `just check` passed; core `just check` exited101 at
its vocabulary gate on pre-existing untracked graphify-out reports. Those reports
were subsequently archived without weakening ignores or gates; independent
installed-rollout `just check` now passes in both repositories.
The default editor interpreter reports unresolved Graphify/Docling imports;
the intended installed interpreters plus scratch PYTHONPATH resolve and run them.
Those environment-only diagnostics were explicitly dispositioned, not suppressed;
this is not a blanket Python LSP-clean claim.

Rust LSP/just gates and deterministic tests supplement this evidence; they do
not execute model quality or every provider workflow. No blanket "all seven OK"
or "fully fixed" verdict is warranted by these tracked repairs.
