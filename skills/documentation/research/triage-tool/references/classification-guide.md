# Tool Classification Guide

Use this guide when classifying a tool during the **Classify** step of the triage workflow.
Customise the tag taxonomy and scope criteria to match your research domain.

## Tag Taxonomy

| Tag | Meaning |
|---|---|
| `library` | Reusable code library or package |
| `framework` | Opinionated structure for building something |
| `cli` | Operated via command-line interface |
| `service` | Runs as a standalone service or API |
| `daemon` | Runs as a background process |
| `mcp-server` | Exposes capabilities via MCP protocol |
| `sdk` | Language-specific developer SDK |
| `benchmark` | Evaluation or benchmark tool |
| `dataset` | Training or evaluation dataset |

A tool may carry multiple tags. Add domain-specific tags as needed.

## Scope Assessment

| Verdict | Criteria |
|---------|----------|
| **In scope** | Directly relevant to the research domain tracked in the repo |
| **Borderline** | Adjacent to the domain but with partial relevance; note the overlap |
| **Out of scope** | No meaningful connection to the research domain |

For borderline tools: triage and flag the overlap in REVIEWED.md; do not silently exclude or include.

## Examples

### Clearly in scope
- Replace with a concrete example from your research domain.

### Borderline
- Replace with a borderline example — a tool that is adjacent but has partial relevance.

### Out of scope
- Replace with an out-of-scope example — log it as out-of-scope in REVIEWED.md.
