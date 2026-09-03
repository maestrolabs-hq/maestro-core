# Docling

Docling converts documents — PDF, DOCX, PPTX, HTML, images, audio, and more —
into a structured representation and Markdown. It is a document-conversion
library and CLI, not a code-graph provider: it holds no repository index and
no ongoing state, only downloaded model weights.

## What it does and why Maestro uses it

Docling's standard pipeline runs page-layout detection, table-structure
recognition, and figure classification over a document and exports the result
as Markdown, HTML, or a lossless JSON `DoclingDocument`. A second pipeline
mode replaces those specialized models with a single vision-language model
(VLM) that reads page images directly. Maestro uses the CLI to turn arbitrary
source documents — specs, papers, vendor PDFs — into Markdown an agent can
read, without a cloud call: both pipelines run fully offline once the model
artifacts are on disk.

## Vocabulary mapping

| Docling term | What it means | Our term (protocol / estate) |
| --- | --- | --- |
| `DocumentConverter` | the Python entry point that runs a pipeline over a source | conversion pipeline |
| standard pipeline | layout + TableFormer + OCR + classifiers, one model per concern | deterministic conversion pipeline |
| VLM pipeline | a single vision-language model reads page images directly | model-driven conversion pipeline |
| artifacts / artifacts-path | a local directory of pre-downloaded model weights | local model cache |
| `DoclingDocument` | the unified structured representation of a converted document | structured document |
| preset (`--vlm-model`) | a named, pre-configured VLM + prompt + runtime combination | VLM configuration preset |
| `docling-serve` / remote conversion | a separately hosted Docling instance reached over HTTP | remote conversion service — not used |

## Identity

| Field | Value |
| --- | --- |
| Upstream | `docling-project/docling` (MIT) |
| Installed | `docling` 2.124.0 (`docling-core` 2.94.1, `docling-ibm-models` 4.0.2, `docling-parse` 7.17.0) via `uv tool install docling`, Python 3.13.15 |
| PyTorch | default PyPI wheels (CUDA), accepted at install time for a representative VLM test; not the CPU-only build |
| Binaries | `docling`, `docling-tools` at `~/.local/bin` |
| Model artifacts | `~/models/document/parsing/docling` (layout Heron, TableFormer, figure classifier — 702 M), separate from `<workspace>/.maestro/state`; this is the operator's existing model estate, not workspace state |
| Deliberately not downloaded | `code_formula` (640 M, code/formula VLM) and `rapidocr` (its default checkpoint set targets Chinese) — the standard pipeline runs with `--no-ocr` in this deployment instead of pulling either |
| VLM model reuse | `ibm-granite/granite-docling-258M`, already present in the operator's model estate at `~/models/document/parsing/granite-docling-258m`; exposed to Docling via a symlink named `ibm-granite--granite-docling-258M` inside the artifacts directory, so `resolve_model_artifacts_path` finds it locally and no second copy was downloaded |

## CLI surface

| Command | Purpose |
| --- | --- |
| `docling [OPTIONS] SOURCE` | Convert a document (implicit `convert`); writes Markdown to the current directory by default |
| `docling convert-remote` | Convert via a remote `docling-serve` instance — not used in this deployment |
| `docling-tools models download [MODELS]... -o PATH` | Pre-fetch named model artifacts into a local directory for offline use |
| `docling-tools models download-hf-repo REPO` | Pre-fetch an arbitrary Hugging Face repository as artifacts |

Useful flags exercised during provisioning: `--artifacts-path PATH` (use local
artifacts instead of auto-downloading), `--no-ocr` (skip OCR — required here
since RapidOCR was not downloaded), `--pipeline vlm --vlm-model granite_docling`
(switch to the VLM pipeline with the reused local model), `--output DIR`.

## Usage

```shell
docling --artifacts-path ~/models/document/parsing/docling --no-ocr document.pdf
```

```shell
docling --pipeline vlm --vlm-model granite_docling \
  --artifacts-path ~/models/document/parsing/docling document.pdf
```

Both were run against a 9-page arXiv PDF during provisioning: the standard
pipeline converted it in 6.62 s; the VLM pipeline, running the reused
Granite Docling model on the operator's RTX 5090, took 106 s (about 11.8 s per
page). Both produced full Markdown output with no network access and no new
downloads.

## Notes

- Not a Maestro provider: no repository index, no `<workspace>/.maestro/state`
  entry. The only persistent artifact is the model-weight directory above,
  which lives in the operator's model estate.
- Delegating the VLM pipeline to the operator's existing `llama.cpp` server
  (`maestro-llamacpp`) was considered and rejected for this deployment:
  `llama.cpp` requires a GGUF file, and Granite Docling is only present as
  `safetensors`. Converting it would create a second, duplicate copy of the
  same model in a different format rather than reusing the one already on
  disk, which is the opposite of the goal.
- The document conversion output (Markdown/JSON) is never written into this
  repository or into `.maestro/state`; conversions during provisioning ran in
  a temporary directory outside the repository.
- The `docling-mcp` MCP server is documented separately in
  [`docs/providers/docling.md`](../providers/docling.md); it shares this same
  model-artifacts directory.
