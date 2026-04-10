# Model Tier Guide

The `Model` column in the wave document is the authoritative capability assignment.
Tier names are provider-agnostic — `wave-executor` resolves them to concrete model IDs
via `references/model-map.yaml` at execution time.

This guide explains the reasoning so you can judge edge cases and unlisted tasks.

---

## Tiers

### fast — mechanical operations

Use when the task is fully determined by explicit instructions with no judgment calls.

Characteristics:
- Follows a checklist or script step-by-step
- Output is verifiable against a known structure (rows added, files exist, exit codes)
- No synthesis across multiple documents required
- Failure mode: "step missed", not "wrong interpretation"

Examples:
- Pre-populating index files with a known list of items
- Running sync/build scripts and verifying exit codes
- Flipping status markers in a tracking file
- Lint and consistency audit passes

---

### standard — structured output with bounded judgment

Use when the task drives a skill or fills a template and the output format is pre-defined.
Judgment is required but constrained to a small, known decision space.

Characteristics:
- Uses a tool or skill with a defined output schema
- Must choose between a small set of options (e.g. promote / skip / flag)
- Output quality matters but format is fixed and verifiable
- Failure mode: "wrong choice in a known decision tree"

Examples:
- Triage tasks driven by `triage-tool` or `triage-paper` skills
- Writing documents that follow a fixed template (rubrics, configuration files)
- Binary decisions with a defined rationale format
- Consolidation passes that add rows to existing tables from already-written sources

---

### smart — open-ended synthesis or deep evidence evaluation

Use when the task requires reading multiple documents, reconciling conflicting evidence,
or producing analysis whose quality cannot be checked against a schema.

Characteristics:
- Must draw cross-cutting conclusions from N source files
- Evidence may be absent, weak, or contradictory — requires explicit acknowledgment
- Output quality is judged on depth and accuracy, not format compliance
- Failure mode: "shallow or confident-sounding but unsupported analysis"

Examples:
- Extracting cross-cutting themes from a set of analysis documents
- Per-item deep analysis with evidence evaluation and rubric scoring
- Writing synthesis sections that require comparing multiple sources

---

## Default

Omitting the `Model` column or leaving a cell blank defaults to **`standard`**.

## Decision rule for unlisted tasks

Ask: "Could a `fast` agent complete this correctly by following explicit steps?"

- Yes → `fast`
- No, but the output format is defined and the decision space is small → `standard`
- No, and the output quality depends on synthesis across documents → `smart`

---

## Provider mapping

Tier names are resolved at execution time using `references/model-map.yaml`.
Current defaults (Anthropic):

| Tier | Model ID |
|------|----------|
| `fast` | `haiku` |
| `standard` | `sonnet` |
| `smart` | `opus` |

To use a different provider, update `model-map.yaml` — wave documents do not need to change.
