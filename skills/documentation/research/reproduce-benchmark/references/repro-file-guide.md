# Repro File Guide

Detail for writing `benchmarks/sources/<slug>-repro.md`. The template in `assets/templates/REPRO-benchmark.yaml` is the source of truth for field and section names.

## Metric categories

Compare actual output against reported figures and categorise each metric:

| Category | Rule |
| --- | --- |
| Verified | Reproduced within plus or minus 10 per cent of the reported value |
| Not reproduced | Result differs materially from the reported value; quote both |
| Could not test | The scenario requires unavailable infrastructure; note why |

## Outcome values

| Value | Meaning | Typical situation |
| --- | --- | --- |
| `verified` | All reported figures reproduced within tolerance | Harness ran and every metric held |
| `partially-verified` | Some figures reproduced; others could not be confirmed | Some scenarios ran, others failed tolerance or could not run |
| `unverified` | Harness exists but was not run | Missing infrastructure, paid API, or no local clone |
| `failed` | Harness exists but errors prevented completion | Missing module, build error, crash |

## Required frontmatter and sections

Frontmatter fields: `title`, `date`, `type` (always `repro`), `slug`, `source` (array of reference and analysis paths), `environment` (with `os` and `runtime`) and `outcome`. Optional: `pinned_commit`, `links`.

Sections, in this order: `## Harness location`, `## Reproduction attempt`, `## How to reproduce`, `## Reported figures (as reported)`, and optionally `## Notes`.

## Reported against verified

```text
BAD:  "Achieves 96% savings."
GOOD: "Reports 96% savings (as reported, BENCHMARK.md). Verified: 100% on 4 of 21 scenarios; remainder not run."
```

## Back-filling the analysis file

If the analysis file's Stage 2.2 ("Independent verification") does not reference the repro file, add:

```text
See [`benchmarks/sources/<slug>-repro.md`](../benchmarks/sources/<slug>-repro.md) - outcome: <outcome>.
```

## Command cheat sheet

```bash
# Check for an existing repro
ls benchmarks/sources/<slug>-repro.md 2>/dev/null || echo "not found"

# Install harness dependencies (vendored submodule)
cd tools/<slug> && npm install

# Run the benchmark and keep the output, including errors
npm run benchmark 2>&1 | tee /tmp/<slug>-benchmark-out.txt

# Validate the finished repro file
./scripts/validate-repro-benchmark.sh benchmarks/sources/<slug>-repro.md
```
