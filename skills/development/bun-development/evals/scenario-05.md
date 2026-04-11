# Scenario 05: Fix Shell Command Injection in Bun Subprocess Code

## User Prompt

A Bun API endpoint processes user-provided file paths and formats. Review the following code for command injection vulnerabilities. For each vulnerability, explain the attack vector and produce a safe replacement.

```typescript
import { Database } from "bun:sqlite";

const db = new Database("files.db");

// Endpoint: POST /convert
// Body: { inputPath: string, format: string, outputDir: string }
export async function convertFile(
  inputPath: string,
  format: string,
  outputDir: string
): Promise<string> {
  // Convert the file using ImageMagick
  const proc = Bun.spawn(["sh", "-c", `convert ${inputPath} -format ${format} ${outputDir}/output.${format}`]);
  await proc.exited;

  // Log the conversion
  const logEntry = `${new Date().toISOString()} converted ${inputPath} to ${format}`;
  db.query(`INSERT INTO conversion_log (entry) VALUES ('${logEntry}')`).run();

  return `${outputDir}/output.${format}`;
}

// Endpoint: GET /preview
// Query: ?path=...
export async function previewFile(userPath: string): Promise<string> {
  const proc = Bun.spawn(["sh", "-c", `cat ${userPath}`]);
  const output = await new Response(proc.stdout).text();
  return output;
}
```

Produce a secure rewrite of this module. Explain each vulnerability, describe what an attacker could do, and show how the fixed code prevents it.

## Expected Behavior

1. Identify shell injection in `convertFile` via `["sh", "-c", ...]` with interpolated user-controlled variables — explain a concrete attack vector
2. Identify SQL injection in the `db.query()` call with interpolated `logEntry` — explain the risk
3. Identify shell injection in `previewFile` via `["sh", "-c", `cat ${userPath}`]` — explain what an attacker could read or execute
4. Replace `["sh", "-c", ...]` patterns with direct argument arrays to `Bun.spawn` (no shell interpolation)
5. Replace the interpolated SQL query with `db.prepare()` and bound parameters
6. Produce a complete secure rewrite with all three vulnerabilities fixed

## Success Criteria

- Agent identifies the shell injection vulnerability in `convertFile` from using `sh -c` with interpolated user input
- Agent identifies the SQL injection vulnerability in the `db.query()` log insertion
- Agent identifies the shell injection vulnerability in `previewFile` from using `sh -c` with a user-provided path
- Agent replaces `["sh", "-c", interpolated_string]` with direct argument arrays (e.g., `["convert", inputPath, "-format", format, ...]`) so no shell parsing occurs
- Agent replaces the interpolated SQL query with `db.prepare()` and bound parameters
- Agent provides a concrete attack scenario for each vulnerability

## Failure Conditions

- Agent misses the SQL injection in the `db.query()` log insertion
- Agent replaces `["sh", "-c", ...]` with another shell-interpolation pattern instead of a direct argument array
- Agent does not provide concrete attack examples for the vulnerabilities
- Agent retains `["sh", "-c", ...]` for any of the subprocess calls in the secure rewrite
- Agent leaves template literal interpolation in any `db.query()` or `db.prepare()` string
