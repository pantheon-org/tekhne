---
name: commanderjs
description: |-
  Complete Commander.js CLI framework guidance covering command structure, options, arguments, subcommands, action handlers, version management, and TypeScript integration. Use when: building CLI tools, parsing command-line arguments, implementing subcommands, handling options/flags, creating interactive CLIs, or migrating from other CLI frameworks.
  
  Keywords: Commander.js, CLI, command-line, arguments, options, flags, subcommands, action handlers, version, help text, TypeScript, yargs, meow, program, parseAsync, opts, args, variadic, required options, default values, custom help, error handling
allowed-tools: Read, Write, Edit, Bash
---

# Commander.js

Complete Commander.js framework guidance for building robust command-line interfaces with proper argument parsing, subcommands, options, and TypeScript support.

## Mindset

Treat Commander.js as a declarative contract, not an argv reader. Model every input the CLI accepts as a `.argument()`, `.option()`, or subcommand before writing a single line of handler logic — the declaration is what generates `--help` output, coerces types, and produces Commander's own validation errors for free. Never reach for `process.argv[n]` as a shortcut past that declaration: every value read directly from argv is a value Commander no longer validates, documents, or reports errors for consistently with the rest of the program. Keep action handlers thin. Parse into a fully typed options object first, then pass that single object into a service function — never scatter individual `opts.foo`, `opts.bar` properties across a call site, because a service that grows a fourth or fifth positional parameter loses the compiler's ability to catch an argument-order mistake. Verify subcommand option scope before trusting `program.opts()` inside a subcommand action: each `.command()` owns its own option scope, and reading the parent program's `opts()` from inside a child action silently returns the wrong values instead of failing loudly.

## When to Use

Use this skill when building a new CLI tool with Commander.js, adding subcommands, options, or arguments to an existing Commander.js program, migrating a CLI from raw `process.argv` parsing or another framework (yargs, meow) onto Commander.js, or diagnosing an option/argument/action handler that is not behaving as declared (wrong scope, a missing `await`, argv leaking through, help text not matching actual parsing behavior).

## When NOT to Use

Do not reach for this skill when the target CLI deliberately avoids third-party argument-parsing dependencies, when the runtime's own built-in flag parser is the project's stated convention, or when the task is building a terminal UI (menus, prompts, live-updating panes) rather than parsing arguments — a TUI is a different problem from argument parsing and needs different tooling.

## How to Use

Read individual reference files for detailed guidance:

- [Core Basics](references/core-basics.md)
- [Options & Flags](references/options-flags.md)
- [Commands & Structure](references/commands-structure.md)
- [Action Handlers](references/actions-handlers.md)
- [TypeScript Setup](references/typescript-setup.md)
- [Practices & Patterns](references/practices-patterns.md)

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
6. **Validate parsing** - Test with `--help` and invalid inputs to verify argument parsing and error handling
7. **Apply best practices** - Error handling, validation, and testing

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

## Anti-Patterns

### NEVER access `process.argv` directly when Commander.js is available

- **WHY:** Commander.js handles argument parsing, validation, and help generation; bypassing it for any argument creates inconsistency in error handling and help output.
- **BAD**: `const url = process.argv[2]` alongside Commander.js commands.
- **GOOD**: Define all arguments as Commander.js options or arguments: `program.argument('<url>', 'Target URL')`.

### NEVER use `.action()` callback without handling errors

- **WHY:** Unhandled rejections in async action callbacks crash the process without helpful error messages.
- **BAD**: `program.command('fetch').action(async (opts) => { await riskyOp(); })`
- **GOOD**: Wrap in try/catch and call `program.error(err.message)` for Commander-formatted error output.

### NEVER add `.parseAsync()` without `await`

- **WHY:** Commander.js v8+ requires `await program.parseAsync()` for async actions; without await, the process exits before async actions complete.
- **BAD**: `program.parseAsync(process.argv)` without await.
- **GOOD**: `await program.parseAsync(process.argv)` inside an async IIFE or main function.

### NEVER define commands with positional arguments and options that share ambiguous prefixes

- **WHY:** Commander.js can misparse `--option` values as positional arguments when options are not consumed before positional parsing.
- **BAD**: `program.argument('<file>').option('--format <fmt>')` with ambiguous ordering in usage examples.
- **GOOD**: Always place options before positional arguments in usage examples and validate input explicitly.

### NEVER use `program.opts()` to read option values inside a subcommand

- **WHY:** Each subcommand has its own option scope; reading `program.opts()` in a subcommand returns the parent options, not the subcommand options.
- **BAD**: Reading `program.opts().verbose` inside a `program.command('deploy').action()`.
- **GOOD**: Use `command.opts()` (the action's first argument when using `.action((opts) =>)`) to access subcommand-specific options.

## References

- [Commander.js GitHub Repository](https://github.com/tj/commander.js)
- [Commander.js Documentation](https://github.com/tj/commander.js/blob/master/Readme.md)
- [Commander.js Examples](https://github.com/tj/commander.js/tree/master/examples)
