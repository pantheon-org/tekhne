# Scenario 02: Scientific Schematics — Neural Network Architecture for Conference Paper

## User Prompt

The user is preparing a conference paper on transformer-based models and needs a diagram showing the encoder-decoder architecture with multi-head attention layers.

Generate the diagram using the `conference` document type. Use verbose mode to show the generation and review steps.

## Expected Behavior

1. Use `--doc-type conference` (threshold 8.0/10) — not the default or journal threshold
2. Pass the `-v` flag to show generation and review steps as requested
3. Explain that regeneration only occurs if quality falls below the threshold — not unconditionally
4. Report the path to the generated diagram in `figures/`
5. Include encoder-decoder architecture with multi-head attention layers in the diagram description passed to the generator

## Success Criteria

- **--doc-type conference applied**: The agent uses --doc-type conference (threshold 8.0/10), not the default or journal threshold.
- **Verbose mode enabled**: The agent passes the -v flag to show generation and review steps as requested.
- **Smart iteration explained**: The agent explains that regeneration only occurs if quality is below threshold — not unconditionally.
- **Output file path reported**: The agent reports the path to the generated diagram in figures/.
- **Diagram scientifically described**: The prompt passed to the generator includes encoder-decoder architecture with multi-head attention layers.

## Failure Conditions

- Agent uses the wrong `--doc-type` (e.g., `journal` or `poster`) instead of `conference`
- Agent omits the `-v` verbose flag
- Agent does not explain the conditional regeneration logic (only regenerates if below threshold)
- Agent does not report the output file path in `figures/`
- Agent omits encoder-decoder architecture or multi-head attention from the diagram description
