# Makefile Generator Reference Documentation

This directory contains comprehensive reference guides for generating production-ready Makefiles.

## Quick Navigation

- [Makefile Structure](./makefile-structure.md) - File organization, layout, includes
- [Variables Guide](./variables-guide.md) - Assignment operators, automatic variables, scope
- [Targets Guide](./targets-guide.md) - Standard targets, .PHONY declarations, prerequisites
- [Patterns Guide](./patterns-guide.md) - Pattern rules, language-specific build patterns
- [Optimization Guide](./optimization-guide.md) - Parallel builds, caching, ccache integration
- [Security Guide](./security-guide.md) - Safe variable expansion, credential handling

## Reading Order

### For New Makefiles

1. Start with [Makefile Structure](./makefile-structure.md) to understand file organization
2. Read [Variables Guide](./variables-guide.md) for variable assignment and scope
3. Review [Targets Guide](./targets-guide.md) for standard GNU targets
4. Check [Patterns Guide](./patterns-guide.md) for your language (C/C++, Go, Python, Java)

### For Production Makefiles

1. Read [Optimization Guide](./optimization-guide.md) for performance improvements
2. Review [Security Guide](./security-guide.md) for credential handling and safe practices

## Common Lookup Tasks

| Task | Reference |
|------|-----------|
| Docker integration | [Patterns Guide § Pattern 8](./patterns-guide.md) |
| Multi-binary projects | [Patterns Guide § Pattern 7](./patterns-guide.md) |
| Go with version embedding | [Patterns Guide § Pattern 5](./patterns-guide.md) |
| Parallel builds, ccache | [Optimization Guide](./optimization-guide.md) |
| Credentials, secrets | [Security Guide](./security-guide.md) |
| Complex dependencies | [Patterns Guide](./patterns-guide.md) |
| Order-only prerequisites | [Optimization Guide](./optimization-guide.md) or [Targets Guide](./targets-guide.md) |
| Variable operators (?=, :=, +=) | [Variables Guide](./variables-guide.md) |

## Usage During Generation

When generating Makefiles, **ALWAYS** use the Read tool to consult relevant references before writing Makefile content. This ensures patterns are current and correctly applied.

Example workflow:
```
1. Read references/patterns-guide.md (find your language pattern)
2. Read references/security-guide.md (for Docker/deploy targets)
3. Generate Makefile combining patterns
4. Document which references were consulted in Makefile header
```
