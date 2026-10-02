# Scenario 6: Adopt Nx boundaries in a legacy repository with no tags and no CI base

## User Prompt

A legacy Nx workspace has no project tags, no `targetDefaults` and CI jobs that check out a shallow clone with no known base branch. You cannot fix everything in one release. Describe what behaviour to expect and what to configure right now, how to relax things temporarily without losing control, and what to document.

## Repo state

- Twenty projects, none tagged.
- `nx.json` has no `targetDefaults` entry.
- CI uses a shallow checkout and runs `nx affected` with no `--base`.
