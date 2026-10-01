# Scenario 4: Mixed Results Against Reported Figures

## User Prompt

The user says: "Reproduce the benchmark for `fastgrep` (slug: `fastgrep`) and tell me if the numbers hold up. The README says it is 10x faster on all six benchmarks."

Repo state: the tool is vendored at `tools/fastgrep/` (pinned commit `a1b2c3d`), `references/web/fastgrep.md` and `analysis/ANALYSIS-fastgrep.md` exist, and no repro file exists yet. The harness runs. Observed results: three benchmarks land within 5 per cent of the reported speed-up, two come in at roughly 3x instead of the reported 10x, and one benchmark needs a GPU that this machine does not have. The first run shows a large cold-start penalty.
