---
name: implementation-plan-splitter
description: >
  Merged into implementation-planner. Use that skill for both creating new plans
  and restructuring existing monolithic planning documents into hierarchical
  directory structures.
license: MIT
compatibility: opencode
metadata:
  version: 2.0.0
  status: merged
---

# Implementation Plan Splitter

This skill has been merged into
[implementation-planner](../implementation-planner/SKILL.md).

All functionality is now part of the unified skill, including:

- Splitting monolithic planning docs into hierarchical structures
- `generate-structure.sh` automation from JSON plans
- `validate-structure.sh` structure validation
- Expert heuristics for flatten vs subdivide decisions
- Error recovery workflows

Use [implementation-planner](../implementation-planner/SKILL.md) — Mode 2.
