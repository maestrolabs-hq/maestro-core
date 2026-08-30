# maestro-core

The orchestrator agents work under. Maestro delegates all the work and does
none of it, while staying accountable for all of it.

It knows nothing about any particular agent. Clients ask; Maestro answers.

## Layout

```text
crates/protocol   what a client may ask, and what Maestro answers
crates/cli        the `maestro` binary
```

Two crates. There were six: `policy`, `sink`, `supervisor` and `ledger` were
each created before anything needed them, and a seam is only real when
something varies across it. Nothing varied, and every one of them held zero
lines.

The designs are still here, in `docs/`. They are the part that was worth
keeping. Crates come back when code asks for them, which is also when their
boundaries and their names will be known rather than guessed.

## Documents

- [CONTEXT.md](./CONTEXT.md) — glossary
- [docs/supervisor.md](./docs/supervisor.md) — residency, delegation, accountability (design; no crate yet)
- [docs/protocol.md](./docs/protocol.md) — request and answer
- [docs/ledger.md](./docs/ledger.md) — durability, states, retry (design; no crate)

## CLI shape

Noun-verb, after herdr. The command is a thin client over the supervisor.

```text
maestro memory   capture | recall | status | drain
maestro status | completion <shell>
```

## Status

Workspace builds. No commands implemented.
