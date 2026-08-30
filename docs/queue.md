# Durable queue

Embedded SQLite. Written before acknowledgement, drained afterwards.

## Schema

```sql
CREATE TABLE capture (
  id              INTEGER PRIMARY KEY,
  created_at      TEXT    NOT NULL,
  source_id       TEXT    NOT NULL,
  session         TEXT    NOT NULL,
  wing            TEXT    NOT NULL,
  payload         BLOB    NOT NULL,
  state           TEXT    NOT NULL,   -- pending | delivered | dead
  attempts        INTEGER NOT NULL DEFAULT 0,
  next_attempt_at TEXT,
  last_error      TEXT
);

CREATE INDEX capture_drainable ON capture (state, next_attempt_at);

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

There is no daemon. Each capture drains opportunistically: after acknowledging,
the same process attempts delivery of due rows, then exits. A session that never
captures again leaves rows pending until the next session drains them, which is
the correct trade for not running a background process.

`maestro memory drain` forces a drain, and is the recovery path when a consumer
was down for a long time.

## Retry

Exponential backoff on `next_attempt_at`, bounded attempts. On exhaustion the
row moves to `dead` — parked, still present, still drainable once the consumer
is healthy.

The retry rate is bounded; retention is not. Dropping a capture because a
consumer misbehaved would reintroduce the silent loss this queue exists to
prevent.

## Visibility

A child never sees a delivery failure — it was released at acknowledgement.
`maestro memory status` is therefore the only place queue depth, dead letters
and last error are observable, which makes it load-bearing rather than
cosmetic.
