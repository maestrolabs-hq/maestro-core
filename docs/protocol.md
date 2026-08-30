# Protocol

The contract between a child project and Maestro: one envelope in, one
acknowledgement out.

Two processes sit on the child's side of that exchange, and confusing them is
easy:

```text
child ──spawns──▶ maestro (CLI) ──socket──▶ maestro (supervisor)
                  short-lived,               resident, owns the ledger
                  one per exchange           and the delivery schedule
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

```json
{
  "v": 1,
  "kind": "capture",
  "source": { "id": "pi", "session": "01a04e5c-…" },
  "project": { "root": "<absolute path to the project root>" },
  "material": { "path": "<absolute path to the material>" }
}
```

| Field | Meaning |
| --- | --- |
| `kind` | `capture` or `recall` |
| `source.id` | opaque label for the child. Maestro records it and never branches on it |
| `source.session` | groups captures and anchors the watermark |
| `project.root` | determines the scope |
| `material.path` | where the material is. Only for `capture` |

There is deliberately no field naming the event that produced the envelope. A
capture is a capture; Maestro has no reason to know whether a session was
settling, compacting or tearing down, and giving it that knowledge would put
child-specific behaviour in Maestro.

A `recall` envelope carries `kind`, `source` and `project` only.

## Acknowledgement

Capture, once the material is durable:

```json
{ "v": 1, "ok": true, "id": 4127 }
```

Recall:

```json
{ "v": 1, "ok": true, "content": "…", "tokens": 812 }
```

Refusal:

```json
{ "v": 1, "ok": false, "error": "unsupported envelope version: 2" }
```

`ok: true` on a capture means durable, not delivered. Delivery happens after
the child has already been released.

## Timing

A child waits for the acknowledgement under its own hard timeout and treats a
timeout as a non-event. Maestro must therefore acknowledge as soon as the write
is durable and never hold the pipe open for delivery.

Delivery happens in the supervisor, after the CLI has already exited and the
child has already been released. Nothing about a slow or unreachable sink can
reach back into the exchange.
