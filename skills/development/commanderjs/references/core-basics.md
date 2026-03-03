# Core Basics

Foundational Commander.js concepts and program setup.

## Program Initialization

### Basic Setup

```typescript
import { Command } from 'commander';

const program = new Command();

program
  .name('my-cli')
  .description('Description of your CLI tool')
  .version('1.0.0');
```

### Loading Version from package.json (RECOMMENDED)

**Always load version from package.json to keep it synchronized:**

```typescript
// index.ts or main CLI file
import { Command } from 'commander';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { dirname, join } from 'node:path';

// ESM - Get package.json path
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const packageJson = JSON.parse(
  readFileSync(join(__dirname, '../package.json'), 'utf-8')
);

const program = new Command();

program
  .name('my-cli')
  .description('Description of your CLI tool')
  .version(packageJson.version); // Load from package.json
```

**With Bun (simpler import):**

```typescript
import { Command } from 'commander';
import packageJson from '../package.json' with { type: 'json' };

const program = new Command();

program
  .name('my-cli')
  .description('My CLI tool')
  .version(packageJson.version); // Load from package.json
```

**CommonJS (if not using ESM):**

```typescript
const { Command } = require('commander');
const packageJson = require('../package.json');

const program = new Command();

program
  .name('my-cli')
  .description('My CLI tool')
  .version(packageJson.version);
```

**Benefits:**
- Single source of truth for version
- No manual version updates in multiple files
- Automatic sync with `npm version` commands
- `--version` flag always accurate

## Parsing Arguments

**Always use `parseAsync()` for async action handlers:**

```typescript
// Good - async support
await program.parseAsync(process.argv);

// Avoid - no async support
program.parse(process.argv);
```

## Basic Structure

```typescript
import { Command } from 'commander';

const program = new Command();

program
  .name('my-tool')
  .description('My CLI tool')
  .version('1.0.0')
  .option('-d, --debug', 'Enable debug mode')
  .option('-c, --config <path>', 'Config file path')
  .action((options) => {
    console.log('Options:', options);
  });

await program.parseAsync(process.argv);
```

## Accessing Parsed Options

```typescript
// Inside action handler - options passed as parameter
program
  .option('-p, --port <number>', 'Port number', '3000')
  .action((options) => {
    console.log('Port:', options.port); // Camel-cased
  });

// Outside action handler - use .opts()
await program.parseAsync(process.argv);
const options = program.opts();
console.log('Options:', options);
```

## Help Text

Commander automatically generates help text:

```bash
my-cli --help
```

Customize help:

```typescript
program
  .addHelpText('before', 'Custom text before help')
  .addHelpText('after', 'Custom text after help');
```

## Version Information

```typescript
program.version('1.0.0');
// Enables --version flag
```

Custom version flag:

```typescript
program.version('1.0.0', '-v, --version', 'Display version');
```

## Exit Handling

For testing, override default exit behavior:

```typescript
program.exitOverride();

try {
  await program.parseAsync(['node', 'test', '--invalid']);
} catch (err) {
  // Handle error
}
```

## Environment Variables

Access via `process.env`:

```typescript
program
  .option('-p, --port <number>', 'Port', process.env.PORT || '3000')
  .action((options) => {
    const port = parseInt(options.port);
  });
```

## Usage Pattern

```typescript
program
  .usage('[options] <command>');
```

## Aliases

```typescript
program
  .alias('i'); // npm install -> npm i
```

## Hook Lifecycle

```typescript
program
  .hook('preAction', (thisCommand, actionCommand) => {
    console.log('Before action runs');
  })
  .hook('postAction', (thisCommand, actionCommand) => {
    console.log('After action runs');
  });
```

## Complete Example

```typescript
import { Command } from 'commander';

const program = new Command();

program
  .name('deploy')
  .description('Deploy application to production')
  .version('1.0.0')
  .option('-e, --env <name>', 'Environment', 'production')
  .option('-d, --dry-run', 'Run without making changes')
  .option('-v, --verbose', 'Verbose output')
  .hook('preAction', () => {
    console.log('Starting deployment...');
  })
  .action(async (options) => {
    console.log('Deploying to:', options.env);
    if (options.dryRun) {
      console.log('Dry run mode - no changes made');
      return;
    }
    // Deployment logic
  });

await program.parseAsync(process.argv);
```
