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

Nothing restarts the supervisor if it dies. In practice any active client
revives it quickly — an agent session touches the supervisor when it starts and
several times while it runs — so a crash is usually corrected within one turn of
any running session.

The gap that remains is narrow but real: when no client is running, nothing
revives it. That matters only when Maestro is holding work with no active
session, which is also when it matters most. A systemd user unit is the eventual
answer. Until then `maestro status` must make an absent supervisor obvious
rather than letting it look idle.

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

The shape of a delegation contract is not yet specified, and is deliberately
deferred. Nothing else in this document depends on it.

Whether a client's own main agent may do work directly, rather than handing
everything to a child, is a workflow rule Maestro enforces — not a property of
the supervisor. It is a policy rule, alongside the others about what is
required.

## Accountability

Every delegation, refusal and handoff is recorded in the ledger before it is
acted on. The ledger is the record Maestro answers questions from — not
in-memory state, which a crash would take with it.

Visibility is not a reporting feature bolted on afterwards. It is the reason the
ledger is written first.

## Crates

Two exist:

| Crate | Owns |
| --- | --- |
| `protocol` | what a client may ask, and what Maestro answers |
| `cli` | the `maestro` binary |

The supervisor, the ledger, its policy and its sinks are described in this
document and are not crates. They were, briefly: six crates were created
before anything needed them, and a seam is only real when something varies
across it. Nothing did. They come back when code asks for them, which is also
when their boundaries will be known rather than guessed.

Handoff contracts, refusals and routing are all rules about what is allowed or
required. Whether they are one module or three is not yet knowable, and guessing
produced three empty crates once already.
