# Providers

Seven independent provider integrations. Provider vocabulary is scoped to this
directory by the vocabulary gate. Identities, graphs, indexes, scores and
results are never merged. A fanout returns labeled, scoped evidence, not one
combined answer.

| Provider | Role | Observed state boundary |
| --- | --- | --- |
| [CGC](cgc.md) | Structural code/Cypher/complexity | `<workspace>/.maestro/state/providers/cgc/kuzudb` |
| [CodeGraph](codegraph.md) | Native structural source/context | Per-project `.codegraph`; only core aliased to shared provider root in this audit |
| [Codebase-Memory](codebase-memory.md) | Structural queries, source, traces, explicit ADR document | Configured provider cache plus separate daemon/runtime state |
| [Graphify](graphify.md) | Local portable AST graph, optional authorized semantic extraction | Per-repo graphify-out plus separately served global artifact; global community/ownership fidelity degraded |
| [Semantica](semantica.md) | Native context graph, decisions, literal retrieval, reasoning | One direct active provider JSON; source-grounded extraction/durable refresh not yet wired |
| [MemPalace](mempalace.md) | Retained historical/temporal memory | Native home alias to its own provider state; production remains unchanged |
| [Docling](docling.md) | Structured conversion/anchors/authoring, no repository index | Model artifacts, in-process documents, and separate save/thumbnail CACHE_DIR outputs |

`<workspace>` is derived at runtime; it is a container, not a Git repository.
The eight canonical roots are dot-github, maestro-core, maestro-governance,
maestro-herdr-config, maestro-llamacpp, maestro-manifests, maestro-pi-config and
maestro-project-documentation. Current branches/dirty eligible source are valid;
main-branch history or retained memory is not current-checkout truth.

Graphify and MemPalace native homes are filesystem aliases under the direct
provider-state root. CodeGraph state relocation is not fully deployed. There
is no new provider proxy, staging/generation/promotion system or fused graph.
Unique scratch fixtures and patch preimage hashes validate repairs, not an
additional permanent production state architecture.

- [Capabilities](capabilities.md): complete per-tool/schema evidence ledger.
- [Preparation](preparation.md): every applicable native preparation stage,
  preservation requirements, unsafe cleaner bypasses and untested paths.
- [Validation](validation.md): exact bounded successes/failures, guarded fixes,
  runnable tests, remaining quality/production decisions.

The tracked provider-fanout skill/helper and reproducible patch artifacts live
in maestro-pi-config. They have been tested in isolated state, not deployed to
live skills/packages. Production MCP gateway remains untested; cached metadata
and disconnected servers are not a live validation. No model substitution or
production MemPalace write is authorized by these documents.
