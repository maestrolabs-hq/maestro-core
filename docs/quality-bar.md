# Quality bar

Taken from the `fld-forge/governance` repository, which is the reference
implementation of this standard. This document records the bar and what it costs
to meet, so adopting it is a decision rather than a copy-paste.

## The principle worth stealing

Every claim is enforced by something that goes red. The governance repo does not
document its invariants and hope; it names each one and points at the test that
holds it. Its own words: an unmeasured value "is written as unmeasured, never
invented".

Three practices follow from that, and they are the parts most repos skip.

**One command, same gates, everywhere.** `just check` runs exactly the commands
the CI quality job runs. Not similar ones — the same ones. A gate that behaves
differently locally is a gate nobody trusts.

**Architecture is a test, not a convention.** Import Linter contracts fail the
build when a module imports something it must not. Ruff's banned-api rule
confines `subprocess`, `socket`, `http.client` and `urllib.request` to a single
adapter module. "Only this module does IO" is not prose there; it is red.

**Documentation is gated.** `tests/unit/test_docs.py`, `test_docs_ci.py`,
`test_readme.py` assert that the docs match the config. The doc gate parses
`.pre-commit-config.yaml` rather than matching its text, so — quoting the repo —
"a claim made in a comment cannot satisfy a hook assertion". Docs rot silently
everywhere else; here rot is a failing test.

## Python reference gates

Seven, run by pre-commit and by CI identically:

```text
ruff check .
ruff format --check .
ty check --error-on-warning src scripts tests
mypy                      # strict
deptry src                # stdlib-only invariant
lint-imports              # architecture contracts
pytest -q                 # 90% branch-coverage floor
```

Thresholds: line length 100; mccabe max-complexity 8; max-statements 30;
max-args 5; `--cov-fail-under=90` on **branch** coverage, chosen because "this
tool mutates repository settings, so every untested conditional is a real risk".

Hook types are `[pre-commit, pre-merge-commit]`. Without the second, a merge that
commits on its own brings in changes no gate ever saw.

Gitleaks is pinned to the same version locally and in CI — same engine both
sides, so a commit blocked locally is blocked remotely. The repo is honest that
local hooks are not airtight (`--no-verify`, rebase, fast-forward merges all
bypass them), which is why CI scans full history.

## CI jobs

`quality`, `dependency-review`, `pip-audit`, `secrets-scan`, `semgrep`,
`uv-audit`, `zizmor`, plus CodeQL default setup and a separate Scorecard
workflow.

Every action is pinned by commit SHA. Workflow permissions are `contents: read`.
`persist-credentials: false` on checkout. Concurrency cancels superseded runs. A
weekly cron catches bit-rot without pushes.

## The Rust profile already exists

The same repo specifies a Rust translation. Required on every PR and protected
push:

```text
rust-format   rust-lint   rust-test   rust-audit
dependency-review   secrets-scan   actions-security   CodeQL
```

Weekly or manual, as evidence rather than gates: matrix/all-features, MSRV,
`cargo-audit` as redundant defence, Semgrep Rust, Scorecard, scheduled CodeQL,
docs/coverage. Heavy-matrix freshness must stay under eight days; a missing,
failed or stale heavy run is not reported as healthy.

Its scaffold is fixed: edition 2024, no application dependencies, no Cargo
features, MIT, `README.md`, `SECURITY.md`, `CONTRIBUTING.md`, editor and git
config, hook config, `NORTHSTAR.md`.

`maestro-core` already matches the first four: edition 2024, no dependencies, no
features, MIT.

## What cannot apply yet

Roughly half this bar is GitHub-side: branch rulesets, required status checks,
CodeQL, Scorecard, dependency-review, Dependabot. Our repositories have no
remote, so those are unreachable today.

What is reachable locally, and worth having before any code exists: the gate
command, the hook wiring including `pre-merge-commit`, formatting, linting,
tests with a coverage floor, secret scanning, and architecture contracts.

## What this costs

The governance repo carries 16 ADRs, a NORTHSTAR with measured KPIs, gated docs
and roughly 20 test modules for a stdlib-only tool. That is proportionate to
something holding credentials that mutate a fleet.

Adopting the whole bar on an empty repository is how it becomes ceremony nobody
maintains. The gates that pay for themselves from the first commit are
formatting, linting, tests and the architecture contracts — the last because
`maestro-core`'s crate boundaries are its main design claim, and an unenforced
boundary is a comment.
