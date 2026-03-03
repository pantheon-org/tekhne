# Archived Migration Scripts

This directory contains one-time migration scripts that were used during repository restructuring. They are preserved for historical reference but are no longer needed for day-to-day operations.

## Scripts

### convert-audits.sh
Converted old audit format to new structure. Used during initial audit system setup (2026-02).

### migrate-audit-paths.sh
Restructured audit directory paths from flat structure to hierarchical organization by domain/category.

### migrate-skills-from-agents.sh
Migrated skills from old `.agents/` structure to new `skills/` organization.

### restructure-audits.sh
Reorganized audit directories to align with new skill taxonomy (12 domains).

## Status

These scripts are **deprecated** and should not be run. They are kept for historical context only.

All functionality has been replaced by the TypeScript CLI tool at `cli/`.
