---
title: "<Tool name>"
author: "<Author / org>"
date: YYYY-MM-DD
type: reference
tags:
  - tool
  - context-management
  - <compression|tiered-loading|token-budgeting|injection|cli|daemon>
source: "<primary landing page (GitHub / docs / npm / PyPI)>"
source_repo: "<GitHub URL>"
local_clone: "../tools/<repo-name>"
version: "<vX.Y.Z / commit SHA>"
context: "<why we care; what synthesis bucket this supports>"
related:
  - "../ANALYSIS-<tool-analysis-file>.md"
---

## TL;DR (3–8 bullets)

-
-
-

## What's novel / different

What does this do that adjacent tools do not?

## Architecture overview

### Context representation

How does the tool model the context window? (flat token stream, markdown vault, structured slots, etc.)

### Injection mechanism

How does content get into the context? (system prompt prepend, tool result injection, retrieval, etc.)

### Compression / summarization

Any lossy or lossless reduction applied before injection?

### Eviction / overflow handling

What happens when context budget is exceeded?

### Session lifecycle

How is context state managed across turns / sessions?

## Deployment model

- Runtime: (CLI / daemon / library / MCP server / etc.)
- Language:
- Dependencies:
- Storage:

## Benchmarks / self-reported metrics

Quote numbers with source; mark "as reported" for unverified claims.

## Open questions / risks / missing details

-

## Notes

Corrections, updates, follow-up pointers.
