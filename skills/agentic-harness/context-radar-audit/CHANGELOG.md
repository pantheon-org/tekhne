# Changelog

All notable changes to the context-radar-audit skill.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this skill adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-09-28

### Added

- Audit a project's Claude Code tool stack (MCP servers, plugins, hooks) against Context Radar's public comparison dataset, with scripts to collect active tools and fetch the dataset, and a trigger eval set.
- A full instruction-coverage eval set (`evals/instructions.json`, four scenarios, `evals/summary.json`) covering pick-one redundancy citation, hook-invoked tools not being mistaken for inactive ones, phantom-tool findings, and refusing to edit global config on a vague go-ahead. A `references/data-shape.md` reference for the full dataset field list, linked from the References table.
