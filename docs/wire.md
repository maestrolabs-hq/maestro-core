# Wire

The contract between a child project and Maestro. One envelope in on stdin,
one acknowledgement out on stdout, one process per exchange.

`v` is present on every message. A child that sends `v` Maestro does not know
gets a refusal, never a guess.

## Envelope

```json
{
  "v": 1,
  "kind": "capture",
  "source": { "id": "pi", "session": "01a04e5c-…" },
  "project": { "root": "/home/franc/workspace/MaestroLabs/maestro-pi-config" },
  "material": { "path": "/home/franc/.pi/agent/sessions/…/….jsonl" }
}
```

| Field | Meaning |
| --- | --- |
| `kind` | `capture` or `recall` |
| `source.id` | opaque label for the child. Maestro records it and never branches on it |
| `source.session` | groups captures and anchors the watermark |
| `project.root` | determines the wing |
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
