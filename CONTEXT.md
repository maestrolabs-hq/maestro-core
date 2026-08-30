# Context

Glossary for maestro-core. Terms only — no implementation detail.

Maestro knows nothing about any particular agent. Nothing in this glossary
names one.

## Maestro

A framework agents run under, and the master orchestrator of agent work. It
governs behaviour — refusing destructive commands, running sanity checks,
monitoring, observability, carrying messages outward — and it owns the memory
path from capture through to delivery.

## Child project

Anything that consumes Maestro. A child sends envelopes and reads
acknowledgements; it holds no orchestration logic of its own.

## Envelope

The versioned JSON message a child sends. It states what the child wants —
material captured, or material recalled — and never why. Maestro does not learn
what happened in the child that produced it.

## Acknowledgement

Maestro's reply to an envelope, on stdout. For a capture it means the material
is durable. For a recall it carries the recalled material.

## Capture

Material accepted into the durable queue. A capture is complete once durable,
before any consumer has seen it. This is the point the acknowledgement reports.

## Durable queue

The record of captures accepted but not yet delivered. It is why a consumer
being unavailable cannot lose material, and it is the reason Maestro sits
between a child and a consumer rather than being bypassed.

## Watermark

Per-session marker of how far capture has already reached, so a child sends
only what is new.

## Consumer

A downstream store Maestro delivers to: MemPalace, CodeGraphContext, Graphify.
Consumers are reached over MCP. A consumer is never a runtime dependency —
Maestro accepts and acknowledges captures whether or not any consumer is
reachable.

## Delivery

Handing a captured item to a consumer. Happens after acknowledgement, and may
fail without losing the capture.

## Dead letter

A capture whose delivery attempts are exhausted. It is parked, stays visible,
and can be drained again. A capture is never dropped.

## Recall

Returning previously captured material. Bounded, because the requester spends
its own context on the result.

## Wing / Room

How captured material is filed. A wing corresponds to one project; a room
subdivides it by kind. Recall is scoped to a wing.
