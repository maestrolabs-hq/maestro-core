# maestro-core

The engine of the Maestro framework, and the master orchestrator of agent work.

Maestro governs agent behaviour — refusing destructive commands, sanity checks,
monitoring, observability, outward bridges — and owns the memory path from
capture through to delivery.

It knows nothing about any particular agent. Children send envelopes; Maestro
answers.

## Layout

```text
crates/protocol   envelope types and versioning
crates/queue      durable SQLite queue
crates/delivery   delivery to consumers over MCP
crates/cli        the `maestro` binary
```

## Documents

- [CONTEXT.md](./CONTEXT.md) — glossary
- [docs/protocol.md](./docs/protocol.md) — envelope and acknowledgement
- [docs/queue.md](./docs/queue.md) — durability, states, retry

## CLI shape

Noun-verb, after herdr:

```text
maestro memory   capture | recall | status | drain
maestro status | completion <shell>
```

## Status

Workspace builds. No commands implemented.
