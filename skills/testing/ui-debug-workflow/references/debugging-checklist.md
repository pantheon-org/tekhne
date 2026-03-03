# Debugging Checklist

Use this checklist to keep UI debugging runs reproducible.

## Pre-Run

- Confirm baseline branch and changed branch names.
- Confirm URL, viewport, and test account.
- Confirm build/start commands and environment variables.

## Baseline Capture

- Start app from baseline branch.
- Capture screenshots at key states.
- Save DOM snapshots for affected components.
- Save console and server logs.

## Changed Capture

- Switch to changed branch.
- Repeat identical interaction steps.
- Capture the same artifact set.

## Comparison

- Diff screenshots side-by-side.
- Compare DOM and console output.
- Record exact pass/fail criteria.
