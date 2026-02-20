# DOM Snapshots

## Types of Snapshots

### 1. Accessibility Tree (agent-browser)

Quick, lightweight view of interactive elements:

```bash
agent-browser --session test snapshot > dom-snapshot-accessible.txt
```

**Use for**:
- Finding element IDs for clicking (@e42, @e36, etc.)
- Understanding page structure
- Identifying interactive elements

### 2. Full HTML

Complete DOM with all attributes:

```bash
# Via agent-browser
agent-browser --session test eval "document.documentElement.outerHTML" > full-dom.html

# Via Playwright
const html = await page.content();
fs.writeFileSync('full-dom.html', html);
```

**Use for**:
- Inspecting classes and data attributes
- Comparing DOM structure before/after
- Debugging styling issues

## When to Capture

- ✅ Before and after each major interaction
- ✅ When debugging DOM structure issues  
- ✅ To compare element attributes (classes, data attributes)
- ✅ After errors or unexpected behavior

## Example: Comparing DOM States

```bash
# Baseline
agent-browser --session baseline snapshot > baseline-dom.txt

# Changed
agent-browser --session changed snapshot > changed-dom.txt

# Compare
diff baseline-dom.txt changed-dom.txt
```

## Extracting Specific Data

```bash
# Count elements
agent-browser --session test eval "
  {
    codeBlocks: document.querySelectorAll('pre code').length,
    annotations: document.querySelectorAll('mark[data-bind-id]').length,
    buttons: document.querySelectorAll('button').length
  }
" > element-counts.json

# Check for specific classes
agent-browser --session test eval "
  Array.from(document.querySelectorAll('pre code')).map((el, i) => ({
    index: i,
    hasHighlighting: el.querySelectorAll('span.hljs-keyword').length > 0,
    language: el.className.match(/language-(\w+)/)?.[1]
  }))
" > code-block-status.json
```

## Related Documentation

- [Screenshots](./screenshots.md) - Visual evidence
- [Logs](./logs.md) - Build and runtime logs
