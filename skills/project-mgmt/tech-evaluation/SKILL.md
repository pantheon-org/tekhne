---
name: tech-evaluation
description: >
  Investigate a candidate library, dependency, or file format against a fixed question set and end with exactly
  one cited recommendation (adopt / keep_current / reject / needs_more_research) -- never "it depends." Validates
  the structured output against a JSON schema before writing a human-readable finding. Use when choosing between
  candidate npm packages, dependencies, or file formats for a specific integration point, or when a plan or ADR
  decision hinges on an unverified factual claim about a library (does it escape by default, does it bundle
  cleanly, is it maintained). Do NOT use for one-line version lookups, purely subjective preference calls with no
  verifiable claim, or a technology choice already settled by an existing ADR -- supersede the ADR instead.
  Triggers: "evaluate this library", "investigate this dependency", "compare X vs Y", "properly evaluate this
  technology", "which format should we use", "is this package maintained".
---

# Tech Evaluation

Investigate a candidate library, dependency, or file format against a fixed question set, and end with exactly one
verifiable recommendation -- never "it depends." A production decision recorded without cited evidence is a guess,
not a finding; this skill exists to close that gap. Produces a schema-validated structured record plus a
human-readable write-up in the project's dated findings directory (see
[Finding Record Layout](references/finding-record-layout.md) for the exact path and shape).

## Prerequisites

- A concrete technology to evaluate (an npm package, a file format, a spec) and, ideally, the alternatives already
  ruled out or being compared against.
- This skill REQUIRES the `create-context-file` skill to persist the human-readable finding, and the `context-index`
  skill to regenerate the index afterwards -- do not run this evaluation if neither is available in the current
  environment.
- A subagent capable of both web research and reading this repository's own source (to check integration/bundling
  fit against real code, not just documentation).

## Mindset

A verdict without a cited source is a guess, not a finding -- the schema enforces at least one piece of evidence
per question for exactly this reason. `needs_more_research` is a legitimate recommendation when the evidence is
genuinely inconclusive, but it MUST NEVER be used to avoid picking a side when the evidence already points one
way. One recommendation, not a comparison table with no conclusion: the point of this skill is to convert
"it depends" into a decision someone can act on in production.

## When to Use

- Choosing between candidate crates/dependencies for a specific integration point (e.g. a CLI-parser
  decision such as `usage`/`usage-rs` vs. `clap`, recorded in a dated plan under `.context/plans/`).
- A plan or ADR decision hinges on a factual claim about a library that keeps getting revisited because nobody
  wrote the answer down with evidence.
- The user explicitly asks to "investigate," "research," or "properly evaluate" a technology option mid-decision.

## When NOT to Use

- Purely subjective preference calls with no verifiable claims to check -- there's nothing for the schema to
  validate.
- A technology choice already settled by an existing ADR -- supersede that ADR instead, don't re-litigate via a
  finding.
- One-line lookups ("what's the latest version of X") -- this is for decisions worth a citation trail, not a
  quick fact check.

## Workflow

1. **Fix the question set.** By default, use five dimensions (`correctness`, `integration_fit`, `footprint`,
   `maturity`, `practical_fit`); PREFER substituting or adding a dimension when the default five don't fit the
   technology under review (e.g. a file-format choice may need a `tooling` dimension instead of `integration_fit`).
   The schema requires at least one question, not exactly five, so adapt this UNLESS the default set already
   covers the decision.

2. **Dispatch a research subagent** (foreground -- the result is needed before continuing). Give it:
   - The concrete technology and its exact identifier (npm package name, repo URL, spec link).
   - The alternatives already considered and why each was ruled out, so it doesn't re-litigate settled ground.
   - The question set from step 1, verbatim.
   - An instruction to cite evidence for every verdict -- a file path with line number, a URL, or a specific doc
     section, never "the docs" or "general knowledge." AVOID trusting a claim about this repository's own
     constraints (this crate's `Cargo.toml` pin, the `mise.toml` toolchain, an existing call site) without reading
     the actual source first -- a generic claim about "how a crate usually behaves" is not evidence about this
     project's actual dependency graph or toolchain.
   - The scaffold at `assets/templates/tech-evaluation-scaffold.yaml`, and an instruction to write its output as
     YAML matching that shape to a path you specify.
   - An explicit instruction that the final recommendation MUST be exactly one of `adopt` / `keep_current` /
     `reject` / `needs_more_research`, with a non-hedging rationale and a single imperative next action.

3. **Run the validator and check its exit code before trusting the YAML:**

   ```bash
   scripts/validate-tech-evaluation.sh <path-to-yaml>
   # exit 0 → schema-valid, safe to render into a finding
   # exit 1 → missing evidence, an out-of-enum verdict, or a too-short rationale
   ```

   If validation fails, send the YAML back to the subagent rather than manually patching it into shape -- a
   pitfall this schema exists specifically to catch.

4. **Render the human-readable finding** with the `context-file` skill (`type: FINDING`, dated and filed under the
   project's findings directory). See [Finding Record Layout](references/finding-record-layout.md) for the exact
   path convention and section layout expected.

5. **Regenerate the context index** (`context-index` skill) so the finding is discoverable, and confirm the new
   file is present in the regenerated index before reporting the evaluation as complete.

6. If the evaluation feeds a plans decision or an ADR, link the finding from there via `related:` -- this skill
   produces evidence, not a decision record in its own right.

## Anti-Patterns

**NEVER** accept a recommendation with an empty or single-word rationale.
**WHY:** The schema's `minLength` on `rationale` exists because "it's better" is not a rationale -- it gives the
next reader nothing to check the reasoning against.

```yaml
# BAD
rationale: "seems fine"

# GOOD
rationale: >
  X does not pull in a runtime dependency on libclang (confirmed from its Cargo.toml), unlike the
  alternative, which would break the project's cross-compilation targets in mise.toml.
```

**NEVER** let a subagent answer from memory when the claim concerns this repository.
**WHY:** A generic claim about "how crate X usually behaves" is not evidence about this project's actual
`Cargo.toml` dependency graph or `mise.toml` toolchain -- verify against the real files, not training data.
**BAD:** Trusting "crate X is pure Rust, no native deps" without checking.
**GOOD:** Instructing the subagent to read the actual `Cargo.toml`/`Cargo.lock` and cite the file/line.

**NEVER** skip the schema validation step because the YAML "looks right."
**WHY:** The validator catches exactly the failure modes a rushed read misses.
**GOOD:** ALWAYS run `validate-tech-evaluation.sh` before rendering the human-readable finding.

## References

| Topic | Reference | When to Use |
| --- | --- | --- |
| Question dimensions in depth, with worked examples per dimension | [Question Dimensions](references/question-dimensions.md) | Choosing or substituting a dimension for an unusual technology (file format, spec, protocol) |
| Finding record layout, section-by-section | [Finding Record Layout](references/finding-record-layout.md) | Rendering the human-readable write-up in step 4 |
| Template | [assets/templates/tech-evaluation-scaffold.yaml](assets/templates/tech-evaluation-scaffold.yaml) | Structuring the subagent's YAML output |
| Schema | [assets/schemas/tech-evaluation.schema.json](assets/schemas/tech-evaluation.schema.json) | Understanding what the validator checks |
| Validation script | [scripts/validate-tech-evaluation.sh](scripts/validate-tech-evaluation.sh) | Running step 3 |

This skill follows the repository convention of YAML template + JSON Schema + validation script for structured
artifacts (see `plan-review`'s `review-report` trio for the precedent this one is modelled on).
