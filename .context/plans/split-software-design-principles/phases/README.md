# Implementation Phases

This directory contains the detailed breakdown of all 9 phases for splitting the software-design-principles hub skill.

## Phase Overview

1. **[Phase 1: Create Directory Structure](phase-1-create-directory-structure/)** (~15 min)
   - Create tile root and 4 skill subdirectories
   - Set up references/ and evals/ folders
   - Copy .vscode settings

2. **[Phase 2: Redistribute Reference Documents](phase-2-redistribute-reference-documents/)** (~30 min)
   - Move 44 reference docs using git mv
   - Preserve file history
   - Verify distribution counts

3. **[Phase 3: Author New SKILL.md Files](phase-3-author-new-skill-files/)** (~4-5 hours)
   - Write 4 focused SKILL.md files
   - Extract content from original hub
   - Create pattern-selection-workflow.md

4. **[Phase 4: Create Unified tile.json](phase-4-create-unified-tile/)** (~15 min)
   - Configure tile with 4 skill references
   - Set version and publication settings
   - Validate JSON structure

5. **[Phase 5: Create Eval Scenarios](phase-5-create-eval-scenarios/)** (~1 hour)
   - Author 13 evaluation scenarios (3+4+3+3)
   - Test trigger conditions
   - Validate expected outputs

6. **[Phase 6: Delete Original Hub](phase-6-delete-original-hub/)** (~10 min)
   - Remove software-design-principles directory
   - Commit with breaking change message
   - Document migration path

7. **[Phase 7: Update Repository Documentation](phase-7-update-repository-docs/)** (~20 min)
   - Update AGENTS.md references
   - Modify skill-taxonomy.md
   - Add CHANGELOG entry

8. **[Phase 8: Validation](phase-8-validation/)** (~45 min)
   - Run quality audits (target: B-grade ≥112/140)
   - Execute linting checks
   - Verify file counts and completeness

9. **[Phase 9: Tessl Integration](phase-9-tessl-integration/)** (~30 min)
   - Review skills with Tessl (target: ≥90%)
   - Optimize as needed
   - Publish to public registry

## Total Effort

**Estimated:** 7-8 hours

## Dependencies

```
Phase 1 (no deps)
  └─> Phase 2
       └─> Phase 3
            └─> Phase 4
                 └─> Phase 5
                      └─> Phase 6
                           └─> Phase 7
                                └─> Phase 8
                                     └─> Phase 9
```

All phases are sequential; each depends on the successful completion of the previous phase.

## Status Tracking

Update the status field in each phase's README.md as work progresses:

- **Pending:** Not started
- **In Progress:** Currently working
- **Completed:** Finished and verified
- **Blocked:** Waiting on dependency or issue resolution

## Navigation

- [Back to Plan Overview](../README.md)
- [Phase 1: Create Directory Structure](phase-1-create-directory-structure/)
