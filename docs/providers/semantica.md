# Semantica

Semantica is an independent semantic context-graph provider. It extracts entities
and relationships from repository content, records decisions and precedents,
and answers graph queries and reasoning requests. Its identity, graph, scores,
and result sets are separate from CGC, Graphify, and MemPalace; no provider
results are merged.

## What it does and why Maestro uses it

Semantica provides semantic graph search and reasoning alongside the structural
CGC graph, the repository Graphify graph, and durable MemPalace memory. The
initial workspace-global graph is seeded only from `maestro-core` through
Semantica's native repository ingestion. The provider remains replaceable and
its vocabulary stays within this documented boundary.

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

## Identity

| Field | Value |
| --- | --- |
| Package | `semantica` **pinned 0.6.7** (MIT; uv tool) |
| MCP server | `semantica-mcp` (direct stdio server) |
| Active graph | `<workspace>/.maestro/state/providers/semantica/global-graph.json` |
| Initial corpus | `maestro-core` only, ingested through the native repository API |
| Scope | Workspace-global, with an identity independent of CGC and Graphify |

The Python package has a large dependency footprint spanning scientific
computing, NLP/ML, document parsing, and graph/vector integrations. Semantica
has no native multi-repository registry, so this deployment uses its single
configured file as the workspace-global graph.

## Wiring

```json
"semantica": {
  "command": "semantica-mcp",
  "env": {
    "SEMANTICA_KG_PATH": "<workspace>/.maestro/state/providers/semantica/global-graph.json"
  }
}
```

The workspace path is derived from `HOME`; the placeholder above is not a
machine-specific path. The server uses stdio JSON-RPC and keeps one in-memory graph per process,
loading the configured JSON graph at startup from `SEMANTICA_KG_PATH`. The
direct native server is exposed as installed; no proxy or extra provider layer
is used.

## Skills and Pi integration

No Pi-specific skill was identified in the reviewed 0.6.7 sources, and none is
installed. No provider-specific Pi extension is installed. Semantica is direct MCP
only in this phase; its MCP server is a provider interface, not a Pi skill.

## MCP tools (15 in wheel 0.6.7)

| Tool | Description | Tested |
| --- | --- | --- |
| `extract_entities` | Extract entities from supplied content. | not exercised |
| `extract_relations` | Extract relationships from supplied content. | not exercised |
| `record_decision` | Record a decision and its context. | not exercised |
| `query_decisions` | Query recorded decisions. | not exercised |
| `find_precedents` | Find related prior decisions or cases. | not exercised |
| `get_causal_chain` | Return a causal chain from graph context. | not exercised |
| `add_entity` | Add an entity to the graph. | not exercised |
| `add_relationship` | Add a relationship to the graph. | not exercised |
| `run_reasoning` | Run reasoning over graph context. | not exercised |
| `get_graph_analytics` | Return graph analytics. | not exercised |
| `export_graph` | Export the semantic graph. | not exercised |
| `get_graph_summary` | Return graph summary information. | verified |
| `query_graph` | Query the graph; use `mode: "search"` for semantic search. | verified |
| `update_node` | Update a graph node. | not exercised |
| `delete_node` | Delete a graph node. | not exercised |

## Current state and limitations

The direct initial graph is non-empty and `query_graph` with `mode: "search"`
returns results for the ingested repository. The wheel exposes 15 tools; it
does not expose a tool literally named `search_graph`.

Native mutation durability is inconsistent. `add_entity` and
`add_relationship` do not persist their changes across a server restart.
`update_node` and `delete_node` write directly when `SEMANTICA_KG_PATH` is set,
without an atomic write protocol. These limitations are documented rather than
fixed in this direct integration phase.

Semantica's JSON graph is not a ledger and does not replace Maestro durability.
Keep inference and graph storage local by using the configured local provider
and local graph path; remote model or graph backends are outside this
integration.

## Extraction wiring

Entity and relation extraction requires a spaCy model. `NERExtractor` defaults
to `method="ml"` and hard-defaults to `en_core_web_sm`; with no model installed
it silently falls back to a naive pattern stub (entities labeled `UNKNOWN`,
relations `related_to`). That fallback is a wiring gap, not a tool defect.

This deployment was unwired until 2026-09-05. `en_core_web_md` (which carries
vectors) and `en_core_web_sm` are now installed in the semantica uv tool venv,
and extraction was verified producing real labels (PERSON/ORG/GPE) and
dependency-based predicate relations. Pass `model="en_core_web_md"` explicitly;
the extractor otherwise picks `_sm`.

Two caveats on durability and setup:

- The models live only in the tool venv; a `uv tool` reinstall of semantica
  wipes them. Install with `uv pip install --python <semantica tool venv
  python> <model wheel>` — `python -m spacy download` does not work in a uv
  tool environment.
- `get_graph_analytics` fails on 0.6.7: PageRank calls `graph.nodes()` but
  `ContextGraph.nodes` is a dict ("'dict' object is not callable"). A local
  venv hotfix addresses it; the patch is not durable across reinstall and is
  upstream-reportable.

Optional higher-quality extraction uses `method="llm"` through
`OpenAIProvider(base_url=...)` pointed at the estate's local llama.cpp
endpoint (`http://127.0.0.1:8080/v1`, model `qwen38-semantic`); it needs the
`openai` package and a non-empty `OPENAI_API_KEY` (any value). The MCP
`extract_entities`/`extract_relations` schemas do not forward `base_url`, so
LLM extraction runs through the Python API or an `ExtractionConfig` file, not
per-MCP-call. The embeddings stack (sentence-transformers, fastembed, torch)
is already present and semantic search works.
