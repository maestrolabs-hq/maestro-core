# Protocol

The contract between a child project and Maestro: one envelope in, one
acknowledgement out.

Two processes sit on the child's side of that exchange, and confusing them is
easy:

```text
child ──spawns──▶ maestro (CLI) ──socket──▶ maestro (supervisor)
                  short-lived,               resident, owns the ledger
                  one per exchange
```

The CLI is spawned per exchange and exits. The supervisor does not: it is the
long-lived process described in [supervisor.md](./supervisor.md), and the CLI
starts it only when it is not already listening.

A child speaks to the CLI over stdin and stdout and needs to know nothing about
the socket behind it. That is the point of the split — the child's contract
stays a pipe even though the system behind it is resident.

`v` is present on every message. A child that sends `v` Maestro does not know
gets a refusal, never a guess.

## Envelope

No message kind is defined yet. `v` is the only field this document commits
to: every message carries it, and Maestro refuses a version it does not
recognize rather than guessing at what an unknown message means.

```json
{ "v": 1 }
```

Refusal:

```json
{ "v": 1, "ok": false, "error": "unsupported envelope version: 2" }
```

The first real command defines its own request and acknowledgement shape.
Writing one down ahead of a caller is how a protocol ends up frozen around a
guess; better to let the shape follow the need.

## Timing

A child waits for the acknowledgement under its own hard timeout and treats a
timeout as a non-event. Maestro must therefore acknowledge as soon as the work
it is accountable for is durable, and never hold the pipe open past that
point.
