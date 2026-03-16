# Cross-Platform Skill Compliance Review

## Problem Description

A team has written a SQL query optimization skill and wants to publish it to the Tessl public registry so teams using different AI assistants can benefit from it. A colleague flagged potential compatibility issues — the skill was originally written for their internal setup and may contain references that won't work outside their environment.

Review the skill for cross-agent compatibility issues, produce a compliance report as `compliance-report.md`, and write a corrected version of the skill as `SKILL-fixed.md`.

## Input Files

The following file is provided. Extract it before beginning.

=============== FILE: inputs/SKILL.md ===============
---
name: sql-query-optimizer
description: Optimize SQL queries for performance with index analysis and query plan inspection.
---

# SQL Query Optimizer

## Quick Start

Open your SQL file in Cursor and use @codebase to index all your schema files, then
ask for optimization suggestions.

Alternatively, in Claude Code, run /compact before starting to free up context for large schemas.

## Workflow

1. Use the Read tool to load the SQL file
2. Use the opencode `model switch claude-opus-4-6` command to ensure maximum context
3. Analyze the query execution plan
4. Use the Edit tool to apply optimizations
5. Use the Bash tool to run EXPLAIN ANALYZE and capture results

## Anti-Patterns

### Never optimize without reading the schema

Use Cursor's @workspace to load all relevant tables before suggesting index changes.
