# Context

Glossary for maestro-core. Terms only, no implementation detail.

Maestro knows nothing about any particular agent, and nothing here names one.
Nothing here names a sink implementation either -- a sink you can name in the
engine is a sink you cannot swap, and a test enforces it.

**Most of this is design, not code.** Two crates exist, `protocol` and `cli`,
and no command is implemented. Every term below marked *(designed)* describes
something written down in `docs/` and not built. Saying so is cheaper than a
reader discovering it.

## Maestro

The orchestrator agents work under. It delegates all the work and does none of
it, while remaining accountable for all of it: what was handed out, to whom,
whether it returned, and whether what returned is what was asked for.

## Supervisor *(designed)*

The always-on process. It listens, holds live state across concurrently running
projects, and must always be able to accept a request. Residency exists for
concurrency, not for speed: state that spans projects cannot live in a process
that only exists during a command.

## Never waiting *(designed)*

A property of the supervisor, not of its callers. No work — a consumer, a model,
a child agent — may block the accept loop. Slow work is recorded and handed to a
worker. A caller may still wait for its own answer.

## Client

Anything that talks to the supervisor: the `maestro` command, or a shim inside
an agent. A client holds no orchestration logic. It wakes the supervisor if it
is not listening, then asks.

## Delegation

Work handed out, carrying a stated expectation of what must come back.
Delegating without stating the expectation is how accountability is lost.

## Handoff

The return of delegated work, checked against the expectation the delegation
stated.

## Refusal

Maestro declining something: a handoff that does not satisfy what was asked, a
destructive command, a step that breaks a workflow. A refused delegate stays
blocked until it satisfies the contract. Blocking is what separates delegation
from hope.

## Ledger *(designed)*

The durable record of what Maestro is accountable for: delegations, refusals
and handoffs, written before the thing they record is acted on, so a crash
cannot erase what was promised. The ledger, not memory, is what Maestro
answers questions from.

Its shape is deferred until a delegation contract exists to give it one.
