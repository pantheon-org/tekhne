# Action Handlers

Complete guide to implementing action handlers with async/await, error handling, and validation.

## Basic Action Handler

```typescript
program
  .command('build')
  .action(() => {
    console.log('Building...');
  });
```

## Async Action Handlers

**Always use `parseAsync()` with async handlers:**

```typescript
program
  .command('deploy')
  .action(async (options) => {
    console.log('Deploying...');
    await performDeployment();
    console.log('Deployment complete');
  });

// Must use parseAsync()
await program.parseAsync(process.argv);
```

## Action Handler Parameters

### Options Only

```typescript
program
  .option('-p, --port <number>', 'Port')
  .action((options) => {
    console.log('Port:', options.port);
  });
```

### Arguments Only

```typescript
program
  .command('greet')
  .argument('<name>', 'Name')
  .action((name) => {
    console.log('Hello,', name);
  });
```

### Arguments and Options

```typescript
program
  .command('deploy')
  .argument('<environment>', 'Environment')
  .option('-b, --branch <name>', 'Branch')
  .action((environment, options) => {
    console.log('Deploying', options.branch, 'to', environment);
  });
```

### Multiple Arguments and Options

```typescript
program
  .command('copy')
  .argument('<source>', 'Source')
  .argument('<dest>', 'Destination')
  .option('-f, --force', 'Force overwrite')
  .action((source, dest, options) => {
    console.log(`Copying ${source} to ${dest}`);
    if (options.force) {
      console.log('Force mode enabled');
    }
  });
```

### Access to Command Object

```typescript
program
  .command('info')
  .action((options, command) => {
    console.log('Command name:', command.name());
    console.log('Options:', options);
  });
```

## Error Handling

### Try-Catch

```typescript
program
  .command('deploy')
  .action(async (options) => {
    try {
      await performDeployment(options);
      console.log('Deployment successful');
    } catch (error) {
      console.error('Deployment failed:', error.message);
      process.exit(1);
    }
  });
```

### Process Exit Codes

```typescript
program
  .command('test')
  .action(async () => {
    try {
      const results = await runTests();
      if (results.failed > 0) {
        console.error(`${results.failed} tests failed`);
        process.exit(1); // Exit with error code
      }
      console.log('All tests passed');
      process.exit(0); // Exit successfully
    } catch (error) {
      console.error('Test execution failed:', error.message);
      process.exit(2); // Different error code
    }
  });
```

### Custom Error Handler

```typescript
program
  .configureOutput({
    outputError: (str, write) => {
      write(`ERROR: ${str}`);
    }
  })
  .exitOverride((err) => {
    if (err.code === 'commander.unknownOption') {
      console.error('Unknown option provided');
      process.exit(1);
    }
    throw err;
  });
```

## Validation

### Validate Required Arguments

```typescript
program
  .command('connect')
  .argument('<url>', 'Server URL')
  .action((url) => {
    try {
      new URL(url); // Validate URL format
    } catch {
      console.error('Invalid URL format');
      process.exit(1);
    }
    console.log('Connecting to:', url);
  });
```

### Validate Option Values

```typescript
program
  .command('serve')
  .option('-p, --port <number>', 'Port', (value) => {
    const port = parseInt(value, 10);
    if (isNaN(port) || port < 1 || port > 65535) {
      throw new Error('Port must be between 1 and 65535');
    }
    return port;
  })
  .action((options) => {
    console.log('Starting server on port:', options.port);
  });
```

### Validate Option Combinations

```typescript
program
  .command('build')
  .option('-w, --watch', 'Watch mode')
  .option('-p, --production', 'Production build')
  .action((options) => {
    if (options.watch && options.production) {
      console.error('Cannot use --watch with --production');
      process.exit(1);
    }
    console.log('Building...');
  });
```

## Progress and Feedback

```typescript
program
  .command('install')
  .action(async (options) => {
    console.log('Installing dependencies...');
    
    const packages = ['pkg1', 'pkg2', 'pkg3'];
    for (let i = 0; i < packages.length; i++) {
      console.log(`[${i + 1}/${packages.length}] Installing ${packages[i]}`);
      await installPackage(packages[i]);
    }
    
    console.log('Installation complete!');
  });
```

## State Management

Avoid global state, use closures or pass state:

```typescript
const createDeployCommand = (config: Config) => {
  return new Command('deploy')
    .action(async (options) => {
      // Use config passed via closure
      await deploy(config, options);
    });
};

const config = loadConfig();
program.addCommand(createDeployCommand(config));
```

## Conditional Logic

```typescript
program
  .command('deploy')
  .argument('<environment>', 'Environment')
  .option('-f, --force', 'Force deployment')
  .option('-d, --dry-run', 'Dry run')
  .action(async (environment, options) => {
    if (options.dryRun) {
      console.log('Dry run mode - simulating deployment');
      return simulateDeployment(environment);
    }
    
    if (!options.force) {
      const confirmed = await promptConfirm(
        `Deploy to ${environment}?`
      );
      if (!confirmed) {
        console.log('Deployment cancelled');
        return;
      }
    }
    
    await deploy(environment);
  });
```

## Parallel Operations

```typescript
program
  .command('build-all')
  .action(async () => {
    console.log('Building all targets...');
    
    const builds = [
      buildClient(),
      buildServer(),
      buildWorker(),
    ];
    
    const results = await Promise.allSettled(builds);
    
    const failed = results.filter(r => r.status === 'rejected');
    if (failed.length > 0) {
      console.error(`${failed.length} builds failed`);
      process.exit(1);
    }
    
    console.log('All builds complete');
  });
```

## Complete Example

```typescript
import { Command } from 'commander';
import { promises as fs } from 'fs';

const program = new Command();

program
  .command('process')
  .description('Process files')
  .argument('<input>', 'Input file')
  .argument('[output]', 'Output file')
  .option('-f, --format <type>', 'Output format', 'json')
  .option('-m, --minify', 'Minify output')
  .option('-v, --verbose', 'Verbose logging')
  .action(async (input, output, options) => {
    try {
      // Validation
      if (options.verbose) {
        console.log('Validating inputs...');
      }
      
      const validFormats = ['json', 'yaml', 'xml'];
      if (!validFormats.includes(options.format)) {
        throw new Error(
          `Format must be one of: ${validFormats.join(', ')}`
        );
      }
      
      // Check input exists
      try {
        await fs.access(input);
      } catch {
        throw new Error(`Input file not found: ${input}`);
      }
      
      // Process
      if (options.verbose) {
        console.log(`Reading ${input}...`);
      }
      
      const data = await fs.readFile(input, 'utf-8');
      
      if (options.verbose) {
        console.log('Processing data...');
      }
      
      let processed = processData(data, options.format);
      
      if (options.minify) {
        if (options.verbose) {
          console.log('Minifying...');
        }
        processed = minify(processed);
      }
      
      // Output
      const outputPath = output || `${input}.${options.format}`;
      
      if (options.verbose) {
        console.log(`Writing to ${outputPath}...`);
      }
      
      await fs.writeFile(outputPath, processed);
      
      console.log(`Success: ${outputPath}`);
      
    } catch (error) {
      console.error('Error:', error.message);
      if (options.verbose) {
        console.error(error.stack);
      }
      process.exit(1);
    }
  });

await program.parseAsync(process.argv);

// Helper functions
const processData = (data: string, format: string): string => {
  // Implementation
  return data;
};

const minify = (data: string): string => {
  // Implementation
  return data;
};
```

## Best Practices

1. **Always use async/await** with `parseAsync()`
2. **Validate inputs early** in action handler
3. **Use try-catch** for error handling
4. **Provide meaningful error messages** with context
5. **Use appropriate exit codes** (0 = success, non-zero = error)
6. **Avoid global state** - use closures or parameters
7. **Give user feedback** during long operations
8. **Handle edge cases** explicitly
9. **Log verbosely when requested** with `-v` flag
10. **Fail fast** - exit on validation errors
