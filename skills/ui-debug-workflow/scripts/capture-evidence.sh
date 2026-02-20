#!/bin/bash
# capture-evidence.sh - Automated evidence collection for UI debugging
#
# Usage: ./capture-evidence.sh <session-name> <url> <output-dir>
#
# Example:
#   ./capture-evidence.sh ui-test http://localhost:19888 ./docs/change-fix/changed

set -e

SESSION="${1:-ui-debug-session}"
URL="${2:-http://localhost:3000}"
OUTPUT_DIR="${3:-./docs/change-fix/$(date +%Y-%m-%d-%H%M%S)/evidence}"

mkdir -p "$OUTPUT_DIR"

echo "=== UI Debug Evidence Capture ==="
echo "Session: $SESSION"
echo "URL: $URL"
echo "Output: $OUTPUT_DIR"
echo ""

# Step 1: Open the application
echo "[1/8] Opening application..."
agent-browser --session "$SESSION" open "$URL"
sleep 3

# Step 2: Capture initial state
echo "[2/8] Capturing initial load..."
agent-browser --session "$SESSION" screenshot "$OUTPUT_DIR/01-initial-load.png"
agent-browser --session "$SESSION" snapshot > "$OUTPUT_DIR/01-initial-dom.txt"

# Step 3: Check for permission dialog and close it
echo "[3/8] Checking for dialogs..."
agent-browser --session "$SESSION" eval "
  const dialog = document.querySelector('dialog[open]') || 
                 document.querySelector('[role=\"dialog\"]') ||
                 document.querySelector('.modal');
  if (dialog) {
    const continueBtn = dialog.querySelector('button') || 
                        Array.from(dialog.querySelectorAll('button')).find(b => 
                          b.textContent.toLowerCase().includes('continue') ||
                          b.textContent.toLowerCase().includes('ok') ||
                          b.textContent.toLowerCase().includes('close')
                        );
    if (continueBtn) {
      continueBtn.click();
      'Dialog closed';
    } else {
      'Dialog found but no close button';
    }
  } else {
    'No dialog found';
  }
" > "$OUTPUT_DIR/02-dialog-check.log"

sleep 2
agent-browser --session "$SESSION" screenshot "$OUTPUT_DIR/02-dialog-closed.png"

# Step 4: Count and highlight code blocks
echo "[4/8] Analyzing code blocks..."
agent-browser --session "$SESSION" eval "
  const codeBlocks = document.querySelectorAll('pre code');
  codeBlocks.forEach((block, i) => {
    block.style.outline = '3px solid red';
    block.setAttribute('data-test-index', i);
  });
  JSON.stringify({
    totalCodeBlocks: codeBlocks.length,
    languages: Array.from(codeBlocks).map(b => b.className.match(/language-(\w+)/)?.[1] || 'unknown')
  });
" > "$OUTPUT_DIR/03-code-blocks.json"

sleep 1
agent-browser --session "$SESSION" screenshot "$OUTPUT_DIR/03-code-blocks-highlighted.png"

# Step 5: Test code block selection
echo "[5/8] Testing code block selection..."
agent-browser --session "$SESSION" eval "
  const codeBlock = document.querySelector('pre code');
  if (codeBlock) {
    codeBlock.scrollIntoView({ block: 'center' });
    const range = document.createRange();
    range.selectNodeContents(codeBlock);
    const sel = window.getSelection();
    sel.removeAllRanges();
    sel.addRange(range);
    JSON.stringify({
      selected: true,
      length: sel.toString().length,
      preview: sel.toString().substring(0, 100)
    });
  } else {
    JSON.stringify({ selected: false, error: 'No code block found' });
  }
" > "$OUTPUT_DIR/04-selection-test.json"

sleep 1
agent-browser --session "$SESSION" screenshot "$OUTPUT_DIR/04-text-selected.png"

# Step 6: Check for annotations
echo "[6/8] Checking for annotations..."
agent-browser --session "$SESSION" eval "
  const annotations = document.querySelectorAll('mark[data-bind-id]');
  const annotationsList = Array.from(annotations).map(mark => ({
    id: mark.getAttribute('data-bind-id'),
    type: Array.from(mark.classList).find(c => ['deletion', 'comment', 'replacement', 'insertion'].includes(c)),
    text: mark.textContent.substring(0, 50),
    inCodeBlock: mark.closest('pre code') !== null
  }));
  JSON.stringify({
    totalAnnotations: annotations.length,
    annotations: annotationsList
  });
" > "$OUTPUT_DIR/05-annotations.json"

sleep 1
agent-browser --session "$SESSION" screenshot "$OUTPUT_DIR/05-annotations-check.png"

# Step 7: Inspect syntax highlighting
echo "[7/8] Checking syntax highlighting..."
agent-browser --session "$SESSION" eval "
  const codeBlocks = document.querySelectorAll('pre code');
  const highlightingStatus = Array.from(codeBlocks).map((block, i) => {
    const hljsSpans = block.querySelectorAll('span[class*=\"hljs\"]');
    return {
      index: i,
      hasHighlighting: hljsSpans.length > 0,
      spanCount: hljsSpans.length,
      classes: Array.from(new Set(Array.from(hljsSpans).map(s => s.className))).slice(0, 10)
    };
  });
  JSON.stringify({
    codeBlocks: highlightingStatus.length,
    allHighlighted: highlightingStatus.every(s => s.hasHighlighting),
    details: highlightingStatus
  });
" > "$OUTPUT_DIR/06-syntax-highlighting.json"

# Step 8: Capture final state
echo "[8/8] Capturing final state..."
agent-browser --session "$SESSION" eval "document.documentElement.outerHTML" > "$OUTPUT_DIR/07-full-html.html"
agent-browser --session "$SESSION" screenshot "$OUTPUT_DIR/07-final-state.png"

# Close the session
echo ""
echo "Closing browser session..."
agent-browser --session "$SESSION" close

# Generate summary
echo ""
echo "=== Evidence Collection Complete ==="
echo "Files created:"
ls -lh "$OUTPUT_DIR" | tail -n +2 | awk '{print "  - " $9 " (" $5 ")"}'
echo ""
echo "Next steps:"
echo "  1. Review screenshots in $OUTPUT_DIR"
echo "  2. Compare with baseline evidence"
echo "  3. Fill out debug report template"
echo ""
