# Anti-Patterns for AGENTS.md

Detailed explanations of what NEVER to include in AGENTS.md files and why.

## Critical Anti-Patterns with Reasoning

### NEVER Include Welcome Language
**Examples**: "Welcome to...", "This document explains...", "You should..."

**Why this is problematic**:
- **Token waste**: AI systems don't need social pleasantries or introductions
- **Processing overhead**: Every token counts in context window management
- **Scanning inefficiency**: AI systems scan for actionable information, not narrative flow
- **Redundant information**: The fact that it's documentation is already clear from context

### NEVER Use Placeholder Examples
**Examples**: `<your-file>`, `[replace-with-actual-path]`, `TODO: add example`

**Why this is problematic**:
- **Pattern recognition failure**: AI systems learn patterns from concrete examples, not abstractions  
- **Implementation uncertainty**: Placeholders force AI to guess real file structures
- **Context switching**: AI must mentally map placeholders to actual project structure
- **Reduced utility**: Placeholder examples provide no actionable guidance

### NEVER Write Long Prose Paragraphs
**Examples**: Multi-sentence explanations, narrative descriptions, background context

**Why this is problematic**:
- **Scan vs read**: AI systems scan for actionable items, not reading comprehension
- **Information density**: Bullets and code blocks are faster to parse than prose
- **Action extraction**: AI needs to extract "what to do" from narrative text
- **Token efficiency**: Headers + bullets convey same information in fewer tokens

### NEVER Duplicate Content from Skills
**Examples**: Copying testing patterns, development workflows, code standards

**Why this is problematic**:
- **Redundant loading**: Same information loaded multiple times wastes context
- **Maintenance burden**: Updates required in multiple locations
- **Version conflicts**: Outdated copies create inconsistency
- **Skill undermining**: Reduces value of specialized skill system

### NEVER Include Obvious Instructions
**Examples**: "run tests", "write clean code", "handle errors properly"

**Why this is problematic**:
- **Baseline knowledge**: AI systems already know standard development practices  
- **Signal dilution**: Obvious content reduces the signal-to-noise ratio
- **Expert knowledge displacement**: Token space better used for project-specific insights
- **Credibility reduction**: Stating obvious things reduces trust in non-obvious guidance

### NEVER Explain "Why" Unless Non-Obvious
**Examples**: "We use React because it's popular", "Tests are important for quality"

**Why this is problematic**:
- **Action focus**: AI systems need "what to do", not "why it matters"
- **Assumed knowledge**: AI systems understand standard technology rationales
- **Implementation priority**: Limited tokens should focus on "how", not "why"
- **Expert knowledge**: Only include "why" for non-obvious trade-offs and decisions

### NEVER Include Framework Tutorials
**Examples**: "How to write a React component", "Express.js routing basics"

**Why this is problematic**:
- **Baseline competency**: AI systems already know framework fundamentals
- **Documentation duplication**: Official framework docs are authoritative source
- **Project-specific focus**: AGENTS.md should cover project patterns, not framework basics
- **Maintenance overhead**: Framework tutorials become outdated quickly

## Good vs Bad Examples

### ❌ Bad Example
```markdown
# Welcome to Our Project

This document explains how to work with our codebase. You should read this carefully before making changes.

## About React Components
React components are reusable pieces of UI. In our project, we organize them in folders like this:
- `src/components/<ComponentName>/index.tsx` - Your component goes here
- `src/components/<ComponentName>/styles.css` - Your styles go here

Remember to write tests for your components! Testing is important for code quality.
```

### ✅ Good Example  
```markdown
# Project Name

## Component Organization
- Components: `src/components/Button/index.tsx`, `src/components/Modal/index.tsx`
- Styles: Colocated `.css` files
- Tests: `Button/Button.test.tsx`
- Example: Copy pattern from `src/components/Button/`
```

## Usage in Main SKILL.md

Reference this file with explicit loading trigger:

```markdown
**MANDATORY - READ ENTIRE FILE**: Before writing any AGENTS.md content, 
load `references/anti-patterns.md` to understand what to avoid.

**Do NOT load** other reference files for anti-pattern review.
```