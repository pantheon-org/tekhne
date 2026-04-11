# Scenario 03: Sci-Data-Extractor — Batch Processing with Failure Review

## User Prompt

A researcher has a folder `./literature/` containing 5 PDF files and wants all of them extracted using the `review` template into CSV format, saved to `./output/`.

Run `batch_extract.py` on the folder. After the batch completes, the summary shows that 2 files failed (e.g., `paper3.pdf` and `paper5.pdf`).

Report the results to the user, explain what steps to take to diagnose and re-run the failed files individually, and show the exact command for re-running one of the failed files with `--print` to inspect output.

## Expected Behavior

1. Run `./scripts/batch_extract.py ./literature/ ./output/ --template review --format csv`
2. Read the batch summary output and correctly identify `paper3.pdf` and `paper5.pdf` as failed files
3. Provide the exact command to re-run one failed file individually with `--print` for inspection, e.g. `./scripts/extractor.py ./literature/paper3.pdf --template review -o /tmp/paper3_debug.md --print`
4. Explicitly warn the user that the output folder contains only 3 successful extractions and the dataset is incomplete until failed files are resolved

## Success Criteria

- **batch_extract.py invoked with correct arguments**: The agent runs ./scripts/batch_extract.py ./literature/ ./output/ --template review --format csv.
- **failed files identified from batch summary**: The agent reads the batch summary output and correctly identifies paper3.pdf and paper5.pdf as failed files.
- **individual re-run command provided with --print flag**: The agent provides the exact command to re-run one failed file individually, including --print for inspection, e.g. ./scripts/extractor.py ./literature/paper3.pdf --template review -o /tmp/paper3_debug.md --print.
- **user advised not to treat partial batch as complete**: The agent explicitly warns that the output folder contains only 3 successful extractions and that the dataset is incomplete until failed files are resolved.

## Failure Conditions

- Agent omits `--template review` or `--format csv` from the batch command, or uses the wrong paths
- Agent fails to read the batch summary or does not identify the correct failed files
- Agent provides a re-run command without the `--print` flag, or omits the individual re-run step entirely
- Agent does not warn the user that the dataset is incomplete due to the 2 failed extractions
