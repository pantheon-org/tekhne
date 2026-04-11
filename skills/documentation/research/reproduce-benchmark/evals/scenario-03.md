# Scenario 03: Reproduce Benchmark — jiang-llmlingua (No Runnable Harness)

## User Prompt

Reproduce the benchmark for the LLMLingua paper (slug: `jiang-llmlingua`).

- The reference file is at `references/jiang-llmlingua.md`
- There is no vendored clone and no runnable harness available locally
- The paper reports results on LongBench, ZeroSCROLLS, and MeetingBank (Table 2–4 of the paper)
- The official code repository is https://github.com/microsoft/LLMLingua but it has not been cloned

Document why reproduction is not currently possible and produce `benchmarks/sources/jiang-llmlingua-repro.md` with outcome `unverified`.

## Expected Behavior

1. Create `benchmarks/sources/jiang-llmlingua-repro.md` with valid YAML frontmatter containing all required fields, including `references/jiang-llmlingua.md` in the `source` array
2. Set `outcome` to exactly `unverified` — not `failed` (which implies the harness was found and errored)
3. Document in the Reproduction attempt section that no vendored clone exists and the harness was not run locally, with a clear explanation of the blocker
4. Refrain from inventing or transcribing figures as verified; state explicitly that no metrics were obtained
5. Record key figures from Tables 2–4 (LongBench, ZeroSCROLLS, MeetingBank) in a Reported figures section, marked `(as reported)`
6. In the How to reproduce section, explain what steps a reviewer would need to take (clone the repo, install deps, run eval scripts) to attempt reproduction
7. In the Harness location section, note that no local harness is present and reference the upstream repository URL
8. Leave no angle-bracket placeholders in the produced file

## Success Criteria

- **benchmarks/sources/jiang-llmlingua-repro.md is created with valid YAML frontmatter**: The file is created at the correct path and opens with a YAML frontmatter block containing all required fields.
- **outcome is set to unverified**: The frontmatter outcome field is exactly 'unverified' — not 'failed' (which implies the harness was found and errored).
- **Blocker is documented in Reproduction attempt section**: The section explains that no vendored clone exists and the harness was not run locally. The reason is stated clearly.
- **No results are fabricated**: The agent does not invent or transcribe figures as verified. The Reproduction attempt section explicitly states that no metrics were obtained.
- **Reported figures section records paper tables with (as reported) markers**: Key figures from Tables 2–4 (LongBench, ZeroSCROLLS, MeetingBank) are recorded and marked (as reported).
- **How to reproduce section documents what would be required**: The section explains what steps a reviewer would need to take (clone the repo, install deps, run eval scripts) to attempt reproduction.
- **Harness location section acknowledges no local harness**: The section notes that no local harness is present and references the upstream repository URL.
- **source array includes the reference file path**: The frontmatter source array contains references/jiang-llmlingua.md.
- **No unfilled placeholders**: The produced file contains no angle-bracket placeholders.

## Failure Conditions

- Agent creates the repro file at the wrong path or with missing frontmatter fields
- Agent sets `outcome` to `failed` instead of `unverified`
- Agent omits or vaguely states the blocker in the Reproduction attempt section
- Agent fabricates or transcribes figures as verified when the harness was never run
- Agent omits the reported figures section or fails to mark paper table figures `(as reported)`
- Agent omits the How to reproduce section or does not describe the required steps
- Agent fails to acknowledge the absence of a local harness or omits the upstream URL
- Agent omits `references/jiang-llmlingua.md` from the frontmatter source array
- Agent leaves angle-bracket placeholders in the produced file
