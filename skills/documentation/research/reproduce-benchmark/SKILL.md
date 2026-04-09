---
name: reproduce-benchmark
description: "Reproduce and verify the benchmark claims of a tool or paper already triaged in the research repo. Locates the benchmark harness, runs it, and records verified vs. reported figures in benchmarks/sources/{slug}-repro.md. Triggers: reproduce benchmark, verify claims, run benchmark, check benchmark, benchmark reproduction, verify numbers."
allowed-tools: Read, Write, Edit, Bash
---

# Reproduce Benchmark

Run a tool's or paper's benchmark harness and record what could and could not be verified.

## When to Use

- User asks to reproduce or verify benchmark claims for a triaged tool or paper
- Following up on a triage or analysis where self-reported metrics need independent verification
- User provides a slug, reference path, or analysis path and says "run the benchmark"

## When Not to Use

- The subject has not yet been triaged — run `triage-tool` or `triage-paper` first
- A repro file already exists in `benchmarks/sources/` — report it and offer to re-run instead
- The benchmark requires live external services or paid APIs that are unavailable — set outcome to `unverified` and document the blocker

## Mindset

Reproduction is a verification exercise, not a sales pitch.

1. **Record what you observe**: quote actual output, not what the README says the output should be.
2. **Mark everything reported**: any figure from the README or paper must carry `(as reported)`. Never blend reported and verified numbers without distinguishing them.
3. **Partial is better than fabricated**: if only some scenarios run, record those honestly. Set outcome to `partially-verified`.
4. **Environment matters**: cold-start overhead, fixture quality, and dependency versions all affect results. Document them.

## Workflow

### 1. Identify the subject

- Confirm the slug from the user (e.g. `context-mode`, `jiang-llmlingua`).
- Locate the reference file (`references/<slug>.md`) and analysis file (`analysis/ANALYSIS-<slug>.md`) if they exist.

### 2. Check for existing repro

- Search `benchmarks/sources/` for `<slug>-repro.md`.
- If found, report it and ask whether to re-run or update in place.

### 3. Locate the benchmark harness

- Check the analysis file's Stage 2 ("Evaluative") section first.
- Otherwise check the reference file's "Benchmarks / self-reported metrics" section and the vendored source at `tools/<slug>/`.
- Record the harness file paths and any npm/make/cargo/shell invocation scripts.

### 4. Set up the environment

- Install dependencies required by the harness (e.g. `npm install` in the submodule).
- Note the OS, primary runtime version, and any other relevant environment details.
- If the tool is vendored as a submodule, record the pinned commit SHA.

### 5. Run the harness

- Execute the benchmark using the commands identified in step 3.
- Capture stdout/stderr. Do not suppress errors.
- If the harness fails, document the error and set outcome to `failed`.

### 6. Record results

Compare actual output against reported figures. Categorise each metric as:

- **Verified** — reproduced within ±10% of reported value
- **Not reproduced** — result differs materially from reported value; quote both
- **Could not test** — scenario requires unavailable infra (note why)

### 7. Write the repro file

Read `assets/templates/REPRO-benchmark.yaml` to get the required frontmatter fields and section structure. Create `benchmarks/sources/<slug>-repro.md` with a YAML frontmatter block (all `required_fields` from the template) followed by the required sections.

Set `outcome` to one of:

| Value | Meaning |
|---|---|
| `verified` | All reported figures reproduced within tolerance |
| `partially-verified` | Some figures reproduced; others could not be confirmed |
| `unverified` | Harness exists but was not run (missing infra, etc.) |
| `failed` | Harness exists but errors prevented completion |

### 8. Back-fill the analysis file (if applicable)

If an ANALYSIS file exists and Stage 2.2 ("Independent verification") does not yet reference the repro file, add a pointer:

```
See [`benchmarks/sources/<slug>-repro.md`](../benchmarks/sources/<slug>-repro.md) — outcome: <outcome>.
```

### 9. Validate and report

Run the validator and summarise findings:

```bash
./scripts/validate-repro-benchmark.sh benchmarks/sources/<slug>-repro.md
```

Report: what was verified, what was not, and any caveats about fixture quality or environment.

## Quick Commands

```bash
# Check for existing repro
ls benchmarks/sources/<slug>-repro.md 2>/dev/null || echo "not found"

# Install harness deps (vendored submodule)
cd tools/<slug> && npm install

# Run benchmark and capture output
npm run benchmark 2>&1 | tee /tmp/<slug>-benchmark-out.txt

# Validate the completed repro file
./scripts/validate-repro-benchmark.sh benchmarks/sources/<slug>-repro.md
```

## Anti-Patterns

### NEVER omit YAML frontmatter

**WHY:** Files without frontmatter fail schema validation and break indexing tools that rely on structured metadata.

**BAD** Start the file with `# <slug> — Benchmark Reproduction` followed by bold-text fields. → **GOOD** Open with `---` YAML frontmatter block containing all required fields before any prose.

### NEVER blend reported and verified numbers

**WHY:** Mixing unverified claims with observed results corrupts the research record.

**BAD** `"Achieves 96% savings."` when the harness was not run. → **GOOD** `"Reports 96% savings (as reported, BENCHMARK.md). Verified: 100% on 4 of 21 scenarios; remainder not run."

### NEVER suppress harness errors

**WHY:** A silent failure looks like a clean run. Errors are data.

**BAD** Redirect stderr to `/dev/null` or omit error output from the repro file. → **GOOD** Quote the error verbatim in the Reproduction attempt section and set outcome to `failed` or `partially-verified`.

### NEVER skip the existing repro check

**WHY:** Re-running an expensive benchmark without checking for a prior run wastes time and creates conflicting records.

**BAD** Write a new repro file without checking `benchmarks/sources/`. → **GOOD** Check first; if a file exists, report it and ask whether to re-run.

## References

- **Repro artifacts**: [YAML template](assets/templates/REPRO-benchmark.yaml) · [schema](assets/schemas/repro-benchmark.schema.json) · [validator](scripts/validate-repro-benchmark.sh)
