# Debug and Fix a Broken CLI Tool

## Problem Description

A developer has built a CLI tool for managing cloud storage buckets, but they're reporting that two of the commands behave strangely: the `upload` command always uses the wrong region regardless of what the user passes, and the `list` command ignores its `--prefix` option.

After looking at the code, a senior engineer suspects the problem is with how options are being read inside the subcommand handlers.

The following is the broken CLI code. Identify the bugs, fix them, and write a short explanation of what was wrong.

## Output Specification

Create a file called `cli-fixed.ts` with the corrected code and a file called `explanation.md` explaining what was wrong and why.

## Input Files

The following file contains the broken implementation. Extract it before starting.

=============== FILE: inputs/cli-broken.ts ===============
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
