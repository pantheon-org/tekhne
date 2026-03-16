# Task: Fix Shell Command Injection in Bun Subprocess Code

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
