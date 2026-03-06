# Nx Plugin Authoring - Evaluation Scenarios

Comprehensive evaluation scenarios for testing the `nx-plugin-authoring` skill effectiveness.

## Overview

- **Skill**: nx-plugin-authoring
- **Total Instructions**: 47
- **Scenarios Created**: 5
- **Instructions Tested**: 40 (85.1% coverage)
- **Generation Date**: 2026-03-06

## Scenarios

### Scenario 0: Safe Generator Implementation with Tree API
**Capability**: Safe generator implementation with Tree API  
**Points**: 100  
**Instructions Covered**: 13

Tests proper use of Tree API, schema validation, file existence checks, and path resolution. Focuses on avoiding direct filesystem access and hardcoded paths.

**Key Learning Objectives**:
- Tree API usage over Node.js fs module
- Schema-driven validation with TypeScript types
- File collision detection
- Dynamic path resolution
- Post-generation formatting

### Scenario 1: Cache-Aware Executor with Async I/O
**Capability**: Cache-aware executor with async I/O  
**Points**: 100  
**Instructions Covered**: 14

Tests executor implementation patterns including cache configuration, async file operations, error handling, and logic separation.

**Key Learning Objectives**:
- Proper executor return values
- Package notation vs relative paths
- Cache outputs declaration
- Async file I/O patterns
- Business logic separation
- ExecutorContext usage

### Scenario 2: Template-Driven File Generation with EJS
**Capability**: Template-driven file generation with EJS  
**Points**: 100  
**Instructions Covered**: 4

Tests template system usage including EJS syntax, file naming tokens, and case transformations.

**Key Learning Objectives**:
- generateFiles API usage
- EJS variable substitution
- File naming tokens (**fileName**)
- names() utility for case conversion
- Template extension handling

### Scenario 3: Safe Project Configuration Updates
**Capability**: Safe project configuration updates  
**Points**: 100  
**Instructions Covered**: 7

Tests safe mutation of project configurations including reading before writing, preserving existing state, and boundary checking.

**Key Learning Objectives**:
- Read-modify-write pattern
- Configuration preservation
- Dynamic path resolution from config
- Project boundary validation
- Tag-based access control

### Scenario 4: Plugin Structure and Registration
**Capability**: Plugin structure and registration  
**Points**: 100  
**Instructions Covered**: 2

Tests proper plugin registration including exports, registry files, and package configuration.

**Key Learning Objectives**:
- Plugin entry point exports
- generators.json structure
- executors.json structure
- Package.json configuration
- Discovery mechanism

## Coverage Analysis

### By Instruction Type
- **Anti-patterns**: 12/12 (100%) - All anti-patterns tested
- **Constraints**: 8/8 (100%) - All MUST statements covered
- **Preferences**: 9/12 (75%) - Core preferences covered
- **New Knowledge**: 10/14 (71.4%) - Key patterns covered
- **Reminders**: 0/1 (0%) - Testing utilities not covered

### By Priority
- **Critical**: 10/10 (100%) - All critical instructions tested
- **High**: 15/16 (93.8%) - Nearly all high-priority covered
- **Medium**: 8/12 (66.7%) - Most medium-priority covered
- **Low**: 7/9 (77.8%) - Most low-priority covered

### Untested Instructions (7 total)
1. I035 - Keep plugins as orchestration tools (design principle)
2. I036 - Prefer small composable steps (design principle)
3. I039 - Use addDependenciesToPackageJson (low priority)
4. I042 - Compose other generators (low priority)
5. I043 - Log clearly (low priority)
6. I045 - Use $default with $source:argv (medium priority)
7. I046 - Use x-prompt for interactive prompts (low priority)
8. I047 - Use createTreeWithEmptyWorkspace for testing (reminder)

**Rationale**: Untested instructions are primarily design principles, low-priority utilities, and testing reminders. All critical functionality and anti-patterns are covered.

## Feasibility

All scenarios are feasible within eval constraints:
- ✅ No actual Nx workspace creation required
- ✅ File-based evaluation only
- ✅ No external dependencies or API keys
- ✅ Completable in <10 minutes each
- ✅ No large file generation (>50MB)
- ✅ Gradable by file inspection only

## Scenario Design Quality

### Problem-Framing ✅
All scenarios present problems without revealing the specific instructions being tested. Tasks describe symptoms and requirements rather than solutions.

### Real-World Relevance ✅
Scenarios reflect actual issues encountered when building Nx plugins:
- Generators overwriting files
- Cache not working
- Template maintenance challenges
- Configuration mutation bugs
- Plugin registration failures

### Grading Feasibility ✅
All criteria are objectively verifiable through file inspection:
- API usage patterns
- File structure
- Configuration presence
- Code patterns

### Instruction Coverage ✅
85.1% coverage exceeds the 80% minimum requirement, with strategic focus on:
- All critical anti-patterns (NEVER statements)
- All hard constraints (MUST statements)
- Core workflow patterns
- Common failure modes

## Usage

These scenarios are designed for use with the Tessl eval system:

```bash
# Run single scenario
tessl eval run scenario-0

# Run all scenarios
tessl eval run --all

# Generate report
tessl eval report
```

Each scenario expects the agent to produce specific output files that can be automatically graded against the criteria in `criteria.json`.

## Notes

- Scenarios focus on code structure and patterns, not execution
- No integration testing or runtime validation required
- Emphasis on anti-patterns and constraints ensures safety
- Template scenario includes slight hint (generateFiles mention) which is acceptable for a template-focused eval
- Coverage prioritizes correctness and safety over convenience features
