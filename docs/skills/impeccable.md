# Impeccable

Impeccable is the design-guidance skill for agent-built frontends: one skill,
23 commands, and 61 deterministic detector rules that steer AI-generated
interfaces away from the default template look.

## What it does and why Maestro uses it

Every model trained on the same SaaS templates produces the same tells —
one typeface for everything, the same gradients, cards nested in cards.
Impeccable counters that with durable product truth (`PRODUCT.md`), a recorded
visual system (`DESIGN.md`), a shared design vocabulary (`polish`, `audit`,
`critique`, `distill`, `animate`, `bolder`, `quieter`, …), and deterministic
detector rules that run with no LLM and no API key. The estate uses it as an
operator-side quality bar for any frontend surface the agents produce — like
Plannotator, it is a review-and-guidance tool at the human boundary, not a
Maestro provider: no facade wraps it and none is planned.

## Vocabulary mapping

| Impeccable term | What it means | Our term (protocol / estate) |
| --- | --- | --- |
| skill | SKILL.md contract plus reference guides and scripts | provider surface (agent skill) |
| `PRODUCT.md` | durable product truth: audience, purpose, constraints, voice | product context record |
| `DESIGN.md` | the incumbent or newly built visual system | design decision record |
| detector rules | 61 deterministic checks, no LLM, no API key | deterministic design gate |
| anti-patterns | the known template tells the rules catch | design drift |
| visitor mode | per-surface audience framing chosen at design time | surface context |
| pin | promote one command to a standalone shortcut | command alias |

## Identity

| Field | Value |
| --- | --- |
| Upstream | `pbakaus/impeccable` (JavaScript, impeccable.style; started from Anthropic's frontend-design skill) |
| Distribution | agent skill + `npx impeccable` CLI + optional browser extension |
| Installed | `~/.pi/agent/skills/impeccable` v4.1.3 (Apache 2.0), Pi harness only |
| Install command | `npx impeccable install` → customize → `pi` → global |
| Allowed tools | the skill restricts itself to `npx impeccable *` and its own scripts |
| Governance | not yet recorded in `maestro-pi-config` (pending decision, same as Plannotator) |

## Command surface

| Command | Purpose |
| --- | --- |
| `/impeccable init` | Inspect the project, ask only for material gaps, write `PRODUCT.md` |
| `/impeccable audit <target>` | Detector-rule and critique pass over a surface |
| `/impeccable critique <target>` | UX design review |
| `/impeccable polish <target>` | Final pass before shipping |
| `/impeccable harden <target>` | Error handling and edge cases |
| `/impeccable bolder` / `quieter` / `animate` / `colorize` / `distill` … | Directional adjustments from the 23-command vocabulary |
| `/impeccable pin <command>` | Create a standalone shortcut (e.g. `/audit`) |
| `/impeccable <description>` | Free-form design request |
| `npx impeccable install` | Installer; detects harnesses (knows Pi), interactive — run it outside a repository you care about |

## Usage

After a Pi reload, start any frontend project with `/impeccable init`, then
drive surfaces with the command vocabulary:

```text
/impeccable audit blog
/impeccable critique landing
/impeccable polish settings
```

## Notes

- The deterministic detectors run locally without any model; only critique
  commands involve the LLM.
- Caution learned the hard way: `npx impeccable install` does not understand
  `--help` and runs immediately with defaults, writing into the detected
  project harness (`.github/…`) — invoke it from a neutral directory.
- Skill updates ship as GitHub releases (`skill-vX.Y.Z`, `universal.zip`);
  rerun the installer to update.
