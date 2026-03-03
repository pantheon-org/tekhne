# Troubleshooting Common Issues

Common problems when creating AGENTS.md files and their solutions.

## Issue: Unclear Monorepo Structure

**Problem**: Complex repository with unclear package boundaries or mixed patterns
**Symptoms**: 
- Packages have different tech stacks
- Unclear which directories need own AGENTS.md
- Multiple build systems in use

**Solution**:
1. Start with root AGENTS.md only
2. Add sub-files iteratively based on usage
3. Use JIT index approach to defer complexity
4. Focus on most-used packages first

**Fallback**: Create minimal root file pointing to package-specific documentation

## Issue: Too Many Local Skills to Reference

**Problem**: Project has 10+ local skills, overwhelming to list all
**Symptoms**:
- Skills list dominates AGENTS.md content
- Unclear which skills apply to which scenarios
- Maintenance burden keeping list updated

**Solution**:
1. Group skills by category (testing, deployment, etc.)
2. Create skill index files in .agents/skills/
3. Reference index files, not individual skills
4. Use conditional loading based on file types

**Example**:
```markdown
## Local Skills
- Frontend: See `.agents/skills/frontend/index.md`
- Backend: See `.agents/skills/backend/index.md`
- DevOps: Use `deployment` skill for CI/CD tasks
```

## Issue: Mixed Technology Stacks

**Problem**: Repository contains Python, Node.js, Go, etc. with different patterns
**Symptoms**:
- Universal conventions don't apply
- Different package managers and build tools
- Language-specific patterns conflict

**Solution**:
1. Create language-specific subdirectory AGENTS.md files
2. Keep root file minimal with routing only
3. Use technology-specific templates from references/
4. Avoid forcing universal patterns

**Structure**:
```
AGENTS.md (routing only)
services/api-go/AGENTS.md (Go-specific)
services/web-node/AGENTS.md (Node.js-specific)
```

## Issue: Existing Documentation Conflicts

**Problem**: Project already has README files, wiki pages, or other docs
**Symptoms**:
- Information duplication
- Conflicting instructions
- Unclear which source is authoritative

**Solution**:
1. Reference existing docs, don't duplicate
2. Focus AGENTS.md on AI-specific needs
3. Link to authoritative sources
4. Avoid project overview content

**Pattern**:
```markdown
## Architecture
See [Architecture Decision Records](docs/adr/) for design decisions

## API Documentation  
Authoritative API docs: [OpenAPI spec](api/openapi.yaml)
```

## Issue: Frequent Changes Break Documentation

**Problem**: AGENTS.md becomes outdated quickly due to rapid development
**Symptoms**:
- File paths in examples no longer exist
- Commands don't work
- Build process changed but docs didn't update

**Solution**:
1. Use patterns instead of specific examples where possible
2. Reference configuration files instead of hardcoding values
3. Focus on stable patterns, not volatile implementation details
4. Add validation checks to CI/CD

**Stable patterns**:
```markdown
# Good - pattern-based
- Components: `src/components/**/*.tsx`
- Tests: Colocated `*.test.tsx`

# Bad - specific examples that break
- Button: `src/components/Button/Button.tsx`
- Modal: `src/components/Modal/Modal.tsx`
```

## Issue: Team Disagreement on Conventions

**Problem**: Team members want different approaches documented
**Symptoms**:
- Conflicting style guides
- Multiple "right ways" to do things
- AGENTS.md becomes battleground for opinions

**Solution**:
1. Document current patterns, not ideal patterns
2. Show examples from existing codebase
3. Avoid prescriptive language ("should", "must")
4. Focus on consistency over perfection

**Approach**:
```markdown
## Current Patterns
- New components: Follow `src/components/Button/` structure
- Legacy components: See `src/legacy/` - avoid this pattern
- State management: Redux Toolkit (see `src/store/`)
```