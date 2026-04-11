# Scenario 03: Debug and Fix a Broken CLI Tool

## User Prompt

A developer has built a CLI tool for managing cloud storage buckets, but they're reporting that two of the commands behave strangely: the `upload` command always uses the wrong region regardless of what the user passes, and the `list` command ignores its `--prefix` option.

After looking at the code, a senior engineer suspects the problem is with how options are being read inside the subcommand handlers.

The following is the broken CLI code. Identify the bugs, fix them, and write a short explanation of what was wrong.

## Output Specification

Create a file called `cli-fixed.ts` with the corrected code and a file called `explanation.md` explaining what was wrong and why.

## Input Files

The following file contains the broken implementation. Extract it before starting.

=============== FILE: inputs/cli-broken.ts ===============
```typescript
import { Command } from 'commander';

const program = new Command()
  .name('bucket-tool')
  .description('Cloud storage bucket manager')
  .version('1.0.0')
  .option('--region <name>', 'Cloud region', 'us-east-1');

const uploadCommand = new Command('upload')
  .description('Upload a file to a bucket')
  .argument('<bucket>', 'Target bucket name')
  .argument('<file>', 'File to upload')
  .option('--region <name>', 'Override region for this operation')
  .option('--acl <policy>', 'Access control policy', 'private')
  .action(async (bucket, file) => {
    // Read region from the parent program options
    const region = program.opts().region;
    const acl = program.opts().acl;
    console.log(`Uploading ${file} to ${bucket} in ${region} with ACL: ${acl}`);
    // ... upload logic ...
  });

const listCommand = new Command('list')
  .description('List objects in a bucket')
  .argument('<bucket>', 'Bucket to list')
  .option('--prefix <string>', 'Filter by prefix')
  .option('--limit <number>', 'Max results', '100')
  .action(async (bucket) => {
    const opts = program.opts();
    console.log(`Listing ${bucket} prefix=${opts.prefix} limit=${opts.limit}`);
    // ... list logic ...
  });

program.addCommand(uploadCommand);
program.addCommand(listCommand);

program.parseAsync(process.argv);
```

## Expected Behavior

1. Identify that `program.opts()` inside subcommand actions returns the parent program's options, not the subcommand's options
2. Fix the upload action by using the action's own `options` parameter instead of `program.opts()`
3. Fix the list action by using the action's own `options` parameter instead of `program.opts()`
4. Both fixed action handlers receive options as an action parameter (e.g., `.action((bucket, file, options) => {...})`)
5. Add `await` before `program.parseAsync(...)` (the original was missing the `await`)
6. Document in `explanation.md` that `program.opts()` inside a subcommand returns the parent program's options, not the subcommand's options
7. Document in `explanation.md` that `parseAsync` was called without `await`, causing the process to exit before async actions complete
8. Add `try/catch` error handling to action handlers (the original had none)

## Success Criteria

- **Removes program.opts() from upload action**: In the fixed upload command, `program.opts()` is NOT used to read upload-specific options; the action's own options parameter is used instead
- **Removes program.opts() from list action**: In the fixed list command, `program.opts()` is NOT used to read list-specific options; the action's own options parameter is used instead
- **Action parameter used for options**: Both fixed action handlers receive options as an action parameter and read from that object
- **await on parseAsync**: The fixed `cli-fixed.ts` calls `await program.parseAsync(...)` (the original was missing the `await`)
- **Explanation identifies program.opts() bug**: `explanation.md` identifies that reading `program.opts()` inside a subcommand returns the parent program options, not the subcommand's options
- **Explanation identifies missing await**: `explanation.md` identifies that `parseAsync` was called without `await`, causing the process to exit before async actions complete
- **try/catch added**: The fixed code wraps async action logic in `try/catch`

## Failure Conditions

- Agent leaves `program.opts()` calls in the upload or list action handlers
- Agent does not add the options parameter to the action handler signatures
- Agent does not add `await` before `program.parseAsync(...)`
- Agent does not explain the `program.opts()` scoping bug in `explanation.md`
- Agent does not mention the missing `await` issue in `explanation.md`
- Agent does not add error handling to the action handlers
