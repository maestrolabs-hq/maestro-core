# maestro-core

The orchestrator agents work under. Maestro delegates all the work and does
none of it, while staying accountable for all of it.

It knows nothing about any particular agent. Clients ask; Maestro answers.

## Layout

```text
crates/protocol     what a client may ask, and what Maestro answers
crates/ledger       durable record of everything Maestro is accountable for
crates/policy       what is allowed and required: handoff contracts,
                    refusals, routing, workflow enforcement
crates/sink         where recorded material goes, over MCP
crates/supervisor   the always-on server
crates/cli          the `maestro` binary
```

## Documents

- [CONTEXT.md](./CONTEXT.md) — glossary
- [docs/supervisor.md](./docs/supervisor.md) — residency, delegation, accountability
- [docs/protocol.md](./docs/protocol.md) — request and answer
- [docs/ledger.md](./docs/ledger.md) — durability, states, retry

## CLI shape

Noun-verb, after herdr. The command is a thin client over the supervisor.

```text
maestro memory   capture | recall | status | drain
maestro status | completion <shell>
```

## Status

Workspace builds. No commands implemented.
