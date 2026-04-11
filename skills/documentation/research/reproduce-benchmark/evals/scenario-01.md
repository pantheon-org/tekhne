# Scenario 01: Reproduce Benchmark — context-mode

## User Prompt

Reproduce the benchmark for `context-mode` (slug: `context-mode`).

- The tool is vendored at `tools/context-mode/` (pinned commit `601aaf1`)
- The reference file is at `references/web/context-mode.md`
- The analysis file is at `analysis/ANALYSIS-context-mode.md`
- The benchmark harness is at `tools/context-mode/tests/benchmark.ts` (run via `npm run benchmark`)

Run the harness, record what could be verified against the reported figures in `BENCHMARK.md`, and create `benchmarks/sources/context-mode-repro.md`.

## Expected Behavior

1. Check `benchmarks/sources/context-mode-repro.md` for an existing repro before running anything
2. Locate the harness at `tools/context-mode/tests/benchmark.ts` and run it via `npm run benchmark` (or `npx tsx`) after installing dev dependencies
3. Document the environment in the repro file (OS, Node.js version, and other relevant runtime details)
4. Create `benchmarks/sources/context-mode-repro.md` with valid YAML frontmatter containing all required fields (title, date, type, slug, source, environment, outcome)
5. Set `outcome` to `partially-verified` (or `verified`) based on actual harness results — not copied from the README
6. Clearly distinguish harness-produced figures from reported figures, marking reported values with `(as reported)`
7. Include a `## Reported figures (as reported)` section with every quoted number carrying the marker
8. Provide a `How to reproduce` section with copy-pasteable shell commands
9. Back-fill `analysis/ANALYSIS-context-mode.md` Stage 2.2 with a reference to the repro file
10. Ensure no angle-bracket placeholders remain in the produced file

## Success Criteria

- **Existing repro check performed first**: The agent checks benchmarks/sources/context-mode-repro.md before running anything.
- **Harness is located and invoked correctly**: The agent identifies tools/context-mode/tests/benchmark.ts and runs it via npm run benchmark (or npx tsx) after installing dev dependencies.
- **Environment is documented**: The repro file records OS, Node.js version, and any other relevant runtime details used during the run.
- **benchmarks/sources/context-mode-repro.md is created with valid YAML frontmatter**: The file is created at the correct path and opens with a YAML frontmatter block containing all required fields (title, date, type, slug, source, environment, outcome).
- **outcome field reflects actual results**: outcome is set to 'partially-verified' (or 'verified') based on what the harness actually produced — not copied blindly from the README.
- **Verified and reported figures are clearly distinguished**: Figures from the harness output are presented separately from reported figures. Reported values are marked '(as reported)'.
- **Reported figures section present with (as reported) markers**: The ## Reported figures (as reported) section exists and every quoted number carries the (as reported) marker.
- **How to reproduce section contains runnable shell commands**: The section provides copy-pasteable commands that would allow a third party to replicate the attempt.
- **ANALYSIS Stage 2.2 back-filled with repro pointer**: analysis/ANALYSIS-context-mode.md Stage 2.2 is updated to reference benchmarks/sources/context-mode-repro.md.
- **No unfilled placeholders**: The produced file contains no angle-bracket placeholders such as <slug> or <outcome>.

## Failure Conditions

- Agent skips the existing repro check and runs the harness unconditionally
- Agent fails to locate or invoke the correct harness entry point
- Agent omits environment details (OS, Node.js version) from the repro file
- Agent creates the repro file at the wrong path or with missing frontmatter fields
- Agent copies the outcome value from the README without running the harness
- Agent mixes harness results and reported figures without distinguishing them
- Agent omits the `## Reported figures (as reported)` section or fails to mark numbers
- Agent omits runnable shell commands from the How to reproduce section
- Agent fails to update Stage 2.2 in the ANALYSIS file
- Agent leaves angle-bracket placeholders in the produced file
