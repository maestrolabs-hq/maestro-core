# Docling MCP

Docling MCP converts documents to in-process structured documents and exposes
anchor reads/authoring. It is not a repository graph or a global index.

Identities, indexes, scores and results remain provider-local and are never
merged. See [capabilities](capabilities.md) for every tool/schema disposition,
[preparation](preparation.md) for ordered source hygiene, and
[validation](validation.md) for runnable evidence and remaining gaps.

## Vocabulary mapping

| Docling MCP term | What it means | Our term (protocol / estate) |
| --- | --- | --- |
| local document cache | in-process cache of converted `DoclingDocument`s, keyed by a hash of the source | conversion cache (process-lifetime, not persisted to disk) |
| document key | the cache key for a converted document | conversion cache key |
| anchor (`#/texts/2`) | a stable reference to one item in a document's structure | structural reference |
| conversion mode (local / remote) | run conversion in-process, or delegate to a hosted `docling-serve` instance | conversion backend selection |
| `docling-mcp[local]` extra | the package variant that bundles the `docling` conversion library itself | local conversion capability |
| generation / manipulation tools | tools that build or edit a `DoclingDocument` from scratch, item by item | document assembly tools |

## Inspected wiring (not changed by this repair)

```json
"docling": {
  "command": "uvx",
  "args": ["--from=docling-mcp[local]", "docling-mcp-server", "--transport", "stdio"],
  "env": {
    "DOCLING_ARTIFACTS_PATH": "<workspace-unrelated>/models/document/parsing/docling",
    "DOCLING_MCP_CONVERSION_MODE": "local",
    "DOCLING_MCP_DO_OCR": "false"
  }
}
```

## Identity and state

The audited already-cached environment was docling-mcp[local] 3.2.0,
docling-slim 2.125.0 and docling-core 2.95.0. Governed uvx remains unpinned and
was not rerun; cached version is not a reproducible fresh resolver guarantee.
No protected MCP configuration was changed.

Three state classes differ: model artifacts (operator model estate), process-
lifetime converted/authoring documents, and explicit disk outputs via CACHE_DIR.
Save writes Markdown and JSON; page_thumbnail writes a PNG and mutates the
cached PIL image despite readOnlyHint. CACHE_DIR is unset in governed config and
can default near the package; configure an approved isolated directory before
using disk-output tools. Restart empties the document cache, not model weights.

Explicit `--transport stdio`, local conversion and OCR false were exercised.
Defaults are HTTP transport/remote conversion/OCR enabled, so implicit settings
can select a materially different operation. Local conversion does not imply
all formats are model-free. PDF/OCR/images/remote/object storage remain untested
or blocked; no model was downloaded/loaded in the named Markdown/HTML probes.

## Verified native conversion and authoring

Curated Markdown -> convert -> nonempty document_key -> in-cache -> overview ->
search with `text:"Quartz beacon"` -> exact `#/texts/1` item text -> Markdown
export succeeded over real native stdio without mocks/models. Repeated/same-
content conversion reused the key in the same process; a new process had no
retained cache. Source path attribution may describe the first identical copy.

`search_for_text_in_document_anchors` requires **text**, not query. Anchors are
`#/texts/1`, not decorated `[anchor:...]` strings. An empty/all-unsupported
directory returns [] with isError:false; the nonrecursive directory wrapper
swallows individual conversion errors. It is not an index/readiness success.
Prefer curated single files and account for every input/failed/unsupported item.

## Guarded local Markdown snapshot route (not deployed)

The new 3.2.0 patch registers exactly one additional tool:
`convert_markdown_snapshot_into_docling_document(repository_root,source_path,source_sha256,content_base64)`.
All arguments are required. Existing broad conversion/directory tools are retained
for separately authorized use; fanout never falls back to them on old runtimes.

Run the existing canonical preflight with `--markdown` naming one eligible
inventory-relative `.md` file. It refuses excluded/unlisted/outside/URL/directory
and selected symlink paths and emits base64 only for that selected file, from the
same bytes as the inventory SHA256. Keep that original body unchanged. Fanout
requires the exact source reference and matching path/hash in that fresh inventory.
Native code validates base64, SHA256 and strict UTF8 before cache/conversion, but
**never opens/fetches the supplied path**. References are caller-attributed metadata,
not independent filesystem verification or race-free live-checkout freshness.

The native `DocumentConverter` allows only MD, with explicit
`MarkdownDocumentBackend`/`SimplePipeline`. Images, local/remote resource fetch,
remote services, external plugins and model enrichment are disabled. It refuses
unreviewed backend versions (MCP3.2.0, docling-slim2.125.0, docling-core2.95.0 are
required), not silently ignored optional policy arguments. Actual misleading
PDF/image bytes fail without entering model loaders. PDF/OCR/image conversion,
URLs, object storage and directories remain outside this route.

Real denied-network scratch stdio and actual adapter-worker -> native conversion
-> key -> search(`text`) -> anchor item checks pass. Negation, inline code, table
cells, code syntax and original CRLF bytes were checked; failing fetch/model-loader
sentinels saw zero entries. Explicit remote/OCR=true inherited settings do not
change this local model-free route. The cache binds reference, content hash,
route and versions; changed bytes return a different key, stale body/hash pairs
fail even on repeat calls, and identical copies keep separate source references.

No installed package, governed unpinned uvx command or live MCP config was changed.
The provisioned top-level pin alone does not guarantee required backend versions.
Verify preimages, dependency versions, patched tool schema and scratch fixtures,
then obtain independent deployment/startup approval before using it live.

Cached-document fanout query remains available with separate read/startup
permission. It unwraps native search `{result}`, then resolves each returned
anchor with `get_text_of_document_item_at_anchor({document_key,document_anchor})`
and checks `{text}`. No-match text, echoed queries and anchor IDs cannot satisfy
`expectedEvidence.docling`; raw search/item envelopes remain in evidence.
Adjacent context and original provenance still require separate review.

Native isolated authoring passed title/heading/paragraph/list/table/update/
delete/export/save. Headings require section_level1..100; list_items are objects
with list_item_text/list_marker_text; tables require HTML <table>, not Markdown
rows. Invalid table input failed before adding a table. A synthetic PIL page
verified PNG output only, not real PDF rendering or browser correctness.

## Preservation limits

Native Markdown preserves negation, inline code, code-body syntax, table cells
and paragraph structure on the challenge fixture. It reflows tables, omits the
fence language, and has `prov=[]` for these items: anchor IDs are not original
byte spans. Inline code may split text into adjacent items; retrieve the
surrounding negating context, not an isolated positive-looking word. Retain
original bytes/reference/mapping beside derived output when fidelity matters.

No regex source cleaner or semantic/RAG chunker is required for this key/anchor
workflow. Optional native chunkers/visualizers are separate library surfaces
with tokenizer/model preconditions, not nineteen-tool MCP readiness evidence.
