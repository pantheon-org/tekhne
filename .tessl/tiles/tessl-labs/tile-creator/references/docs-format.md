# Documentation Format

Docs are facts and knowledge that agents query on-demand (like RAG). Use for API docs, guides, tutorials, package documentation.

## Design Principles

### Progressive Disclosure

Structure docs so agents load only what they need:

1. **index.md** - Overview + navigation (always loaded first)
2. **Topic pages** - Detailed content (loaded on-demand)

```
docs/
├── index.md              # Summary + links to all pages
├── getting-started.md    # Loaded when user asks about setup
├── api-reference.md      # Loaded when user asks about API
└── examples/
    ├── basic.md          # Loaded for basic examples
    └── advanced.md       # Loaded for advanced patterns
```

The agent reads index.md first, then follows links based on the user's question.

### Token Efficiency

Agents share context with conversation history, system prompts, and user requests. Every token costs.

**Do:**

- Prefer concise examples over verbose explanations
- Use tables for structured data (parameters, options)
- Use code blocks liberally - they're information-dense
- Let headings and structure convey organization

**Don't:**

- Explain concepts the agent already knows
- Repeat information across pages
- Include prose that could be a bullet point

### Comprehensive Coverage

Aim for **full coverage** of the source material. Docs should be complete enough that the agent rarely needs external sources.

- Document all public APIs, methods, and options
- Include examples for common use cases
- Cover edge cases and error handling
- Don't omit features to save tokens - structure efficiently instead

## Entry Point (index.md)

The `index.md` must:

1. Provide a brief overview (what this is, what it does)
2. Link to ALL other files using relative markdown links

```markdown
# My Library

TypeScript SDK for the Example API.

## Installation

\`\`\`bash
npm install example-sdk
\`\`\`

## Quick Start

\`\`\`typescript
import { Client } from 'example-sdk';
const client = new Client({ apiKey: process.env.API_KEY });
const result = await client.query('hello');
\`\`\`

## Contents

- [Authentication](./auth.md) - API keys, OAuth, tokens
- [Core API](./core-api.md) - Main methods and responses
- [Streaming](./streaming.md) - Real-time responses
- [Error Handling](./errors.md) - Error codes and recovery
```

**Critical**: Every `.md` file must be reachable via links from `index.md`.

## File Naming

Use descriptive, lowercase names with hyphens:

- `chat-completions.md` (good)
- `ChatCompletions.md` (bad)
- `api.md` (too vague)

Agents use filenames to decide what to read - make them self-explanatory.

## Content Structure

Within each file:

```markdown
# Page Title

Brief description of what this page covers.

## Section 1

Content organized by topic.

### Subsection

More specific details.

## Section 2

Next topic.
```

Use headings liberally - they help agents navigate and understand scope.

## tile.json Configuration

```json
{
  "name": "workspace/my-docs",
  "version": "1.0.0",
  "summary": "Documentation for Example SDK",
  "docs": "docs/index.md"
}
```

For software package tiles, add `describes`:

```json
{
  "name": "tessl/npm-example",
  "version": "2.0.0",
  "docs": "docs/index.md",
  "describes": "pkg:npm/example-sdk@2.0.0",
  "summary": "TypeScript SDK for Example API"
}
```
