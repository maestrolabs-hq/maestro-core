# Optional convenience task runner (https://just.systems).
# Every command below works standalone -- just is never required.

# One-command onboarding: install the hook framework.
setup:
    pre-commit install --install-hooks

# Run the quality gates. CI runs these same commands, not equivalents.
check:
    cargo fmt --all --check
    cargo clippy --all-targets --all-features -- -D warnings
    cargo test --all-targets
    cargo machete
    cargo deny check

# Coverage is reported, not gated, until there is behaviour to cover.
# See docs/quality-bar.md: llvm-cov measures lines, not branches, so a
# floor copied from the Python bar would claim a guarantee it cannot make.
coverage:
    cargo llvm-cov --summary-only

# Format in place. `check` only verifies.
fmt:
    cargo fmt --all
