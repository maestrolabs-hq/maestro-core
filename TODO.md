# TODO

Three reviewers went looking for bugs here and found almost none. That is not a
compliment. There are ~200 lines and no command does anything, so there is
nothing to be wrong.

Everything else in the estate exists to serve this repository. Right now it is
the only one that has not started.

---

## P0 -- ship one command end to end

### 1. `maestro memory capture`

`docs/protocol.md` fixes the envelope. `docs/spool.md` fixes the table. Neither
has a caller.

Build the thinnest path that works, test-first:

- accept an envelope on stdin
- validate it against the protocol
- append it to the spool
- exit non-zero if it could not be durably stored

No supervisor, no delivery, no retries. Those are `docs/supervisor.md` and they
stay designs until something is actually queuing.

The point is to stop being a repository of intentions. One command that does
one thing beats four documents describing five that do not exist.

### 2. Then: the smallest thing that drains the spool

Once capture works, a drain proves the envelope survives a round trip. That is
the moment the protocol version stops being a guess.

---

## P1 -- open design questions the docs do not answer

### 0. Should the three verbs move under one `maestro` CLI?

Not yet. Written down here so the question is not re-opened from scratch.

`maestro-pi-config` and `maestro-governance` now share a three-verb contract:

```text
plan     shows what would change, changes nothing
apply    refuses without --auto-approve
destroy  removes only what the tool itself wrote
```

They share no code, only that shape. Two instances is not a pattern -- the
rule of three exists because the second one always looks more alike than it
turns out to be.

Three reasons to wait:

- **This repository has no commands.** A unifying layer would arrive before
  the thing it unifies, which is how the four deleted crates happened.
- **`pi-config` bootstraps a machine.** It has to run before the estate
  exists, so it must not depend on the estate.
- **`governance` has no `destroy`** and probably never will, so the contract
  is not actually uniform yet.

Revisit when this repository has one real verb. At that point there will be a
third user and something to compare.

What is worth carrying across regardless is the guarantee, not the code: show
before acting, refuse without an explicit flag, and never remove what you did
not write.

### 3. "Never waiting" has no worker or concurrency model

The supervisor design says the CLI never waits. Nothing says how many workers
drain the spool, what happens when one wedges, or how a second `maestro`
process on the same machine interacts with the first.

This is the question that decides whether the spool needs row-level locking or
just an append-only file. It should be answered before the table is built, not
after.

### 4. `protocol` is frozen at `v: 1` but scoped to memory

The envelope was designed for one kind of payload and given a version number
that implies it covers all of them. Either widen it deliberately or say it is
memory-specific and let other domains have their own.

---

## How this becomes the best in the world

Blunt: it is not close today, and the gap is not quality -- the gates here are
already better than most funded teams ship. The gap is that **nothing runs**.

The estate has world-class scaffolding around an empty room. Three things would
change that, in order.

### First: earn the right to the claim on one axis

Not "the best agent orchestrator." Pick the one property nobody else offers and
be unarguably first at it:

> **Every delegation is recorded, replayable, and auditable after the fact.**

Nothing in the agent tooling space does this well. Frameworks optimise for
authoring; almost none can answer "what exactly did this agent do last Tuesday,
and would it do the same today." The spool is already the right shape for it --
it is a durable record of handovers, which is the raw material of an audit log.

That is a real gap, it is defensible, and it is worth more the longer it runs.

### Second: make the estate itself the proof

Everything already built is the argument. An orchestration tool whose own
repositories carry gates most companies do not have -- an org-wide prose gate,
content-pinned shared files, mutation testing, SLSA provenance, a drift audit
that opens its own issues -- is credible in a way a README cannot be.

That story only holds if the estate is honest. It was not, six hours ago: docs
described a supervisor that does not exist. It is honest now because a review
made it so. Keep it that way -- **the estate's credibility is a load-bearing
feature, not a nicety.**

Concretely: the fleet audit should publish. A public, machine-readable
statement of what this org enforces, updated weekly by the audit that already
runs, is something almost nobody publishes because almost nobody could.

### Third: be the thing people reach for at 2am

The best tools win incidents. When an agent does something unexpected, the
question is always the same: *what happened, in what order, and why.*

If `maestro replay <id>` answers that in one command, adoption follows without
marketing. Everything in the design -- durable spool, fixed envelope, recorded
handovers -- already points at this. It has simply not been built.

### What would make this fail

- **Building the supervisor before the spool has a caller.** The most likely
  failure is another six months of excellent documents.
- **Widening scope before one command is loved.** Memory, then nothing else,
  until memory is boring.
- **Letting the docs drift ahead of the code again.** That already happened
  once. The prose gate catches vocabulary; nothing catches "this paragraph
  describes software that does not exist." A test that fails when a doc claims
  a command the CLI does not expose would.
