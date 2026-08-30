# Ledger

Embedded SQLite. Written before acknowledgement, drained afterwards.

## Scope: delivery only

This schema covers material accepted and awaiting delivery, and nothing else.
The name is `/var/spool/mail`'s: a spool is where things wait to be sent.

The glossary also calls the ledger the record of delegations, refusals and
handoffs. Those are not here and cannot be added to this table -- there is no
discriminator, the state machine is a delivery lifecycle, and there is no
column relating a handoff to the delegation it answers. They will need their
own tables, designed alongside the delegation contract that is currently
deferred. Saying so here is cheaper than a schema that silently claims to
cover them.

## Schema

```sql
CREATE TABLE spool (
  id              INTEGER PRIMARY KEY,
  created_at      TEXT    NOT NULL,
  source_id       TEXT    NOT NULL,
  session         TEXT    NOT NULL,
  scope           TEXT    NOT NULL,
  payload         BLOB    NOT NULL,
  state           TEXT    NOT NULL,   -- pending | delivered | dead
  attempts        INTEGER NOT NULL DEFAULT 0,
  next_attempt_at TEXT,
  last_error      TEXT
);

CREATE INDEX spool_due ON spool (state, next_attempt_at);

CREATE TABLE watermark (
  session    TEXT PRIMARY KEY,
  position   TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
```

`payload` is the captured material, not the envelope. The envelope is a
transport concern and is not worth keeping.

## Durability

The database runs in WAL mode with `synchronous = FULL`. A capture is
acknowledged only after its transaction commits, so an acknowledgement means
the row survives power loss. Anything weaker makes the acknowledgement a
guess.

## States

```text
pending ──delivered──▶ delivered
   │
   └──attempts exhausted──▶ dead
```

A row leaves `pending` only by being delivered or by exhausting its attempts.
Nothing deletes a row on failure.

## Draining

The supervisor drains. It holds the only handle on this database, which is what
keeps a single writer against WAL rather than several processes contending for
it, and it schedules retries without anything else having to stay alive.

Draining never happens on the path that acknowledges. A capture is durable and
the child released before delivery is attempted, so a slow or unreachable sink
cannot reach back into an exchange.

When the supervisor is not running, nothing drains. Rows stay pending, which is
what pending means, and the next client to wake the supervisor picks them up.
That is the cost of the residency model in
[supervisor.md](./supervisor.md): a machine with no client activity is also a
machine doing no delivery.

`maestro memory drain` forces a drain, and is the recovery path when a sink was
down for a long time.

## Retry

Exponential backoff on `next_attempt_at`, bounded attempts. On exhaustion the
row moves to `dead` — parked, still present, still drainable once the sink
is healthy.

The retry rate is bounded; retention is not. Dropping a capture because a
sink misbehaved would reintroduce the silent loss this queue exists to
prevent.

## Visibility

A child never sees a delivery failure — it was released at acknowledgement.
`maestro memory status` is therefore the only place queue depth, dead letters
and last error are observable, which makes it load-bearing rather than
cosmetic.
