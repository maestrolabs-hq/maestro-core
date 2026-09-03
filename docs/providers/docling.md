# Docling (MCP)

Docling MCP exposes document conversion as MCP tools: convert a document,
cache the result, and read or edit the resulting structured document. Unlike
the six graph and memory providers on this page, it holds no repository
index — its only persistent state is the model-weight cache it shares with
the `docling` CLI.

## What it does and why Maestro uses it

`docling-mcp` wraps the same conversion pipelines as the `docling` CLI
(documented in [`docs/tools/docling.md`](../tools/docling.md)) behind an MCP
server: convert a document into a cached `DoclingDocument`, export it to
Markdown, inspect or edit its structure by anchor, and assemble new documents
programmatically. Maestro uses it so an agent can convert arbitrary source
documents — specs, vendor PDFs, papers — into structured, queryable text over
MCP rather than shelling out, while keeping conversion entirely local.

## Vocabulary mapping

| Docling MCP term | What it means | Our term (protocol / estate) |
| --- | --- | --- |
| local document cache | in-process cache of converted `DoclingDocument`s, keyed by a hash of the source | conversion cache (process-lifetime, not persisted to disk) |
| document key | the cache key for a converted document | conversion cache key |
| anchor (`#/texts/2`) | a stable reference to one item in a document's structure | structural reference |
| conversion mode (local / remote) | run conversion in-process, or delegate to a hosted `docling-serve` instance | conversion backend selection |
| `docling-mcp[local]` extra | the package variant that bundles the `docling` conversion library itself | local conversion capability |
| generation / manipulation tools | tools that build or edit a `DoclingDocument` from scratch, item by item | document assembly tools |

## Identity

| Field | Value |
| --- | --- |
| Upstream | `docling-project/docling-mcp` (MIT) |
| Distribution | `docling-mcp[local]` 3.2.0, run via `uvx --from=docling-mcp[local] docling-mcp-server` (no persistent install; `uvx` resolves and caches the environment on each invocation) |
| Holds no repository index | Docling MCP has no `<workspace>/.maestro/state/providers/` entry; its state is exactly the model-weight directory the `docling` CLI also uses |
| Model artifacts | `~/models/document/parsing/docling` — the operator's model estate, shared with the CLI, not workspace state (see `docs/tools/docling.md` for what was and was not downloaded, and why) |
| Conversion mode | local (in-process), forced explicitly — the package defaults to `remote` and requires a configured `docling-serve` URL otherwise |

## Wiring

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

Three settings are load-bearing and easy to get wrong:

- `--transport stdio` must be passed explicitly. The server's own default
  transport is `streamable-http` on `localhost:8000`, which does not speak
  MCP-over-stdio at all.
- The `[local]` extra is required. Plain `docling-mcp` does not depend on the
  `docling` package; without it, `conversion_mode=local` raises `ImportError`
  and every conversion call fails.
- `DOCLING_MCP_DO_OCR=false` matches the CLI deployment's choice to skip
  RapidOCR (see `docs/tools/docling.md`): OCR defaults to on, and turning it
  on here without the RapidOCR artifacts present triggers the same missing-
  checkpoint error the CLI hit before that flag was added.

`DOCLING_SERVICE_URL` and `DOCLING_CONVERSION_MODE`/`DOCLING_MCP_*` remote
settings are deliberately unset — conversion stays local, never delegating to
a hosted `docling-serve` instance.

## Skills and Pi integration

No provider-supplied Pi skill was identified in the installed distribution.
No provider-specific Pi extension is installed. Docling MCP is used through
its native MCP server only in this deployment.

## MCP tools (19; default `conversion`, `generation`, `manipulation` sets)

| Tool | Description | Tested |
| --- | --- | --- |
| `is_document_in_local_cache` | Check whether a document key is already converted and cached. | verified |
| `convert_document_into_docling_document` | Convert a document from a URL or local path, caching the result. | verified |
| `convert_directory_files_into_docling_document` | Convert every file in a local directory. | not exercised |
| `create_new_docling_document` | Start a new, empty document from a prompt string. | not exercised |
| `export_docling_document_to_markdown` | Export a cached document to Markdown. | verified |
| `save_docling_document` | Save a cached document to disk as Markdown and JSON. | skipped (writes outside the cache; not exercised to avoid touching disk during verification) |
| `page_thumbnail` | Render a thumbnail image for one page of a cached document. | not exercised |
| `add_title_to_docling_document` | Set or update a cached document's title. | not exercised |
| `add_section_heading_to_docling_document` | Insert a section heading into a cached document. | not exercised |
| `add_paragraph_to_docling_document` | Append a paragraph to a cached document. | not exercised |
| `open_list_in_docling_document` | Start a new list in a cached document. | not exercised |
| `close_list_in_docling_document` | Close the current list in a cached document. | not exercised |
| `add_list_items_to_list_in_docling_document` | Add items to an open list. | not exercised |
| `add_table_in_html_format_to_docling_document` | Insert an HTML-defined table into a cached document. | not exercised |
| `get_overview_of_document_anchors` | Return the structural outline of a cached document, by anchor. | not exercised |
| `search_for_text_in_document_anchors` | Search a cached document's text by anchor. | not exercised |
| `get_text_of_document_item_at_anchor` | Read the text of one item by anchor. | not exercised |
| `update_text_of_document_item_at_anchor` | Replace the text of one item by anchor. | skipped (mutating) |
| `delete_document_items_at_anchors` | Delete one or more items by anchor. | skipped (destructive) |

## Notes and limitations

- The local document cache is in-process and process-lifetime only: nothing
  is written to disk unless `save_docling_document` is called explicitly.
  Restarting the MCP server empties the cache.
- `uvx --from=docling-mcp[local]` re-resolves the environment on every server
  start rather than using a fixed `uv tool install`, matching the upstream
  MCP wiring example; this trades a small startup cost for always matching
  the latest `docling-mcp[local]` release rather than a pinned one.
- Verification reused the `docling` CLI's shared model-artifacts directory
  and did not download or store anything beyond it.
