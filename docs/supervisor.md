# The supervisor

Maestro delegates all the work and does none of it. What it keeps for itself is
knowing, at every moment, what was handed out, to whom, whether it came back,
and whether what came back is what was asked for.

That is only possible if it is already running when a request arrives.

## Residency

Maestro is a long-lived process listening on a Unix socket. The `maestro`
command is a thin client: it checks whether the supervisor is listening, starts
it if not, then asks it. Nothing to install; the first command of the day brings
it up.

```text
 pi session A ───┐
 pi session B ───┼──▶ ┌─────────────────────┐
 pi session C ───┤    │  maestro supervisor │
 maestro (CLI) ──┘    │                     │
                      │  · live delegations │
                      │  · outstanding      │
                      │    handoffs         │
                      │  · per-project      │
                      │    state            │
                      └─────────────────────┘
```

Concurrency is the reason for residency, not performance. Several projects run
at once, and a supervisor that only exists during a command cannot hold state
between them.

### The gap this leaves

Nothing restarts the supervisor if it dies. Between a crash and the next
command, no handoff is enforced and nothing is watched — silently. This is
accepted for now in exchange for having no install step; a systemd user unit is
the eventual answer, and `maestro status` must make an absent supervisor
obvious rather than looking idle.

## Never waiting

The supervisor must always be able to accept a request. No operation that talks
to a consumer, a model, or a child agent may block the accept loop. Slow work is
recorded and handed to a worker; the socket stays responsive.

"Never waiting" is a property of the supervisor, not of the caller. A client may
well wait for its answer.

## Delegation and handoff

Work leaves the supervisor as a delegation with a stated expectation of what
comes back. A handoff is the return, checked against that expectation.

A handoff that does not satisfy what was asked is refused, and the delegate stays
blocked until it does. Blocking is the mechanism that makes delegation
accountable: without it, "delegate everything" degrades into "hope everything
comes back".

The shape of a delegation contract is not yet specified.

## Accountability

Every delegation, refusal, handoff and delivery is recorded in the ledger before
it is acted on. The ledger is the record Maestro answers questions from — not
in-memory state, which a crash would take with it.

Visibility is not a reporting feature bolted on afterwards. It is the reason the
ledger is written first.

## Crates

| Crate | Owns |
| --- | --- |
| `wire` | what a client may ask, and what Maestro answers |
| `ledger` | the durable record of everything Maestro is accountable for |
| `policy` | what is allowed and what is required: handoff contracts, refusals, routing, workflow enforcement |
| `sink` | where recorded material goes — memory, observability, bridges — over MCP |
| `supervisor` | the always-on server: listens, holds live state, never waits |
| `cli` | the `maestro` binary; wakes the supervisor, then asks it |

`policy` deliberately holds handoff contracts, refusals and routing together.
All three are rules about what is allowed or required, and their seams are not
yet known. Splitting them now would be guessing; splitting them later is cheap.

The memory queue that earlier drafts made central is one use of `ledger` and one
`sink`. It is not the shape of the system.
