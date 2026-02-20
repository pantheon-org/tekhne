# Troubleshooting: agent-browser Not Found

## Cause

`agent-browser` is not installed or not in PATH.

## Solutions

### Option 1: Install agent-browser

```bash
# Check if available via npm
npm search agent-browser

# If available:
npm install -g agent-browser
```

### Option 2: Use Playwright Directly

#### Code Generation

```bash
# Generate test code interactively
npx playwright codegen http://localhost:3000
```

#### Write Test Script

See [Playwright Testing](../browser-automation/playwright-testing.md) for examples.

### Option 3: Use the Automation Scripts

The skill provides ready-to-use scripts:

```bash
# Full workflow
~/.config/opencode/skills/ui-debug-workflow/scripts/full-debug-session.sh \
  main fix/feature http://localhost:3000 "npm run build" "npm start"
```

## Verification

```bash
# Check agent-browser
which agent-browser

# Check Playwright
npx playwright --version

# List installed browsers
npx playwright list-chromium
```

## Related Documentation

- [Agent-browser](../browser-automation/agent-browser.md)
- [Playwright Testing](../browser-automation/playwright-testing.md)
- [Scripts](../../scripts/) - Ready-to-use automation scripts
