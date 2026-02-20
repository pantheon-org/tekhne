---
name: commanderjs
description: |-
  Complete Commander.js CLI framework guidance covering command structure, options, arguments, subcommands, action handlers, version management, and TypeScript integration. Use when: building CLI tools, parsing command-line arguments, implementing subcommands, handling options/flags, creating interactive CLIs, or migrating from other CLI frameworks.
  
  Keywords: Commander.js, CLI, command-line, arguments, options, flags, subcommands, action handlers, version, help text, TypeScript, yargs, meow, program, parseAsync, opts, args, variadic, required options, default values, custom help, error handling
allowed-tools:
  - Read
  - Write
  - Edit
  - Bash
---

# Commander.js

Complete Commander.js framework guidance for building robust command-line interfaces with proper argument parsing, subcommands, options, and TypeScript support.

## When to Apply

Use this skill when:
- Building CLI tools with Commander.js
- Parsing command-line arguments and options
- Implementing subcommands and nested command structures
- Handling required/optional options and variadic arguments
- Creating custom help text and version information
- Implementing action handlers with async/await
- Integrating Commander.js with TypeScript
- Migrating from other CLI frameworks (yargs, meow)
- Handling CLI errors and validation
- Testing CLI applications

## Categories by Priority

| Priority | Category | Impact | Prefix |
|----------|----------|--------|--------|
| 1 | Core Concepts | CRITICAL | `core-` |
| 2 | Options & Arguments | CRITICAL | `options-` |
| 3 | Commands & Subcommands | HIGH | `commands-` |
| 4 | Action Handlers | HIGH | `actions-` |
| 5 | TypeScript Integration | MEDIUM | `typescript-` |
| 6 | Best Practices | MEDIUM | `practices-` |

## How to Use

Read individual reference files for detailed guidance:

```
references/core-basics.md
references/options-flags.md
references/commands-structure.md
references/actions-handlers.md
references/typescript-setup.md
references/practices-patterns.md
```

Each reference file contains:
- API documentation and usage patterns
- Practical code examples with TypeScript
- Common patterns and best practices
- Error handling and validation techniques
- Migration guides from other frameworks

## Quick Start

```typescript
import { Command } from 'commander';

const program = new Command();

program
  .name('my-cli')
  .description('CLI tool description')
  .version('1.0.0');

program
  .command('build')
  .description('Build the project')
  .option('-o, --output <path>', 'Output directory')
  .option('-w, --watch', 'Watch for changes')
  .action((options) => {
    console.log('Building with options:', options);
  });

await program.parseAsync(process.argv);
```

## Navigation Workflow

1. **Start with core** - Understand program setup and basic structure
2. **Add options** - Define flags, required options, and defaults
3. **Structure commands** - Create subcommands and command hierarchies
4. **Implement actions** - Write action handlers with async/await
5. **Integrate TypeScript** - Add proper typing and type safety
6. **Apply best practices** - Error handling, validation, and testing

## Common Patterns

### Single Command CLI

```typescript
program
  .argument('<source>', 'Source file')
  .argument('[destination]', 'Destination file')
  .option('-f, --force', 'Force overwrite')
  .action((source, destination, options) => {
    // implementation
  });
```

### Multi-Command CLI

```typescript
program
  .command('init')
  .description('Initialize project')
  .action(() => { /* ... */ });

program
  .command('deploy')
  .description('Deploy application')
  .option('-e, --env <name>', 'Environment')
  .action((options) => { /* ... */ });
```

### Modular Subcommands with Typed Options (Recommended)

```typescript
// types/build-options.ts
export interface BuildOptions {
  outDir: string;
  minify: boolean;
  watch: boolean;
}

// services/build-service.ts
import type { BuildOptions } from '../types/build-options';

export const buildProject = (options: BuildOptions): void => {
  console.log('Building to:', options.outDir);
  // All options available with type safety
};

// commands/build.ts
import { Command } from 'commander';
import { buildProject } from '../services/build-service';
import type { BuildOptions } from '../types/build-options';

export const buildCommand = new Command('build')
  .description('Build project')
  .option('-o, --out-dir <path>', 'Output directory', 'dist')
  .option('-m, --minify', 'Minify output', false)
  .option('-w, --watch', 'Watch mode', false)
  .action((options: BuildOptions) => {
    // Pass complete typed object to service
    buildProject(options);
  });

// index.ts
import { buildCommand } from './commands/build';

program.addCommand(buildCommand);
```

## Do's

✓ **ALWAYS pass complete typed options objects to services** (never individual properties)
✓ **Define TypeScript interfaces for all options** (e.g., `BuildOptions`, `DeployOptions`)
✓ Export commands as Command instances from subcommand modules
✓ Use `.addCommand()` to attach subcommands to parent Command
✓ Use `parseAsync()` for async action handlers
✓ Validate options in action handlers
✓ Provide clear descriptions for all commands/options
✓ Use TypeScript for type safety
✓ Handle errors gracefully with try/catch
✓ Use kebab-case for option names
✓ Provide sensible defaults for optional options
✓ Create barrel exports for commands (commands/index.ts)
✓ Test CLI with different argument combinations
✓ Use `.exitOverride()` for testing
✓ Document expected argument formats

## Don'ts

✗ **NEVER pass individual option properties to services** (pass complete typed object)
✗ **DON'T pass options piecemeal** (e.g., `service(opts.a, opts.b, opts.c)`)
✗ Don't use `parse()` with async handlers (use `parseAsync()`)
✗ Don't ignore error handling in action handlers
✗ Don't use camelCase in CLI flags (use kebab-case)
✗ Don't forget to specify option argument types (`<required>` vs `[optional]`)
✗ Don't mix positional arguments with options ambiguously
✗ Don't forget to call `program.parse()` or `program.parseAsync()`
✗ Don't use global state in action handlers
✗ Don't suppress built-in help text without good reason
✗ Don't forget to version your CLI
✗ Don't make all options required (use sensible defaults)

## References

- https://github.com/tj/commander.js
- https://github.com/tj/commander.js/blob/master/Readme.md
- https://github.com/tj/commander.js/tree/master/examples
