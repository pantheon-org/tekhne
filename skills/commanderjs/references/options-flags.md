# Options & Flags

Complete guide to defining and handling options in Commander.js.

## Basic Options

```typescript
program
  .option('-d, --debug', 'Enable debug mode')
  .option('-v, --verbose', 'Verbose output');
```

Options are accessed as camelCase:

```typescript
.action((options) => {
  if (options.debug) { /* ... */ }
  if (options.verbose) { /* ... */ }
});
```

## Options with Values

### Required Value

```typescript
program
  .option('-p, --port <number>', 'Port number')
  .option('-o, --output <path>', 'Output file');

// Usage: my-cli --port 3000 --output dist/
```

### Optional Value

```typescript
program
  .option('-c, --config [path]', 'Config file (optional)');

// Usage: my-cli --config
// or:    my-cli --config custom.json
```

## Default Values

```typescript
program
  .option('-p, --port <number>', 'Port', '3000')
  .option('-h, --host <address>', 'Host', 'localhost');
```

## Option Types

### String (default)

```typescript
program
  .option('-n, --name <value>', 'Name');
```

### Number

```typescript
import { Option } from 'commander';

program
  .addOption(
    new Option('-p, --port <number>', 'Port')
      .argParser(parseInt)
  );
```

Or with validation:

```typescript
program
  .option('-p, --port <number>', 'Port', (value) => {
    const port = parseInt(value, 10);
    if (isNaN(port) || port < 0 || port > 65535) {
      throw new Error('Invalid port number');
    }
    return port;
  });
```

### Boolean

```typescript
program
  .option('-f, --force', 'Force overwrite')
  .option('--no-color', 'Disable colors'); // Negative boolean
```

### Multiple Values (Variadic)

```typescript
program
  .option('-i, --include <paths...>', 'Files to include');

// Usage: my-cli --include file1.js file2.js file3.js
```

### Repeatable Options

```typescript
program
  .option('-v, --verbose', 'Verbose (repeatable)', (_, prev) => prev + 1, 0);

// Usage: my-cli -vvv
// Result: options.verbose === 3
```

Or collecting values:

```typescript
program
  .option('-e, --exclude <pattern>', 'Exclude pattern', 
    (value, prev) => prev.concat([value]), []);

// Usage: my-cli -e "*.test.js" -e "*.spec.js"
// Result: options.exclude === ['*.test.js', '*.spec.js']
```

## Required Options

```typescript
program
  .requiredOption('-c, --config <path>', 'Config file (required)');

// Throws error if not provided
```

## Option Conflicts

```typescript
program
  .option('-b, --build', 'Build project')
  .option('-w, --watch', 'Watch mode')
  .conflictsWith('watch'); // --build and --watch cannot be used together
```

## Option Implies

```typescript
program
  .option('--production', 'Production mode')
  .option('--minify', 'Minify output')
  .addOption(
    new Option('--production')
      .implies({ minify: true })
  );
```

## Environment Variables as Defaults

```typescript
program
  .option('-p, --port <number>', 'Port', process.env.PORT || '3000')
  .option('--api-key <key>', 'API Key', process.env.API_KEY);
```

## Custom Option Processing

```typescript
program
  .option('-d, --date <date>', 'Date', (value) => {
    const date = new Date(value);
    if (isNaN(date.getTime())) {
      throw new Error('Invalid date format');
    }
    return date;
  });
```

## Choices (Enum Values)

```typescript
import { Option } from 'commander';

program
  .addOption(
    new Option('-l, --log-level <level>', 'Log level')
      .choices(['debug', 'info', 'warn', 'error'])
      .default('info')
  );
```

## Hidden Options

```typescript
program
  .addOption(
    new Option('--internal', 'Internal use only')
      .hideHelp()
  );
```

## Option Presets

```typescript
import { Option } from 'commander';

program
  .addOption(
    new Option('-e, --env <name>', 'Environment')
      .choices(['dev', 'staging', 'production'])
      .default('dev')
      .env('NODE_ENV')
  );
```

## Complete Example

```typescript
import { Command, Option } from 'commander';

const program = new Command();

program
  .name('build')
  .description('Build project')
  .requiredOption('-i, --input <path>', 'Input directory')
  .option('-o, --output <path>', 'Output directory', 'dist')
  .option('-f, --format <type>', 'Output format', (value) => {
    const valid = ['esm', 'cjs', 'umd'];
    if (!valid.includes(value)) {
      throw new Error(`Format must be one of: ${valid.join(', ')}`);
    }
    return value;
  }, 'esm')
  .option('-m, --minify', 'Minify output')
  .option('-s, --sourcemap', 'Generate sourcemaps')
  .option('-w, --watch', 'Watch mode')
  .addOption(
    new Option('-l, --log-level <level>', 'Log level')
      .choices(['debug', 'info', 'warn', 'error'])
      .default('info')
  )
  .option('-e, --exclude <patterns...>', 'Exclude patterns')
  .action(async (options) => {
    console.log('Building with options:', options);
    
    // Validate combinations
    if (options.watch && options.minify) {
      console.warn('Warning: minify is slow in watch mode');
    }
    
    // Implementation
  });

await program.parseAsync(process.argv);
```

## Best Practices

1. **Use kebab-case** for option names: `--output-dir` not `--outputDir`
2. **Provide defaults** for optional options when sensible
3. **Validate option values** in custom parsers
4. **Use `requiredOption`** instead of validating in action handlers
5. **Use `choices()`** for enum values
6. **Use descriptive names** that clearly indicate purpose
7. **Group related options** together in code
8. **Document expected formats** in descriptions
