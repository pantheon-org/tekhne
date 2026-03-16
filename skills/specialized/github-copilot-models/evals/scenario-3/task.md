# Screenshot Analysis Pipeline: Find Vision Models

## Problem Description

A QA team runs automated screenshot analysis to catch visual regressions. Their pipeline needs to dynamically discover which AI models on their GitHub Copilot account actually support image/vision input and are available to use. The list changes as models are added or retired, so it needs to query live data rather than hardcode names.

Write a shell script `list-vision-models.sh` that outputs the IDs of all currently usable vision-capable models, one per line. Also write a brief `NOTES.md` explaining one common pitfall when selecting vision models.
