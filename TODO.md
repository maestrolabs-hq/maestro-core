# TODO

Three reviewers went looking for bugs here and found almost none. That is not a
compliment. There are ~200 lines and no command does anything, so there is
nothing to be wrong.

Everything else in the estate exists to serve this repository. Right now it is
the only one that has not started.

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
handle recorded work, what happens when one wedges, or how a second `maestro`
process on the same machine interacts with the first.

This is the question that decides how the ledger is stored -- row-level
locking, an append-only file, or something else. It should be answered before
any of it is built, not after.

---

## P1 -- documents ahead of the code, and gates blind to it

### 5. `docs/supervisor.md` lists three crates; two exist, and the gate is blind

`docs/supervisor.md:82-90` has a table headed "Crates" naming `protocol`,
`ledger` and `cli`. `ls crates/` returns `cli` and `protocol`. `README.md:29`
says the `ledger` crate was deleted; `docs/ledger.md:3` says "No crate
implements this."

The gate written for exactly this defect passes. `crates/cli/tests/vocabulary.rs:134`
carries the doc comment *"Deleting three crates left `supervisor.md` still
listing them in a table headed 'Crates', and every gate stayed green."*
Proved by injection into a scratch copy:

```text
INJECTION A -- a fictional crate named in the table's own format,
               as a backticked bare name in a markdown row:
  test no_document_names_a_crate_that_does_not_exist ... ok        <- passes

INJECTION B -- the same fictional crate named as a directory path,
               in an ordinary sentence:
  test no_document_names_a_crate_that_does_not_exist ... FAILED    <- catches
```

`crates/cli/tests/vocabulary.rs:157` splits each line on the directory prefix
and reads the following word, so it sees a path reference and nothing else. The
markdown-table form it was written to catch is the one form it misses.

(The injected strings are described here rather than quoted, because of #5a.)

**Fix:** also parse the "Crates" table rows; add the table form as a test case.

### 5a. The gate cannot be documented without being tripped

The check is line-based with no awareness of fenced code blocks, so a document
that quotes a path reference -- including a finding explaining what the gate
misses -- fails it. The first write-up of #5 did exactly that:

```text
$ cargo test --all-targets
test no_document_names_a_crate_that_does_not_exist ... FAILED
  TODO.md:111: names a crate that does not exist
test result: FAILED. 3 passed; 1 failed
```

`tests/language.rs` solves the same self-reference problem properly: it builds
its accent ranges from code points so the file cannot fail its own test. This
gate has no equivalent.

**Why it matters:** the estate's stated practice is to record a finding where
the next reader will look. One of its gates mechanically forbids recording this
particular finding, which is how a blind spot stays undocumented.

**Fix:** track fence toggles while iterating lines and skip fenced content.

### 6. The context name every document gives does not exist

`AGENTS.md:93` and `docs/quality-bar.md:98` name the context
`fast / cross-platform`. A matrix job reports one context per leg:

```text
$ gh api repos/maestrolabs-hq/maestro-core/actions/runs/33391234014/jobs --jq '.jobs[].name'
heavy / cross-platform (macos-latest)
heavy / cross-platform (windows-latest)
```

`AGENTS.md:142` warns about precisely this -- *"Renaming a CI job renames a
required context. The ruleset naming the old one blocks every pull request
until it is updated."* Anyone acting on the documents and adding
`fast / cross-platform` verbatim creates a context that can never report,
blocking every pull request indefinitely.

**Fix:** write both leg names in both documents.

### 7. `docs/quality-bar.md` "Not yet in place" describes an estate that is gone

The section lists as missing several controls that now exist, and omits ones
that do not.

**Fix:** regenerate the section against the current fast and heavy tiers.

### 8. `cross-platform` does not exercise the path derivation ADR-0001 protects

The job builds and tests on Windows and macOS, which is worth having. But the
path-derivation code ADR-0001 exists to protect has no test that runs a derived
path on a non-Unix layout, so the matrix proves compilation rather than
behaviour.

**Fix:** a test that derives a path from a synthetic Windows-shaped environment
and asserts the result, running on every platform.

### 9. `just check` is not "the same commands" CI runs

Both `justfile` and `CONTRIBUTING` claim parity. The fast tier runs prose,
brief, markdown, TOML, secrets, actions-security and no-absolute-paths; `just
check` runs none of them.

**Fix:** narrow the claim, or add the language-agnostic gates to `just check`.

### 10. `no-absolute-paths` does not refuse a bare drive letter

The pattern refuses a drive letter only when the first segment is the Windows
user root. A drive letter followed by any other directory -- the canonical
Windows form of the failure ADR-0001 records -- passes. `AGENTS.md:88` and
`docs/quality-bar.md:96` both say it refuses "a drive letter".

(This entry deliberately describes the patterns rather than quoting them: the
gate scans every tracked file, so a document that spells out its own trigger
fails it. The same is true of the workflow that defines them, which is why that
one file is excluded.)

An uppercase-anchored pattern requiring a capital drive letter, a separator and
a path character was probed against all four repositories and is clean, while
catching every non-user-root drive path tried. It false-positives on a
format string ending in a capitalised word before an escape, which is the
tradeoff to weigh.

**Fix:** extend the pattern, or narrow both documents to "a home directory or a
user profile".

### 11. `protocol.md`, `supervisor.md` and `CONTEXT.md` state unbuilt behaviour as fact

`docs/protocol.md` and `docs/supervisor.md` describe behaviour in the present
tense with no "not yet" note. `CONTEXT.md` marks some terms *(designed)* and
not others, so unmarked terms read as built.

This is the failure the TODO's own closing section names: *"nothing catches
'this paragraph describes software that does not exist'"*.

**Fix:** mark every unbuilt behaviour consistently, and add the test that
section asks for.

### 12. ADR-0001 cites `just doctor` as its mitigation; nothing runs it

The ADR's stated mitigation for a wrongly-derived path is that `just doctor`
prints what resolved. No gate runs it and no test covers it.

**Fix:** run it in CI, or drop the claim.

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
and would it do the same today." A durable record of handovers is the raw
material of an audit log.

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
marketing. Everything in the design -- a durable ledger and recorded
handovers -- already points at this. It has simply not been built.

### What would make this fail

- **Building the supervisor before anything real depends on it.** The most
  likely failure is another six months of excellent documents.
- **Letting the docs drift ahead of the code again.** That already happened
  once. The prose gate catches vocabulary; nothing catches "this paragraph
  describes software that does not exist." A test that fails when a doc claims
  a command the CLI does not expose would.

---

## Round 2 -- the gates, injected against

This repository holds the estate's most elaborate gates. Forty-five injections
were run against a scratch copy to find what each one cannot see.

### 13. Six of the eight gates report green when the scanner finds nothing

Every gate asserts that a `found` vector is empty; none has a positive control.
Making the file walk return an empty list, with a real violation present:

```text
M1 sources() empty -- vocabulary/sink        PASS(missed)
M2 sources() empty -- crate existence        PASS(missed)
M3 sources() empty -- english gate           PASS(missed)
M4 sources() empty -- module size            PASS(missed)
M5 scan() returns an empty Vec               PASS(missed)
```

Not hypothetical: three silent-skip paths already exist
(`common/mod.rs:20`, `vocabulary.rs:40`, `standards.rs:65`):

```text
H1 banned word in a chmod-000 .md            PASS(missed)
H2 banned word in an invalid-UTF-8 .md       PASS(missed)
H5 banned word under a chmod-000 directory   PASS(missed)
```

`docs/quality-bar.md:18` -- *"No gate is allowed to be inert. A check that
cannot fail is worse than no check: it reports green while looking at
nothing."* `NORTHSTAR.md:22` -- *"Every check here has been proved by injecting
the fault it exists to catch and watching it fail."* Neither holds.

**Fix:** panic on a directory-read error and on a file that fails to decode;
assert the source list is non-empty against a known floor; add one positive
control per gate that runs the matcher against a fixture and asserts it fires.

### 14. The sink gates miss every form Rust code actually uses

`contains_word` (`vocabulary.rs:23`) treats `_` and alphanumerics as word
characters, so identifiers and plurals are not boundary matches:

```text
A1 the sink name in a .md (control)     FAIL(caught)
A2 the same name pluralised             PASS(missed)
A3 the same name inside an identifier   PASS(missed)
A4 the same name in snake_case          PASS(missed)
```

Retired vocabulary is invisible when wrapped across a line, and one retired
word is already back in the documents.

**Fix:** match on a normalised token stream rather than raw word boundaries, and
join wrapped lines before scanning.

### 15. Three lines of boilerplate disable the module-size gate, in all three repositories

`code_lines` counts lines before the first test-module attribute and never
resumes, so anything below it is uncounted:

```text
honest 401-line module                          FAILED   (control)
same 400 lines, with a test module on lines 2-3     ok    <- evaded
```

This repository's looser matcher also accepts a negated test attribute as the
terminator. `AGENTS.md` points at this gate for "single responsibility".

**Fix:** count all non-test lines, not the prefix before the first test
attribute.

### 16. Gate coverage gaps, verified

- Every gate is blind to YAML, JSON and the justfile.
- The English gate misses combining accents, Latin Extended-A and every
  non-Latin script; the justfile is not read at all.
- A symlink makes the gates read outside the repository.
- The duplication gate only looks inside `crates/`.
- The module-size gate skips `tests/`, where the largest module actually is.
- The brief gate accepts a shebang, a bare `#`, or a `//!` anywhere in the file,
  and reads fewer than half the tracked files.
- The gauntlet-loop gate is case-sensitive under a claim that it is not.
- Doc tests are never executed anywhere in the estate's CI.
- Mutation and coverage jobs measure three lines; the protocol crate
  contributes nothing.

### 17. Documents against code

- `quality-bar.md` contradicts itself and lists a gate with nothing to check.
- ADR-0001's decision text is stricter than the gate now claiming to enforce it.
- `rust-version = "1.85"` is declared and inherited by nothing.
- "English only" is one sentence at one line number in three repositories, with
  two different gates behind it.
