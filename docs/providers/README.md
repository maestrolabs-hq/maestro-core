# Providers

This directory contains one page per provider. Provider-specific vocabulary is
allowed here only; this documented boundary is exempted by the vocabulary gate.

All providers use the HOME-derived direct state root
`<workspace>/.maestro/state/providers/{cgc,graphify,mempalace,semantica}`.
The `<workspace>` placeholder is resolved at runtime and never names a machine
path.

Providers remain logically independent: each has a separate identity, index,
score, and result set. Results are never merged. Each provider uses one direct
active state; there is no staging, generations, promotion, proxy, or extra hash
workflow.

Graphify and MemPalace keep their native homes under the shared root. Filesystem
aliases at `~/.graphify` and `~/.mempalace` preserve compatibility because some
native paths are not configurable.

| Provider page | Role | Direct state path |
| --- | --- | --- |
| [CGC](./cgc.md) | Structural code graph and AST queries | `<workspace>/.maestro/state/providers/cgc/kuzudb` |
| [Graphify](./graphify.md) | Portable repository graph and optional semantic extraction | `<workspace>/.maestro/state/providers/graphify/` |
| [MemPalace](./mempalace.md) | Durable local memory and temporal knowledge graph | `<workspace>/.maestro/state/providers/mempalace/` |
| [Semantica](./semantica.md) | Semantic context graph, decisions, and reasoning | `<workspace>/.maestro/state/providers/semantica/global-graph.json` |

[Semantica evaluation evidence note](./semantica-evaluation.md)
