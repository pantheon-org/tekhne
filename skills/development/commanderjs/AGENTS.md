# Commander.js Reference Guide

Complete reference for building command-line interfaces with Commander.js.

## Core Patterns

### Modular Command Structure (Recommended)

Always export Command instances from modules and use `.addCommand()`:

```typescript
// commands/build.ts
import { Command } from 'commander';

export const buildCommand = new Command('build')
  .description('Build project')
  .option('-e, --env <name>', 'Environment', 'development')
  .action((options) => {
    console.log('Building:', options);
  });

// index.ts
import { Command } from 'commander';
import { buildCommand } from './commands/build';
import packageJson from './package.json' with { type: 'json' };

const program = new Command();
program
  .name('my-cli')
  .version(packageJson.version)
  .addCommand(buildCommand);

await program.parseAsync(process.argv);
```

### Typed Options Pattern (REQUIRED)

Always pass typed option objects from commands to service layer:

```typescript
// types/build-options.ts
export type BuildOptions = {
  env: string;
  output: string;
  sourcemap: boolean;
};

// services/build-service.ts
export const buildProject = (options: BuildOptions): void => {
  console.log('Building with:', options);
};

// commands/build.ts
import { Command } from 'commander';
import { buildProject } from '../services/build-service';
import type { BuildOptions } from '../types/build-options';

export const buildCommand = new Command('build')
  .description('Build project')
  .option('-e, --env <name>', 'Environment', 'development')
  .option('-o, --output <dir>', 'Output directory', 'dist')
  .option('--sourcemap', 'Generate sourcemaps', false)
  .action((options: BuildOptions) => {
    buildProject(options); // Type-safe!
  });
```

**Why:** Type safety prevents errors, documents API contracts, enables refactoring.

### Subcommands with Barrel Exports

```typescript
// commands/db/migrate.ts
export const migrateCommand = new Command('migrate')
  .description('Run migrations')
  .action(() => { /* ... */ });

// commands/db/seed.ts
export const seedCommand = new Command('seed')
  .description('Seed database')
  .action(() => { /* ... */ });

// commands/db/index.ts
import { Command } from 'commander';
import { migrateCommand } from './migrate';
import { seedCommand } from './seed';

export const dbCommand = new Command('db')
  .description('Database operations')
  .addCommand(migrateCommand)
  .addCommand(seedCommand);

// index.ts
import { dbCommand } from './commands/db';
program.addCommand(dbCommand);
```

## Essential Patterns

### Program Setup

```typescript
import { Command } from 'commander';
import packageJson from './package.json' with { type: 'json' };

const program = new Command();

program
  .name('my-cli')
  .description('CLI tool')
  .version(packageJson.version); // Load from package.json

await program.parseAsync(process.argv);
```

### Options

```typescript
program
  .option('-p, --port <number>', 'Port', '3000')
  .option('-f, --force', 'Force operation')
  .requiredOption('-c, --config <path>', 'Config file');
```

### Arguments

```typescript
program
  .argument('<source>', 'Source file')
  .argument('[destination]', 'Destination file')
  .argument('<files...>', 'Files to process');
```

### Action Handlers (Async)

```typescript
program
  .command('deploy')
  .action(async (options) => {
    try {
      await deploy(options);
    } catch (error) {
      console.error('Deployment failed:', error.message);
      process.exit(1);
    }
  });

await program.parseAsync(process.argv); // Must use parseAsync
```

### TypeScript Types with Service Layer

```typescript
// types/build-options.ts
export type BuildOptions = {
  env: string;
  minify: boolean;
};

// services/build-service.ts
export const buildProject = (options: BuildOptions): void => {
  console.log('Building:', options);
};

// commands/build.ts
import type { BuildOptions } from '../types/build-options';
import { buildProject } from '../services/build-service';

export const buildCommand = new Command('build')
  .option('-e, --env <name>', 'Environment', 'development')
  .option('-m, --minify', 'Minify', false)
  .action((options: BuildOptions) => {
    buildProject(options); // Pass all options to service
  });
```

## Directory Structure

```
src/
├── commands/
│   ├── build.ts          # Export buildCommand
│   ├── deploy.ts         # Export deployCommand
│   ├── db/
│   │   ├── migrate.ts    # Export migrateCommand
│   │   ├── seed.ts       # Export seedCommand
│   │   └── index.ts      # Export dbCommand
│   └── index.ts          # Barrel exports
└── index.ts              # Main program
```

## Key Rules

1. **Export Command instances** - Never use inline command definitions for large CLIs
2. **Use `.addCommand()`** - Attach subcommands using this method
3. **Create barrel exports** - Export commands from `commands/index.ts`
4. **One command per file** - Follow single function per module pattern
5. **Use `parseAsync()`** - Always use async parsing for async handlers
6. **Type your options** - Define TypeScript interfaces for options
7. **Handle errors** - Use try/catch in action handlers
8. **Validate inputs** - Check arguments and options in handlers
9. **Use kebab-case** - All CLI flags should use kebab-case
10. **Provide defaults** - Give sensible defaults for optional options

## Common Anti-Patterns

❌ **Inline subcommands** (harder to test/maintain):
```typescript
const db = program.command('db');
db.command('migrate').action(() => {});
```

✅ **Modular subcommands**:
```typescript
export const migrateCommand = new Command('migrate').action(() => {});
export const dbCommand = new Command('db').addCommand(migrateCommand);
```

❌ **Using `parse()` with async**:
```typescript
program.action(async () => {});
program.parse(process.argv); // Wrong
```

✅ **Using `parseAsync()`**:
```typescript
program.action(async () => {});
await program.parseAsync(process.argv); // Correct
```

❌ **No error handling**:
```typescript
program.action(async () => {
  await deploy(); // What if this throws?
});
```

✅ **Proper error handling**:
```typescript
program.action(async () => {
  try {
    await deploy();
  } catch (error) {
    console.error('Error:', error.message);
    process.exit(1);
  }
});
```

## Testing Pattern

```typescript
import { Command } from 'commander';

export const createProgram = (): Command => {
  const program = new Command();
  program.exitOverride(); // Throw instead of exit
  
  // Add commands
  
  return program;
};

// In tests
describe('CLI', () => {
  it('should parse options', async () => {
    const program = createProgram();
    await program.parseAsync(['node', 'test', '--port', '8080']);
    expect(program.opts().port).toBe('8080');
  });
});
```

## Reference Files

For detailed documentation, see:
- `references/core-basics.md` - Program setup, parsing, help
- `references/options-flags.md` - Options, defaults, validation
- `references/commands-structure.md` - Modular commands, subcommands
- `references/actions-handlers.md` - Async handlers, error handling
- `references/typescript-setup.md` - TypeScript integration, types
- `references/practices-patterns.md` - Patterns, testing, best practices

## Quick Reference

| Concept | Pattern | Example |
|---------|---------|---------|
| Program | `new Command()` | `const program = new Command()` |
| Command | Export from module | `export const cmd = new Command('name')` |
| Attach | `.addCommand()` | `program.addCommand(buildCommand)` |
| Option | `.option()` | `.option('-p, --port <n>', 'Port', '3000')` |
| Required | `.requiredOption()` | `.requiredOption('-c, --config <path>')` |
| Argument | `.argument()` | `.argument('<source>', 'Source file')` |
| Action | `.action()` | `.action(async (opts) => { /* ... */ })` |
| Parse | `parseAsync()` | `await program.parseAsync(process.argv)` |
| Type | Interface | `action((opts: BuildOptions) => {})` |

## External Resources

- GitHub: https://github.com/tj/commander.js
- Documentation: https://github.com/tj/commander.js/blob/master/Readme.md
- Examples: https://github.com/tj/commander.js/tree/master/examples
