# Archify

Archify is the architecture renderer behind the planned `maestro architecture`
facade. It turns plain-language descriptions, repository evidence, or pasted
Mermaid into polished, validated diagrams delivered as self-contained HTML.

## What it does and why Maestro uses it

Archify produces architecture, workflow, sequence, data-flow, and
lifecycle/state diagrams as standalone HTML with inline SVG, dark/light
themes, optional trace motion, and PNG/JPEG/WebP/SVG/WebM export. Maestro uses
it because reviewed architecture must be a portable artifact: one HTML file
that renders anywhere, can be attached to evidence, and can be regenerated
from the same inputs. It accepts source evidence from an open repository, so
diagrams can be grounded in code rather than drawn from memory.

## Vocabulary mapping

| Archify term | What it means | Our term (protocol / estate) |
| --- | --- | --- |
| skill | SKILL.md contract plus a Node renderer, invoked by the agent | provider surface (not a PATH executable) |
| type router | picks the diagram type from the request | render request routing |
| authoring invariants | validation rules every diagram must pass | reviewed rendering |
| trace motion | animated flow along edges in the viewer | optional viewer capability |
| delivery | the standalone HTML artifact | rendered architecture artifact |
| source evidence | repository inspection feeding the diagram | generation source snapshot |
| update awareness | optional upstream version reminder (HTTP GET) | disable with `ARCHIFY_UPDATE_CHECK_DISABLED=1` |

## Identity

| Field | Value |
| --- | --- |
| Upstream | `tt-a1i/archify` (JavaScript, MIT, based on Cocoon-AI/architecture-diagram-generator) |
| Distribution | agent skill — **not** a standalone executable |
| Installed | `~/.pi/agent/skills/archify` v2.17.0-dev.1 via `npx skills add tt-a1i/archify -g --agent pi --copy` |
| Reviewed pin | `binary` line in `maestro-pi-config` `config/provision.txt`: release v2.16.0 archive + SHA-256 (manual, never auto-installed) |
| Known drift | installed skill (2.17.0-dev.1) is ahead of the reviewed pin (v2.16.0); realign when 2.17 ships as a reviewed release |

## Skill surface

| Component | Purpose |
| --- | --- |
| `SKILL.md` | Full generation and viewer contract (fast authoring path, type router, Mermaid input, authoring invariants, delivery, setup and fallback) |
| `bin/archify.mjs` | Renderer entry point |
| `bin/preview.mjs`, `bin/open-artifact.mjs` | Local preview and artifact opening |
| `bin/visual-check.mjs` | Validation of the rendered output |
| `delta/architecture-delta.mjs` | Architecture change comparison |

## Usage

In any agent that loads the skill (Pi after a reload):

```text
Use Archify to draw: Pi -> Maestro CLI -> Supervisor -> SQLite ledger -> worker -> a memory sink
```

With a repository open, ask for a diagram grounded in the sources; the skill
inspects code as evidence. Mermaid `flowchart`, `sequenceDiagram`, and
`stateDiagram` input is accepted and beautified.

## Notes

- Archify is never registered with Pi's MCP registry; it is a skill, not a server.
- The workspace-intelligence spec assumed an `archify` executable discovered on
  `PATH`. The official distribution is a skill; the `maestro architecture`
  adapter design must be revised to invoke the skill's Node renderer (or
  delegate to the agent) before implementation.
- `maestro-pi-config` records the reviewed release but never downloads or
  installs it; the operator supplies the skill through the official installer.
