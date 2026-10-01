# Scenario 08: DAG Optimization: Sequential Pipeline Bottlenecks and needs-based Parallelism

## User Prompt

A data engineering team's GitLab CI pipeline takes 22 minutes end-to-end. The team lead suspects that independent jobs are running sequentially because of how the stage model was set up, and that switching to DAG-based execution with `needs:` would cut pipeline time significantly.

Analyze the pipeline below for parallelism opportunities. Identify which jobs are genuinely dependent on each other and which can run in parallel. Produce a DAG-optimized version of the pipeline and document the expected improvement.
