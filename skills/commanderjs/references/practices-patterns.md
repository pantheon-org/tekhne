# Best Practices & Patterns

Common patterns, best practices, and anti-patterns for Commander.js applications.

## Typed Options Pattern (REQUIRED)

### Rule: Always Pass Complete Typed Options to Services

Commands should pass the entire options object (typed) to downstream services, never individual properties.

```typescript
// ✅ GOOD: Pass complete typed options object
// types/build-options.ts
export interface BuildOptions {
  outDir: string;
  minify: boolean;
  sourcemap: boolean;
  watch: boolean;
}

// services/build-service.ts
import type { BuildOptions } from '../types/build-options';

export const buildProject = (options: BuildOptions): void => {
  console.log('Building with options:', options);
  
  if (options.watch) {
    console.log('Starting watch mode...');
  }
  
  // All options available with type safety
};

// commands/build.ts
import { Command } from 'commander';
import { buildProject } from '../services/build-service';
import type { BuildOptions } from '../types/build-options';

export const buildCommand = new Command('build')
  .option('-o, --out-dir <path>', 'Output directory', 'dist')
  .option('-m, --minify', 'Minify output', false)
  .option('-s, --sourcemap', 'Generate sourcemaps', false)
  .option('-w, --watch', 'Watch mode', false)
  .action((options: BuildOptions) => {
    // Pass complete typed object
    buildProject(options);
  });
```

### Anti-Pattern: Passing Individual Properties

```typescript
// ❌ BAD: Passing individual options piecemeal
export const buildCommand = new Command('build')
  .option('-o, --out-dir <path>', 'Output directory', 'dist')
  .option('-m, --minify', 'Minify output', false)
  .action((options) => {
    // Loses type safety, hard to refactor
    buildProject(options.outDir, options.minify);
  });

// ❌ BAD: Service doesn't know what options are available
const buildProject = (outDir: string, minify: boolean) => {
  // What about sourcemap? watch? Hard to discover
};
```

### Benefits

**Type Safety:**
- Compile-time validation
- IDE autocomplete for all options
- Refactoring safety

**Explicit Dependencies:**
- Service signature documents what it needs
- No hidden dependencies
- Easy to test (mock the options object)

**Maintainability:**
- Add new options without changing service signatures
- Options interface is the single source of truth

### Directory Structure for Typed Options

```
src/
├── types/
│   ├── build-options.ts
│   ├── deploy-options.ts
│   └── index.ts              # Barrel export types
├── services/
│   ├── build-service.ts       # Receives BuildOptions
│   ├── deploy-service.ts      # Receives DeployOptions
│   └── index.ts
├── commands/
│   ├── build.ts               # Passes BuildOptions
│   ├── deploy.ts              # Passes DeployOptions
│   └── index.ts
└── index.ts
```

### Testing with Typed Options

```typescript
// build-service.test.ts
import { buildProject } from './build-service';
import type { BuildOptions } from '../types/build-options';

describe('buildProject', () => {
  it('should build with minification', () => {
    const options: BuildOptions = {
      outDir: 'dist',
      minify: true,
      sourcemap: false,
      watch: false,
    };
    
    // Easy to test - just pass typed object
    buildProject(options);
    
    // Assert expectations
  });
});
```

## CLI Design Principles

### 1. Clear Command Structure

```typescript
// Good - Clear hierarchy
program
  .command('db migrate')
  .command('db seed')
  .command('db reset')

// Avoid - Ambiguous naming
program
  .command('run-migration')
  .command('do-seed')
  .command('database-clear')
```

### 2. Consistent Naming

```typescript
// Good - Kebab-case for flags
program
  .option('--output-dir <path>', 'Output directory')
  .option('--dry-run', 'Dry run mode')

// Avoid - Mixed casing
program
  .option('--outputDir <path>', 'Output directory')
  .option('--dryRun', 'Dry run mode')
```

### 3. Sensible Defaults

```typescript
// Good - Provide defaults
program
  .option('-p, --port <number>', 'Port', '3000')
  .option('-h, --host <address>', 'Host', 'localhost')
  .option('--log-level <level>', 'Log level', 'info')

// Avoid - No defaults for optional options
program
  .option('-p, --port <number>', 'Port')
  .option('-h, --host <address>', 'Host')
```

## Error Handling Patterns

### Validation Errors

```typescript
program
  .command('deploy')
  .argument('<env>', 'Environment')
  .action((env) => {
    const validEnvs = ['dev', 'staging', 'production'];
    if (!validEnvs.includes(env)) {
      console.error(
        `Invalid environment: ${env}\n` +
        `Valid values: ${validEnvs.join(', ')}`
      );
      process.exit(1);
    }
    
    deploy(env);
  });
```

### Operation Errors

```typescript
program
  .command('build')
  .action(async () => {
    try {
      await build();
      console.log('Build successful');
    } catch (error) {
      console.error('Build failed:', error.message);
      if (process.env.DEBUG) {
        console.error(error.stack);
      }
      process.exit(1);
    }
  });
```

### User-Friendly Messages

```typescript
program
  .command('connect')
  .argument('<url>', 'Server URL')
  .action(async (url) => {
    try {
      await connect(url);
    } catch (error) {
      if (error.code === 'ENOTFOUND') {
        console.error(
          `Unable to connect to ${url}\n` +
          'Please check:\n' +
          '  - The URL is correct\n' +
          '  - You have internet connectivity\n' +
          '  - The server is running'
        );
      } else {
        console.error('Connection failed:', error.message);
      }
      process.exit(1);
    }
  });
```

## Configuration Patterns

### Config File Loading

```typescript
import { promises as fs } from 'fs';

const loadConfig = async (path: string) => {
  try {
    const content = await fs.readFile(path, 'utf-8');
    return JSON.parse(content);
  } catch (error) {
    if (error.code === 'ENOENT') {
      console.error(`Config file not found: ${path}`);
    } else {
      console.error(`Failed to load config: ${error.message}`);
    }
    process.exit(1);
  }
};

program
  .option('-c, --config <path>', 'Config file', 'config.json')
  .action(async (options) => {
    const config = await loadConfig(options.config);
    console.log('Loaded config:', config);
  });
```

### Environment Variables

```typescript
program
  .option(
    '-p, --port <number>',
    'Port',
    process.env.PORT || '3000'
  )
  .option(
    '--api-key <key>',
    'API Key',
    process.env.API_KEY
  )
  .action((options) => {
    if (!options.apiKey) {
      console.error(
        'API key required\n' +
        'Provide via --api-key or API_KEY env var'
      );
      process.exit(1);
    }
  });
```

## Testing Patterns

### Setup for Testing

```typescript
import { Command } from 'commander';

export const createProgram = (): Command => {
  const program = new Command();
  
  program
    .name('my-cli')
    .version('1.0.0')
    .exitOverride(); // Throw instead of exit
  
  return program;
};
```

### Test Examples

```typescript
import { describe, it, expect } from 'bun:test';
import { createProgram } from './cli';

describe('CLI', () => {
  it('should parse options correctly', async () => {
    const program = createProgram();
    
    program
      .option('-p, --port <number>', 'Port')
      .action(() => {});
    
    await program.parseAsync(['node', 'test', '--port', '8080']);
    
    expect(program.opts().port).toBe('8080');
  });
  
  it('should handle missing required options', async () => {
    const program = createProgram();
    
    program
      .requiredOption('-c, --config <path>', 'Config')
      .action(() => {});
    
    await expect(
      program.parseAsync(['node', 'test'])
    ).rejects.toThrow();
  });
});
```

## Interactive Patterns

### Confirmation Prompts

```typescript
import * as readline from 'readline';

const confirm = (question: string): Promise<boolean> => {
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
  });
  
  return new Promise((resolve) => {
    rl.question(`${question} (y/n): `, (answer) => {
      rl.close();
      resolve(answer.toLowerCase() === 'y');
    });
  });
};

program
  .command('delete')
  .argument('<file>', 'File to delete')
  .option('-f, --force', 'Skip confirmation')
  .action(async (file, options) => {
    if (!options.force) {
      const confirmed = await confirm(
        `Delete ${file}?`
      );
      if (!confirmed) {
        console.log('Cancelled');
        return;
      }
    }
    
    await deleteFile(file);
  });
```

## Progress Indication

```typescript
program
  .command('download')
  .argument('<url>', 'URL to download')
  .action(async (url) => {
    console.log('Downloading...');
    
    const total = await getContentLength(url);
    let downloaded = 0;
    
    const stream = await download(url);
    stream.on('data', (chunk) => {
      downloaded += chunk.length;
      const percent = ((downloaded / total) * 100).toFixed(2);
      process.stdout.write(`\rProgress: ${percent}%`);
    });
    
    await new Promise((resolve) => stream.on('end', resolve));
    console.log('\nDownload complete');
  });
```

## Logging Patterns

### Verbosity Levels

```typescript
const createLogger = (verbose: boolean) => ({
  debug: (msg: string) => {
    if (verbose) console.log('[DEBUG]', msg);
  },
  info: (msg: string) => {
    console.log('[INFO]', msg);
  },
  error: (msg: string) => {
    console.error('[ERROR]', msg);
  },
});

program
  .option('-v, --verbose', 'Verbose output')
  .action((options) => {
    const log = createLogger(options.verbose);
    
    log.debug('Starting process');
    log.info('Processing...');
    log.debug('Step 1 complete');
  });
```

## Performance Patterns

### Lazy Loading Commands

```typescript
// commands/build.ts
export const createBuildCommand = async () => {
  const { build } = await import('./lib/build');
  
  return new Command('build')
    .action(async () => {
      await build();
    });
};

// index.ts
program
  .command('build')
  .action(async () => {
    const buildCmd = await createBuildCommand();
    await buildCmd.parseAsync(process.argv);
  });
```

## Version Management

### ✅ Load Version from package.json

**Always load version from package.json to maintain single source of truth:**

```typescript
// index.ts
import { Command } from 'commander';
import packageJson from '../package.json' with { type: 'json' };

const program = new Command();

program
  .name('my-cli')
  .version(packageJson.version); // Single source of truth
```

**Benefits:**
- Automatic sync with `npm version` commands
- No manual version updates in code
- Consistent version across all commands
- `--version` flag always accurate

**Node.js ESM (without import assertions):**

```typescript
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { dirname, join } from 'node:path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const packageJson = JSON.parse(
  readFileSync(join(__dirname, '../package.json'), 'utf-8')
);

program.version(packageJson.version);
```

### ❌ Hardcoded Version

```typescript
// Avoid - requires manual updates
program.version('1.2.3');

// Problem: When you run `npm version patch`, code is out of sync
```

## Anti-Patterns

### ❌ Global State

```typescript
// Avoid
let globalConfig = {};

program
  .command('build')
  .action(() => {
    globalConfig = loadConfig();
  });
```

### ❌ Synchronous parse() with Async Actions

```typescript
// Avoid
program
  .action(async () => {
    await doSomething();
  });

program.parse(process.argv); // Should be parseAsync()
```

### ❌ No Error Handling

```typescript
// Avoid
program
  .command('deploy')
  .action(async () => {
    await deploy(); // What if this throws?
  });
```

### ❌ Unclear Error Messages

```typescript
// Avoid
if (!valid) {
  console.error('Error');
  process.exit(1);
}

// Prefer
if (!valid) {
  console.error(
    'Invalid configuration\n' +
    'Expected format: { name: string, version: string }'
  );
  process.exit(1);
}
```

## Complete Example

```typescript
import { Command } from 'commander';
import { promises as fs } from 'fs';

const program = new Command();

program
  .name('file-processor')
  .description('Process files with various operations')
  .version('1.0.0');

program
  .command('convert')
  .description('Convert file format')
  .argument('<input>', 'Input file')
  .argument('[output]', 'Output file')
  .option('-f, --format <type>', 'Output format', 'json')
  .option('-m, --minify', 'Minify output')
  .option('-v, --verbose', 'Verbose output')
  .option('--dry-run', 'Simulate without writing')
  .action(async (input, output, options) => {
    const log = createLogger(options.verbose);
    
    try {
      // Validation
      log.debug('Validating inputs');
      
      const validFormats = ['json', 'yaml', 'xml'];
      if (!validFormats.includes(options.format)) {
        throw new Error(
          `Invalid format: ${options.format}\n` +
          `Valid formats: ${validFormats.join(', ')}`
        );
      }
      
      // Check input file
      try {
        await fs.access(input);
      } catch {
        throw new Error(`Input file not found: ${input}`);
      }
      
      // Process
      log.info(`Reading ${input}`);
      const data = await fs.readFile(input, 'utf-8');
      
      log.debug('Converting format');
      let converted = convert(data, options.format);
      
      if (options.minify) {
        log.debug('Minifying');
        converted = minify(converted);
      }
      
      // Output
      const outputPath = output || `${input}.${options.format}`;
      
      if (options.dryRun) {
        log.info('Dry run - no files written');
        console.log(converted);
        return;
      }
      
      log.info(`Writing to ${outputPath}`);
      await fs.writeFile(outputPath, converted);
      
      log.info('Conversion complete');
      
    } catch (error) {
      console.error('Error:', error.message);
      if (options.verbose) {
        console.error(error.stack);
      }
      process.exit(1);
    }
  });

await program.parseAsync(process.argv);

// Helpers
const createLogger = (verbose: boolean) => ({
  debug: (msg: string) => {
    if (verbose) console.log('[DEBUG]', msg);
  },
  info: (msg: string) => {
    console.log('[INFO]', msg);
  },
});

const convert = (data: string, format: string): string => {
  // Implementation
  return data;
};

const minify = (data: string): string => {
  // Implementation
  return data;
};
```
