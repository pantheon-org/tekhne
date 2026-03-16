# Scenario 3: Identify Anti-Patterns in a Debug Process

## Context

A junior engineer submitted the following pull request description:

> "Fixed the broken header layout. I noticed it looked off on my laptop so I tweaked
> the CSS until it looked right. Tested on my machine — looks good now! Merging."

## Your Task

Review the submitted PR description against the UI Debug Workflow skill's anti-patterns
and produce a written review saved to `pr-review.md`.

## Requirements

Your output must:

1. Identify every anti-pattern from the skill that the process violates (at minimum:
   missing reproduction steps, skipped console/network review, missing evidence artifacts,
   different conditions between runs if applicable).
2. For each anti-pattern identified, quote or paraphrase the specific part of the PR
   description that violates it.
3. For each anti-pattern identified, provide a corrected action the engineer should have
   taken instead.
4. End with a clear APPROVE or REQUEST CHANGES verdict.

## Output Spec

File: `pr-review.md`
