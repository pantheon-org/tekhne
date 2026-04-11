# Scenario 02: Create Hierarchical AGENTS.md for Monorepo

## User Prompt

Your company maintains a monorepo containing multiple packages: a React UI component library, a shared utilities library, and a backend API service. The engineering team has grown frustrated with AI assistants generating incorrect code because they don't understand the project structure, and developers waste time figuring out which commands work in which package.

The tech lead wants you to set up a proper AGENTS.md documentation system. This should include a root AGENTS.md that provides an overview and shared conventions, plus package-specific AGENTS.md files in each subdirectory that contain locally relevant instructions.

## Output Specification

Create a hierarchical AGENTS.md structure:

- Root `AGENTS.md` with monorepo overview, shared conventions, and navigation to sub-packages
- `packages/ui/AGENTS.md` with UI library-specific instructions
- `packages/utils/AGENTS.md` with utilities library-specific instructions
- `packages/api/AGENTS.md` with API service-specific instructions

Ensure:

- Root file is concise (not a full manual)
- No duplication of instructions between files
- Each package file contains locally relevant commands
- Clear references/links between files

## Expected Behavior

1. Run discovery commands to identify monorepo structure, workspaces, and per-package configs
2. Create a root AGENTS.md with shared conventions and navigation links — not a comprehensive manual
3. Create package-level AGENTS.md files under each subdirectory with locally relevant commands and paths
4. Avoid duplicating identical instructions across root and subdirectory files
5. Place universal rules (linting, git workflow) at the root and package-specific details at each local level
6. Use references/links between files instead of repeating content

## Success Criteria

- **Discovery performed**: Agent ran discovery commands to understand monorepo structure (packages, workspaces, etc.)
- **Hierarchical structure**: Created root AGENTS.md plus subdirectory AGENTS.md files (not single flat file)
- **Root concise**: Root AGENTS.md is concise and does not contain encyclopedic content
- **No duplication root-sub**: No identical instructions duplicated between root and subdirectory AGENTS.md files
- **Universal rules at root**: Common/shared conventions (linting, git workflow) are at root level
- **Package-specific at local**: Package-specific instructions are in their respective subdirectory AGENTS.md
- **References used**: Uses references/links to other files rather than duplicating content
- **Nearest-file relevance**: Subdirectory AGENTS.md contains locally relevant commands and paths
- **Path-specific commands**: Commands include specific paths relevant to each package
- **Verified commands**: All commands are valid for the detected project structure
- **Index/links present**: Root file provides clear navigation to subdirectory documentation
- **Quality check**: No broken links or invalid commands in any file

## Failure Conditions

- Creates a single flat AGENTS.md at the root containing all per-package instructions
- Duplicates identical commands or conventions in both the root file and subdirectory files
- Puts package-specific commands (e.g., UI build flags) in the root file rather than the relevant package
- Creates an encyclopedic root AGENTS.md that embeds full framework documentation
- Provides no navigation or cross-references between root and package files
- Uses generic placeholder commands not tied to specific package paths
