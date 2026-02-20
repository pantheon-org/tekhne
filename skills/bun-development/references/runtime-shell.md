# Shell Command Execution

Bun provides safe shell command execution with the `Bun.$` tagged template.

## Bun.$ Basics

The `$` tagged template automatically escapes arguments to prevent command injection:

```typescript
const userInput = "file.txt; rm -rf /";

// SAFE: userInput is escaped
const output = await Bun.$`cat ${userInput}`.text();

// Equivalent to: cat 'file.txt; rm -rf /'
```

## Command Execution

### Capture Output

```typescript
// Text output
const stdout = await Bun.$`ls -la`.text();

// Raw bytes
const bytes = await Bun.$`cat binary.dat`.bytes();

// JSON output
const data = await Bun.$`npm list --json`.json();

// Stream output
const stream = Bun.$`tail -f app.log`.readable();
```

### Exit Code & Stderr

```typescript
const result = await Bun.$`npm test`;

console.log(result.exitCode);  // 0 for success
console.log(result.stdout);    // standard output
console.log(result.stderr);    // error output
```

### Command Chaining

```typescript
// Sequential execution
await Bun.$`git add . && git commit -m ${message} && git push`;

// Pipe commands
const output = await Bun.$`cat file.txt | grep "pattern"`.text();
```

## Bun.spawn Alternative

For more control, use Bun.spawn:

```typescript
const proc = Bun.spawn(["git", "status"], {
  stdout: "pipe",
  stderr: "pipe",
});

const stdout = await Bun.readableStreamToText(proc.stdout);
const stderr = await Bun.readableStreamToText(proc.stderr);

await proc.exited;
console.log("Exit code:", proc.exitCode);
```

## Bun.which

Find binary paths:

```typescript
const gitPath = await Bun.which("git");
// => "/usr/bin/git"

if (gitPath) {
  await Bun.$`${gitPath} status`;
}
```

## Security Considerations

### SAFE: Bun.$ Tagged Template

```typescript
// Arguments are ALWAYS escaped
await Bun.$`rm ${filename}`;
```

### CRITICAL: bash -c Bypass

```typescript
// DON'T: This bypasses escaping!
await Bun.$`bash -c "rm ${filename}"`;  // VULNERABLE

// Attacker can inject: file.txt; rm -rf /
```

### HIGH: Argument Injection

```typescript
const userInput = "--help; rm -rf /";

// Vulnerable to flag injection
await Bun.$`ls ${userInput}`;

// FIX: Use -- to end arguments
await Bun.$`ls -- ${userInput}`;
```

### Validation

Always validate user input:

```typescript
function isValidFilename(name: string): boolean {
  return /^[a-zA-Z0-9._-]+$/.test(name) && !name.startsWith(".");
}

if (isValidFilename(userInput)) {
  await Bun.$`cat ${userInput}`;
} else {
  throw new Error("Invalid filename");
}
```

## Best Practices

1. **Use Bun.$ for shell commands** - automatic escaping
2. **Validate user input** before passing to commands
3. **Use -- to prevent flag injection** in arguments
4. **Avoid bash -c** - it bypasses escaping
5. **Use Bun.spawn for complex scenarios** with explicit args array

## Common Patterns

### Running NPM Scripts

```typescript
await Bun.$`npm run build`;
await Bun.$`npm test`;
```

### Git Operations

```typescript
const branch = await Bun.$`git rev-parse --abbrev-ref HEAD`.text();
await Bun.$`git add .`;
await Bun.$`git commit -m ${"feat: new feature"}`;
```

### File Operations

```typescript
// Find files
const files = await Bun.$`find . -name "*.ts"`.text();

// Process files
for (const file of files.trim().split("\n")) {
  await Bun.$`prettier --write ${file}`;
}
```

## Anti-patterns

- **DON'T** use bash -c to bypass escaping
- **DON'T** pass untrusted input without validation
- **DON'T** ignore exit codes
- **DON'T** forget -- to prevent flag injection

## References

- https://bun.sh/docs/runtime/shell
- https://bun.sh/docs/api/spawn
