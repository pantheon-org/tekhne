# Agent-Browser Automation

## Overview

`agent-browser` is a CLI tool for basic browser automation using Playwright. It's suitable for screenshots, DOM inspection, and simple interactions, but has limitations with complex user gestures.

## Installation

```bash
# Install Playwright browsers
npx playwright install chromium

# Verify agent-browser is available
which agent-browser
```

## Basic Commands

```bash
# Start a session and open URL
agent-browser --session mysession open http://localhost:3000

# Take screenshot
agent-browser --session mysession screenshot /tmp/screenshot.png

# Get DOM snapshot (accessibility tree)
agent-browser --session mysession snapshot

# Execute JavaScript
agent-browser --session mysession eval "document.title"

# Click element (use @eN from snapshot)
agent-browser --session mysession click @e42

# Close session
agent-browser --session mysession close
```

## Example: Screenshot Capture Workflow

```bash
#!/bin/bash
# capture-evidence.sh

SESSION="ui-debug-session"
OUTPUT_DIR="./docs/changefix/changed"
URL="http://localhost:19888"

mkdir -p "$OUTPUT_DIR"

# Open the application
agent-browser --session "$SESSION" open "$URL"
sleep 3

# Capture initial state
agent-browser --session "$SESSION" screenshot "$OUTPUT_DIR/01-initial-load.png"

# Get DOM structure
agent-browser --session "$SESSION" snapshot > "$OUTPUT_DIR/01-initial-dom.txt"

# Interact with UI (example: close dialog)
agent-browser --session "$SESSION" click @e36
sleep 1
agent-browser --session "$SESSION" screenshot "$OUTPUT_DIR/02-dialog-closed.png"

# Test feature (example: select text)
agent-browser --session "$SESSION" eval "
  const codeBlock = document.querySelector('pre code');
  codeBlock.scrollIntoView({ block: 'center' });
  const range = document.createRange();
  range.selectNodeContents(codeBlock);
  const sel = window.getSelection();
  sel.removeAllRanges();
  sel.addRange(range);
  'Selected ' + sel.toString().length + ' characters';
"
sleep 1
agent-browser --session "$SESSION" screenshot "$OUTPUT_DIR/03-text-selected.png"

# Inspect final state
agent-browser --session "$SESSION" eval "
  {
    annotations: document.querySelectorAll('mark[data-bind-id]').length,
    codeBlocks: document.querySelectorAll('pre code').length
  }
" > "$OUTPUT_DIR/04-final-state.json"

# Close session
agent-browser --session "$SESSION" close

echo "✅ Evidence captured to $OUTPUT_DIR"
```

## Capabilities

### What agent-browser CAN do

✅ **Navigate and load pages** - Open URLs, follow links  
✅ **Capture visual state** - Take screenshots at any point  
✅ **Inspect DOM** - Get accessibility tree snapshots  
✅ **Execute JavaScript** - Run custom code in the page context  
✅ **Click elements** - Interact with buttons, links (via accessibility tree IDs)  
✅ **Read text content** - Extract text from the page

### What agent-browser CANNOT reliably do

❌ **Complex user gestures** - Text selection, drag-and-drop, multi-touch  
❌ **Trigger UI toolbars** - Programmatic selections don't always trigger contextual UI  
❌ **Hover states** - Mouseover effects may not work as expected  
❌ **Keyboard input sequences** - Complex keyboard interactions  
❌ **Real-time interactions** - Animations, transitions may not complete

## When agent-browser is Insufficient

**Signs you need full Playwright**:

1. **Text selection doesn't trigger UI** - Annotation toolbars, context menus don't appear
2. **Drag-and-drop required** - Moving elements or selecting ranges
3. **Complex event sequences** - Multiple steps that depend on timing
4. **Visual validation** - Pixel-perfect comparison of before/after

**See**: [playwright-testing.md](./playwright-testing.md) for full Playwright examples.

## Related Documentation

- [Playwright Testing](./playwright-testing.md) - Full browser automation
- [Evidence Collection](../evidence-collection/screenshots.md) - Screenshot best practices
- [Troubleshooting](../troubleshooting/blank-screenshots.md) - Common issues
