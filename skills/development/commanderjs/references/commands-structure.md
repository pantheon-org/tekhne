# Commands & Subcommands

Complete guide to structuring commands and subcommands in Commander.js.

## CRITICAL: Typed Options Pattern

**Always pass options as typed objects to downstream services/modules.**

### Bad: Destructuring in Action
```typescript
program
  .command('build')
  .option('-e, --env <name>', 'Environment')
  .action(({ env }) => {
    // ❌ BAD: Type information lost, unclear what's passed
    buildProject(env);
  });
```

### Good: Typed Options Object
```typescript
// types/build-options.ts
export interface BuildOptions {
  env: string;
  minify: boolean;
  outDir: string;
}

// services/build-service.ts
export const buildProject = (options: BuildOptions): void => {
  console.log('Building with:', options);
  // Service knows exactly what it receives
};

// commands/build.ts
import { Command } from 'commander';
import { buildProject } from '../services/build-service';
import type { BuildOptions } from '../types/build-options';

export const buildCommand = new Command('build')
  .description('Build the project')
  .option('-e, --env <name>', 'Environment', 'development')
  .option('-m, --minify', 'Minify output', false)
  .option('-o, --out-dir <path>', 'Output directory', './dist')
  .action((options: BuildOptions) => {
    // ✅ GOOD: Full typed object passed to service
    buildProject(options);
  });
```

### Why Typed Options?

1. **Type Safety**: Compiler catches missing/incorrect options
2. **Clarity**: Service signature shows all required inputs
3. **Testability**: Easy to mock complete option objects
4. **Maintainability**: Add options without changing signatures
5. **Documentation**: Types document expected inputs

## Basic Command

```typescript
program
  .command('build')
  .description('Build the project')
  .action(() => {
    console.log('Building...');
  });
```

## Multiple Commands

```typescript
program
  .command('init')
  .description('Initialize project')
  .action(() => {
    console.log('Initializing...');
  });

program
  .command('build')
  .description('Build project')
  .action(() => {
    console.log('Building...');
  });

program
  .command('deploy')
  .description('Deploy project')
  .action(() => {
    console.log('Deploying...');
  });

await program.parseAsync(process.argv);
```

## Command with Arguments

```typescript
program
  .command('deploy')
  .description('Deploy to environment')
  .argument('<environment>', 'Target environment')
  .action((environment) => {
    console.log('Deploying to:', environment);
  });

// Usage: my-cli deploy production
```

## Multiple Arguments

```typescript
program
  .command('copy')
  .description('Copy files')
  .argument('<source>', 'Source file')
  .argument('<destination>', 'Destination file')
  .action((source, destination) => {
    console.log(`Copying ${source} to ${destination}`);
  });

// Usage: my-cli copy src.txt dst.txt
```

## Optional Arguments

```typescript
program
  .command('log')
  .description('Show logs')
  .argument('[lines]', 'Number of lines', '10')
  .action((lines) => {
    console.log(`Showing ${lines} lines`);
  });

// Usage: my-cli log
// or:    my-cli log 50
```

## Variadic Arguments

```typescript
program
  .command('add')
  .description('Add files')
  .argument('<files...>', 'Files to add')
  .action((files) => {
    console.log('Adding files:', files);
  });

// Usage: my-cli add file1.js file2.js file3.js
```

## Command with Options

```typescript
program
  .command('build')
  .description('Build project')
  .option('-e, --env <name>', 'Environment', 'development')
  .option('-w, --watch', 'Watch mode')
  .option('-m, --minify', 'Minify output')
  .action((options) => {
    console.log('Building with options:', options);
  });

// Usage: my-cli build --env production --minify
```

## Modular Commands (Recommended Pattern)

**Create subcommands as exported Command instances, with typed options passed to services:**

### Full Stack: Types → Service → Command

```typescript
// types/build-options.ts
export interface BuildOptions {
  env: string;
  minify: boolean;
  outDir: string;
}

// services/build-service.ts
import type { BuildOptions } from '../types/build-options';

export const buildProject = (options: BuildOptions): void => {
  console.log('Building with:', options);
  // Implementation receives fully typed options
};

// commands/build.ts
import { Command } from 'commander';
import { buildProject } from '../services/build-service';
import type { BuildOptions } from '../types/build-options';

export const buildCommand = new Command('build')
  .description('Build project')
  .option('-e, --env <name>', 'Environment', 'development')
  .option('-m, --minify', 'Minify output', false)
  .option('-o, --out-dir <path>', 'Output directory', './dist')
  .action((options: BuildOptions) => {
    // Pass complete typed options object to service
    buildProject(options);
  });

// types/deploy-options.ts
export interface DeployOptions {
  branch: string;
  force: boolean;
}

// services/deploy-service.ts
import type { DeployOptions } from '../types/deploy-options';

export const deployApp = (environment: string, options: DeployOptions): void => {
  console.log(`Deploying ${options.branch} to ${environment}`);
  // Service receives environment + typed options
};

// commands/deploy.ts
import { Command } from 'commander';
import { deployApp } from '../services/deploy-service';
import type { DeployOptions } from '../types/deploy-options';

export const deployCommand = new Command('deploy')
  .description('Deploy application')
  .argument('<environment>', 'Target environment')
  .option('-b, --branch <name>', 'Branch', 'main')
  .option('-f, --force', 'Force deployment', false)
  .action((environment: string, options: DeployOptions) => {
    // Pass all arguments + typed options to service
    deployApp(environment, options);
  });

// index.ts
import { Command } from 'commander';
import { buildCommand } from './commands/build';
import { deployCommand } from './commands/deploy';

const program = new Command();

program
  .name('my-cli')
  .version('1.0.0')
  .addCommand(buildCommand)
  .addCommand(deployCommand);

await program.parseAsync(process.argv);
```

### Directory Structure with Service Layer

```
src/
├── types/
│   ├── build-options.ts      # BuildOptions interface
│   ├── deploy-options.ts     # DeployOptions interface
│   └── index.ts              # Barrel exports
├── services/
│   ├── build-service.ts      # buildProject(options: BuildOptions)
│   ├── deploy-service.ts     # deployApp(env, options: DeployOptions)
│   └── index.ts              # Barrel exports
├── commands/
│   ├── build.ts              # Export buildCommand
│   ├── deploy.ts             # Export deployCommand
│   └── index.ts              # Barrel exports
└── index.ts                  # Main program
```

## Barrel Exports for Commands

```typescript
// commands/index.ts
export { buildCommand } from './build';
export { deployCommand } from './deploy';
export { testCommand } from './test';

// index.ts
import { Command } from 'commander';
import { buildCommand, deployCommand, testCommand } from './commands';

const program = new Command();

program
  .name('my-cli')
  .version('1.0.0')
  .addCommand(buildCommand)
  .addCommand(deployCommand)
  .addCommand(testCommand);

await program.parseAsync(process.argv);
```

## Subcommands with Modular Pattern + Typed Options

**Subcommands follow the same pattern: types → services → commands:**

```typescript
// types/migrate-options.ts
export interface MigrateOptions {
  rollback: boolean;
  steps?: number;
}

// types/seed-options.ts
export interface SeedOptions {
  clear: boolean;
}

// services/database-service.ts
import type { MigrateOptions } from '../types/migrate-options';
import type { SeedOptions } from '../types/seed-options';

export const runMigrations = (options: MigrateOptions): void => {
  if (options.rollback) {
    console.log('Rolling back migration');
  } else {
    console.log('Running migrations');
  }
};

export const seedDatabase = (env: string, options: SeedOptions): void => {
  console.log('Seeding database for:', env);
  if (options.clear) {
    console.log('Clearing existing data first');
  }
};

// commands/db/migrate.ts
import { Command } from 'commander';
import { runMigrations } from '../../services/database-service';
import type { MigrateOptions } from '../../types/migrate-options';

export const migrateCommand = new Command('migrate')
  .description('Run database migrations')
  .option('--rollback', 'Rollback last migration', false)
  .option('--steps <number>', 'Number of migrations')
  .action((options: MigrateOptions) => {
    runMigrations(options);
  });

// commands/db/seed.ts
import { Command } from 'commander';
import { seedDatabase } from '../../services/database-service';
import type { SeedOptions } from '../../types/seed-options';

export const seedCommand = new Command('seed')
  .description('Seed database')
  .argument('[env]', 'Environment', 'development')
  .option('--clear', 'Clear existing data', false)
  .action((env: string, options: SeedOptions) => {
    seedDatabase(env, options);
  });

// commands/db/index.ts
import { Command } from 'commander';
import { migrateCommand } from './migrate';
import { seedCommand } from './seed';

export const dbCommand = new Command('db')
  .description('Database operations')
  .addCommand(migrateCommand)
  .addCommand(seedCommand);

// index.ts
import { Command } from 'commander';
import { dbCommand } from './commands/db';

const program = new Command();

program
  .name('my-cli')
  .version('1.0.0')
  .addCommand(dbCommand);

await program.parseAsync(process.argv);

// Usage: my-cli db migrate --rollback
//        my-cli db seed production --clear
```

## Inline Subcommands (Less Maintainable)

```typescript
const db = program.command('db');
db.description('Database commands');

db.command('migrate')
  .description('Run migrations')
  .action(() => {
    console.log('Running migrations...');
  });

db.command('seed')
  .description('Seed database')
  .action(() => {
    console.log('Seeding database...');
  });

// Usage: my-cli db migrate
//        my-cli db seed
```

## Nested Subcommands

```typescript
// commands/docker/container/start.ts
import { Command } from 'commander';

export const startCommand = new Command('start')
  .description('Start container')
  .argument('<name>', 'Container name')
  .action((name) => {
    console.log('Starting container:', name);
  });

// commands/docker/container/stop.ts
import { Command } from 'commander';

export const stopCommand = new Command('stop')
  .description('Stop container')
  .argument('<name>', 'Container name')
  .action((name) => {
    console.log('Stopping container:', name);
  });

// commands/docker/container/index.ts
import { Command } from 'commander';
import { startCommand } from './start';
import { stopCommand } from './stop';

export const containerCommand = new Command('container')
  .description('Container operations')
  .addCommand(startCommand)
  .addCommand(stopCommand);

// commands/docker/index.ts
import { Command } from 'commander';
import { containerCommand } from './container';

export const dockerCommand = new Command('docker')
  .description('Docker operations')
  .addCommand(containerCommand);

// index.ts
import { Command } from 'commander';
import { dockerCommand } from './commands/docker';

const program = new Command();

program
  .name('my-cli')
  .version('1.0.0')
  .addCommand(dockerCommand);

await program.parseAsync(process.argv);

// Usage: my-cli docker container start my-app
//        my-cli docker container stop my-app
```

## Command Aliases

```typescript
export const installCommand = new Command('install')
  .alias('i')
  .description('Install dependencies')
  .action(() => {
    console.log('Installing...');
  });

// Usage: my-cli install
// or:    my-cli i
```

## Default Command

```typescript
export const serveCommand = new Command('serve')
  .description('Start server')
  .action(() => {
    console.log('Starting server...');
  });

program.addCommand(serveCommand, { isDefault: true });

// Usage: my-cli
// (runs serve command by default)
```

## Hidden Commands

```typescript
export const debugCommand = new Command('debug')
  .description('Debug mode')
  .hideCommand()
  .action(() => {
    console.log('Debug mode enabled');
  });

// Hidden from help text
```

## Command-Specific Help

```typescript
export const deployCommand = new Command('deploy')
  .description('Deploy application')
  .argument('<environment>', 'Target environment')
  .option('-d, --dry-run', 'Dry run')
  .addHelpText('after', `
Examples:
  $ my-cli deploy production
  $ my-cli deploy staging --dry-run
  `);
```

## TypeScript Command Factory Pattern

```typescript
// commands/build.ts
import { Command } from 'commander';

interface BuildOptions {
  env: string;
  minify: boolean;
  watch: boolean;
}

export const buildCommand = new Command('build')
  .description('Build project')
  .option('-e, --env <name>', 'Environment', 'development')
  .option('-m, --minify', 'Minify output', false)
  .option('-w, --watch', 'Watch mode', false)
  .action((options: BuildOptions) => {
    console.log('Building with options:', options);
  });
```

## Complete Modular Example

```typescript
// commands/build.ts
import { Command } from 'commander';

export const buildCommand = new Command('build')
  .description('Build project')
  .option('-e, --env <name>', 'Environment', 'development')
  .option('-w, --watch', 'Watch mode')
  .action((options) => {
    console.log('Building for:', options.env);
  });

// commands/deploy.ts
import { Command } from 'commander';

export const deployCommand = new Command('deploy')
  .description('Deploy application')
  .argument('<environment>', 'Target environment')
  .option('-b, --branch <name>', 'Git branch', 'main')
  .option('-d, --dry-run', 'Dry run')
  .action((environment, options) => {
    console.log(`Deploying ${options.branch} to ${environment}`);
    if (options.dryRun) {
      console.log('Dry run - no changes made');
    }
  });

// commands/db/migrate.ts
import { Command } from 'commander';

export const migrateCommand = new Command('migrate')
  .description('Run migrations')
  .option('--rollback', 'Rollback last migration')
  .action((options) => {
    if (options.rollback) {
      console.log('Rolling back migration');
    } else {
      console.log('Running migrations');
    }
  });

// commands/db/seed.ts
import { Command } from 'commander';

export const seedCommand = new Command('seed')
  .description('Seed database')
  .argument('[env]', 'Environment', 'development')
  .action((env) => {
    console.log('Seeding database for:', env);
  });

// commands/db/index.ts
import { Command } from 'commander';
import { migrateCommand } from './migrate';
import { seedCommand } from './seed';

export const dbCommand = new Command('db')
  .description('Database operations')
  .addCommand(migrateCommand)
  .addCommand(seedCommand);

// commands/index.ts
export { buildCommand } from './build';
export { deployCommand } from './deploy';
export { dbCommand } from './db';

// index.ts
import { Command } from 'commander';
import { buildCommand, deployCommand, dbCommand } from './commands';

const program = new Command();

program
  .name('project-cli')
  .description('Project management CLI')
  .version('1.0.0')
  .addCommand(buildCommand)
  .addCommand(deployCommand)
  .addCommand(dbCommand);

await program.parseAsync(process.argv);
```

## Directory Structure

```
src/
├── commands/
│   ├── build.ts          # buildCommand export
│   ├── deploy.ts         # deployCommand export
│   ├── db/
│   │   ├── migrate.ts    # migrateCommand export
│   │   ├── seed.ts       # seedCommand export
│   │   └── index.ts      # dbCommand export
│   └── index.ts          # Barrel exports all commands
└── index.ts              # Main program + addCommand()
```

## Best Practices

1. **Export Command instances** from subcommand modules
2. **Use `.addCommand()`** to attach to parent Command
3. **Create barrel exports** in `commands/index.ts`
4. **One command per file** (following single function per module pattern)
5. **Use descriptive command names** that clearly indicate action
6. **Group related commands** in subdirectories
7. **Provide clear descriptions** for all commands
8. **Use TypeScript interfaces** for typed options
9. **Document examples** in help text for complex commands
10. **Validate arguments** in action handlers

## Why Modular Commands?

- **Testability**: Each command can be tested in isolation
- **Maintainability**: Easy to find and modify specific commands
- **Reusability**: Commands can be reused across different CLIs
- **Separation of concerns**: Business logic separated from CLI structure
- **Barrel exports**: Clean import statements in main program
- **Type safety**: Better TypeScript inference with exported types
