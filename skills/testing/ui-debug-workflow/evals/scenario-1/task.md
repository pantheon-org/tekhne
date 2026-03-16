# Scenario 1: Produce Deterministic Reproduction Steps

## Context

A developer reports: "The user table on /admin/users sometimes shows wrong row counts
after sorting. I saw it happen once this afternoon."

## Your Task

Using the UI Debug Workflow skill, produce a written reproduction recipe that a second
developer or automated test can follow to reliably trigger the reported behavior.

## Requirements

Your output must be a Markdown file saved to `debug-reproduction.md` containing:

1. A numbered list of precise reproduction steps (URL to open, interactions to perform,
   observable outcome to watch for).
2. The environment context required: browser, viewport size, seed data state, build mode.
3. An explicit statement of what "reproduced" means (the specific observable symptom).
4. At least one console/network check step that must be performed before any code change.

## Output Spec

File: `debug-reproduction.md`

The file must be self-contained so a reader with no prior context can follow the steps
and reach the same outcome.
