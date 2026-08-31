<div align="center">

# maestro-core

**The orchestrator agents work under**

Delegates all the work and does none of it, while staying accountable for all of it.

  <a href="https://github.com/maestrolabs-hq/maestro-core/actions/workflows/ci.yml"><img alt="CI" src="https://img.shields.io/github/actions/workflow/status/maestrolabs-hq/maestro-core/ci.yml?branch=main&style=for-the-badge&label=CI&labelColor=1c1c1c&color=2ea043"></a>
  <a href="https://github.com/maestrolabs-hq/maestro-core/actions/workflows/heavy.yml"><img alt="Heavy" src="https://img.shields.io/github/actions/workflow/status/maestrolabs-hq/maestro-core/heavy.yml?branch=main&style=for-the-badge&label=Heavy&labelColor=1c1c1c&color=8957e5"></a>
  <a href="https://scorecard.dev/viewer/?uri=github.com/maestrolabs-hq/maestro-core"><img alt="OpenSSF Scorecard" src="https://img.shields.io/ossf-scorecard/github.com/maestrolabs-hq/maestro-core?style=for-the-badge&label=Scorecard&labelColor=1c1c1c"></a>
  <a href="https://github.com/maestrolabs-hq/maestro-core/blob/main/LICENSE"><img alt="License" src="https://img.shields.io/badge/License-MIT-1c1c1c?style=for-the-badge&labelColor=1c1c1c&color=0969da"></a>

  <img alt="Rust" src="https://img.shields.io/badge/Rust-1.98-CE422B?style=flat-square&logo=rust&logoColor=white">

</div>

---

It knows nothing about any particular agent. Clients ask; Maestro answers.

## Layout

```text
crates/protocol   what a client may ask, and what Maestro answers
crates/cli        the `maestro` binary
```

Two crates. There were six: `policy`, `sink`, `supervisor` and `ledger` were
each created before anything needed them, and a seam is only real when
something varies across it. Nothing varied, and every one of them held zero
lines.

The designs are still here, in `docs/`. They are the part that was worth
keeping. Crates come back when code asks for them, which is also when their
boundaries and their names will be known rather than guessed.

## Documents

- [CONTEXT.md](./CONTEXT.md) — glossary
- [docs/supervisor.md](./docs/supervisor.md) — residency, delegation, accountability (design; no crate yet)
- [docs/protocol.md](./docs/protocol.md) — request and answer
- [docs/ledger.md](./docs/ledger.md) — durability, states, retry (design; no crate)

## CLI shape

Noun-verb, after herdr. The command is a thin client over the supervisor.

```text
maestro memory   capture | recall | status | drain
maestro status | completion <shell>
```

## Status

Workspace builds. No commands implemented.
