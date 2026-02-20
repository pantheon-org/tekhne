# TypeScript Integration

Complete guide to using Commander.js with TypeScript for type-safe CLI applications.

## Basic Setup

```typescript
import { Command } from 'commander';

const program = new Command();

program
  .name('my-cli')
  .description('TypeScript CLI')
  .version('1.0.0');

await program.parseAsync(process.argv);
```

## Typed Options

```typescript
interface BuildOptions {
  env: string;
  watch: boolean;
  minify: boolean;
  sourcemap: boolean;
}

program
  .command('build')
  .option('-e, --env <name>', 'Environment', 'development')
  .option('-w, --watch', 'Watch mode', false)
  .option('-m, --minify', 'Minify output', false)
  .option('-s, --sourcemap', 'Generate sourcemaps', false)
  .action((options: BuildOptions) => {
    console.log('Environment:', options.env);
    if (options.watch) {
      console.log('Watch mode enabled');
    }
  });
```

## Service Layer Pattern with Typed Options (REQUIRED)

**Always pass typed options objects to downstream services. Never access options piecemeal.**

### Full Stack Architecture

```
types/         → Define option interfaces
services/      → Business logic (receives typed options)
commands/      → Commander wiring (passes typed options to services)
```

### Complete Example

```typescript
// types/deploy-options.ts
export interface DeployOptions {
  env: string;
  branch: string;
  dryRun: boolean;
  force: boolean;
}

// types/index.ts
export type { DeployOptions } from './deploy-options';

// services/deployment-service.ts
import type { DeployOptions } from '../types/deploy-options';

export const deployApplication = (options: DeployOptions): void => {
  // Service receives complete typed options object
  console.log('Deploying:', {
    env: options.env,
    branch: options.branch,
    dryRun: options.dryRun,
    force: options.force,
  });
  
  if (options.dryRun) {
    console.log('[DRY RUN] Would deploy to:', options.env);
    return;
  }
  
  // Actual deployment logic
};

// services/index.ts
export { deployApplication } from './deployment-service';

// commands/deploy.ts
import { Command } from 'commander';
import { deployApplication } from '../services/deployment-service';
import type { DeployOptions } from '../types/deploy-options';

export const deployCommand = new Command('deploy')
  .description('Deploy application')
  .option('-e, --env <name>', 'Environment', 'staging')
  .option('-b, --branch <name>', 'Branch', 'main')
  .option('-d, --dry-run', 'Dry run', false)
  .option('-f, --force', 'Force deployment', false)
  .action((options: DeployOptions) => {
    // Pass complete typed options to service
    deployApplication(options);
  });

// commands/index.ts
export { deployCommand } from './deploy';

// index.ts
import { Command } from 'commander';
import { deployCommand } from './commands';

const program = new Command();
program.addCommand(deployCommand);
await program.parseAsync(process.argv);
```

### Why This Pattern?

**Type Safety:**
- Compile-time validation of option usage
- IDE autocomplete for all options
- Refactoring safety (rename options confidently)

**Explicit Dependencies:**
- Service signature shows exactly what it needs
- No hidden dependencies on commander context
- Easy to test (mock the options object)

**Single Responsibility:**
- Commands handle parsing only
- Services handle business logic only
- Types define the contract between them

### Anti-Pattern: Direct Access

```typescript
// ❌ BAD: Accessing options piecemeal
export const deployCommand = new Command('deploy')
  .option('-e, --env <name>')
  .option('-b, --branch <name>')
  .action((options) => {
    // Passing individual values - loses type safety
    deployApplication(options.env, options.branch);
  });

// ❌ BAD: Untyped service signature
const deployApplication = (env: any, branch: any) => {
  // What other options are available? Unknown!
};
```

### Correct Pattern: Typed Options Object

```typescript
// ✅ GOOD: Pass complete typed options
export const deployCommand = new Command('deploy')
  .option('-e, --env <name>', 'Environment')
  .option('-b, --branch <name>', 'Branch')
  .option('-d, --dry-run', 'Dry run', false)
  .action((options: DeployOptions) => {
    // Pass complete object with all options
    deployApplication(options);
  });

// ✅ GOOD: Typed service signature
const deployApplication = (options: DeployOptions): void => {
  // All available options are explicit and typed
};
```

## Type-Safe Option Parsing (Legacy Pattern - Use Service Layer Instead)

```typescript
import { Command, OptionValues } from 'commander';

interface DeployOptions extends OptionValues {
  env: string;
  branch: string;
  dryRun: boolean;
  force: boolean;
}

const parseOptions = (options: OptionValues): DeployOptions => {
  return {
    env: options.env as string,
    branch: options.branch as string,
    dryRun: options.dryRun as boolean,
    force: options.force as boolean,
  };
};

program
  .command('deploy')
  .option('-e, --env <name>', 'Environment')
  .option('-b, --branch <name>', 'Branch', 'main')
  .option('-d, --dry-run', 'Dry run')
  .option('-f, --force', 'Force')
  .action((opts: OptionValues) => {
    const options = parseOptions(opts);
    deploy(options);
  });
```

## Custom Option Types

```typescript
interface ServerOptions {
  port: number;
  host: string;
  ssl: boolean;
}

program
  .command('serve')
  .option('-p, --port <number>', 'Port', (value) => {
    const port = parseInt(value, 10);
    if (isNaN(port)) {
      throw new Error('Port must be a number');
    }
    return port;
  }, 3000)
  .option('-h, --host <address>', 'Host', 'localhost')
  .option('--ssl', 'Enable SSL', false)
  .action((options: ServerOptions) => {
    console.log('Starting server:', options);
  });
```

## Typed Arguments

```typescript
program
  .command('greet')
  .argument('<name>', 'Person to greet')
  .argument('[greeting]', 'Greeting message', 'Hello')
  .action((name: string, greeting: string) => {
    console.log(`${greeting}, ${name}!`);
  });
```

## Enum Types with Choices

```typescript
type LogLevel = 'debug' | 'info' | 'warn' | 'error';

interface LogOptions {
  level: LogLevel;
  format: 'text' | 'json';
}

import { Option } from 'commander';

program
  .command('log')
  .addOption(
    new Option('-l, --level <level>', 'Log level')
      .choices(['debug', 'info', 'warn', 'error'] as const)
      .default('info')
  )
  .addOption(
    new Option('-f, --format <format>', 'Output format')
      .choices(['text', 'json'] as const)
      .default('text')
  )
  .action((options: LogOptions) => {
    console.log('Logging at level:', options.level);
  });
```

## Type Guards

```typescript
interface Options extends OptionValues {
  config?: string;
  env: string;
}

const hasConfig = (options: Options): options is Options & { config: string } => {
  return options.config !== undefined;
};

program
  .option('-c, --config [path]', 'Config file')
  .option('-e, --env <name>', 'Environment', 'development')
  .action((opts: OptionValues) => {
    const options = opts as Options;
    
    if (hasConfig(options)) {
      console.log('Using config:', options.config);
    }
    
    console.log('Environment:', options.env);
  });
```

## Generic Command Builder

```typescript
interface CommandConfig<T> {
  name: string;
  description: string;
  options: Record<string, any>;
  action: (options: T) => Promise<void> | void;
}

const createTypedCommand = <T>(config: CommandConfig<T>): Command => {
  const cmd = new Command(config.name);
  cmd.description(config.description);
  
  for (const [key, value] of Object.entries(config.options)) {
    cmd.option(key, value);
  }
  
  cmd.action(config.action);
  return cmd;
};

// Usage
interface BuildOptions {
  env: string;
  minify: boolean;
}

const buildCommand = createTypedCommand<BuildOptions>({
  name: 'build',
  description: 'Build project',
  options: {
    '-e, --env <name>': 'Environment',
    '-m, --minify': 'Minify output',
  },
  action: async (options) => {
    console.log('Building:', options);
  },
});

program.addCommand(buildCommand);
```

## Zod Validation

```typescript
import { z } from 'zod';

const BuildOptionsSchema = z.object({
  env: z.enum(['development', 'staging', 'production']),
  port: z.number().int().min(1).max(65535),
  watch: z.boolean(),
  verbose: z.boolean().optional(),
});

type BuildOptions = z.infer<typeof BuildOptionsSchema>;

program
  .command('build')
  .option('-e, --env <name>', 'Environment')
  .option('-p, --port <number>', 'Port', parseInt)
  .option('-w, --watch', 'Watch mode')
  .option('-v, --verbose', 'Verbose output')
  .action((opts: unknown) => {
    try {
      const options = BuildOptionsSchema.parse(opts);
      console.log('Valid options:', options);
    } catch (error) {
      if (error instanceof z.ZodError) {
        console.error('Invalid options:');
        error.errors.forEach(err => {
          console.error(`  ${err.path.join('.')}: ${err.message}`);
        });
        process.exit(1);
      }
      throw error;
    }
  });
```

## Modular Commands with Types

```typescript
// types/options.ts
export interface BuildOptions {
  env: string;
  minify: boolean;
  sourcemap: boolean;
}

export interface DeployOptions {
  environment: string;
  branch: string;
  dryRun: boolean;
}

// commands/build.ts
import { Command } from 'commander';
import type { BuildOptions } from '../types/options';

export const createBuildCommand = (): Command => {
  return new Command('build')
    .description('Build project')
    .option('-e, --env <name>', 'Environment', 'development')
    .option('-m, --minify', 'Minify output', false)
    .option('-s, --sourcemap', 'Generate sourcemaps', false)
    .action(async (options: BuildOptions) => {
      console.log('Building with options:', options);
    });
};

// commands/deploy.ts
import { Command } from 'commander';
import type { DeployOptions } from '../types/options';

export const createDeployCommand = (): Command => {
  return new Command('deploy')
    .description('Deploy application')
    .argument('<environment>', 'Target environment')
    .option('-b, --branch <name>', 'Branch', 'main')
    .option('-d, --dry-run', 'Dry run', false)
    .action(async (environment: string, options: Omit<DeployOptions, 'environment'>) => {
      const fullOptions: DeployOptions = { environment, ...options };
      console.log('Deploying:', fullOptions);
    });
};

// index.ts
import { Command } from 'commander';
import { createBuildCommand } from './commands/build';
import { createDeployCommand } from './commands/deploy';
import packageJson from '../package.json' with { type: 'json' };

const program = new Command();

program
  .name('my-cli')
  .version(packageJson.version); // Load version from package.json

program.addCommand(createBuildCommand());
program.addCommand(createDeployCommand());

await program.parseAsync(process.argv);
```

## Type-Safe Config Loading

```typescript
import { z } from 'zod';
import { promises as fs } from 'fs';

const ConfigSchema = z.object({
  env: z.string(),
  port: z.number(),
  database: z.object({
    host: z.string(),
    port: z.number(),
    name: z.string(),
  }),
});

type Config = z.infer<typeof ConfigSchema>;

const loadConfig = async (path: string): Promise<Config> => {
  const content = await fs.readFile(path, 'utf-8');
  const data = JSON.parse(content);
  return ConfigSchema.parse(data);
};

program
  .command('start')
  .option('-c, --config <path>', 'Config file', 'config.json')
  .action(async (options: { config: string }) => {
    try {
      const config = await loadConfig(options.config);
      console.log('Starting with config:', config);
    } catch (error) {
      console.error('Failed to load config:', error.message);
      process.exit(1);
    }
  });
```

## Complete TypeScript Example

```typescript
import { Command, Option, OptionValues } from 'commander';
import { z } from 'zod';

// Schema definition
const DeployOptionsSchema = z.object({
  environment: z.enum(['dev', 'staging', 'production']),
  branch: z.string(),
  dryRun: z.boolean(),
  force: z.boolean(),
  verbose: z.boolean().optional(),
});

type DeployOptions = z.infer<typeof DeployOptionsSchema>;

// Command builder
const createDeployCommand = (): Command => {
  return new Command('deploy')
    .description('Deploy application')
    .argument('<environment>', 'Target environment')
    .option('-b, --branch <name>', 'Branch to deploy', 'main')
    .option('-d, --dry-run', 'Simulate deployment', false)
    .option('-f, --force', 'Force deployment', false)
    .option('-v, --verbose', 'Verbose output', false)
    .action(async (environment: string, opts: OptionValues) => {
      try {
        const options = DeployOptionsSchema.parse({
          environment,
          ...opts,
        });
        
        await deploy(options);
      } catch (error) {
        if (error instanceof z.ZodError) {
          console.error('Invalid options:');
          error.errors.forEach(err => {
            console.error(`  ${err.path.join('.')}: ${err.message}`);
          });
          process.exit(1);
        }
        throw error;
      }
    });
};

// Business logic
const deploy = async (options: DeployOptions): Promise<void> => {
  if (options.verbose) {
    console.log('Deploy options:', options);
  }
  
  if (options.dryRun) {
    console.log('Dry run - simulating deployment');
    return;
  }
  
  console.log(`Deploying ${options.branch} to ${options.environment}`);
  
  // Deployment logic
};

// Program setup
const program = new Command();

program
  .name('deploy-cli')
  .description('Deployment CLI tool')
  .version('1.0.0');

program.addCommand(createDeployCommand());

await program.parseAsync(process.argv);
```

## Best Practices

1. **Define interfaces** for all option types
2. **Use Zod** for runtime validation
3. **Create type guards** for conditional logic
4. **Use enums** for fixed choices
5. **Modularize commands** with typed exports
6. **Validate early** in action handlers
7. **Use const assertions** for choices arrays
8. **Type action parameters** explicitly
9. **Use generics** for reusable command builders
10. **Separate types** into dedicated files
