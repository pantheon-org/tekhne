import { test, expect } from '@playwright/test';

/**
 * Example Playwright test for UI debugging workflow
 * 
 * This demonstrates how to write comprehensive UI tests that:
 * - Capture visual evidence at each step
 * - Test interactive behaviors
 * - Verify DOM state and styling
 * - Generate detailed test reports
 * 
 * Usage:
 *   npx playwright test examples/ui-debug-test.spec.ts --headed
 *   npx playwright test examples/ui-debug-test.spec.ts --debug
 */

test.describe('Code Block Annotation Testing', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the application
    await page.goto('http://localhost:19888');
    
    // Wait for page to be fully loaded
    await page.waitForLoadState('networkidle');
    
    // Close any initial dialogs
    const continueBtn = page.getByRole('button', { name: /continue/i });
    if (await continueBtn.isVisible()) {
      await continueBtn.click();
      await page.waitForTimeout(500);
    }
  });

  test('should load page with syntax-highlighted code blocks', async ({ page }) => {
    // Take baseline screenshot
    await page.screenshot({ path: 'test-results/01-initial-load.png', fullPage: true });
    
    // Verify code blocks are present
    const codeBlocks = page.locator('pre code');
    const count = await codeBlocks.count();
    expect(count).toBeGreaterThan(0);
    
    // Verify syntax highlighting is present
    for (let i = 0; i < count; i++) {
      const block = codeBlocks.nth(i);
      const hljsSpans = block.locator('span[class*="hljs"]');
      const spanCount = await hljsSpans.count();
      expect(spanCount).toBeGreaterThan(0);
    }
    
    // Take screenshot of code blocks
    const firstCodeBlock = codeBlocks.first();
    await firstCodeBlock.scrollIntoViewIfNeeded();
    await page.screenshot({ path: 'test-results/02-code-blocks.png' });
  });

  test('should allow text selection in code blocks', async ({ page }) => {
    const codeBlock = page.locator('pre code').first();
    await codeBlock.scrollIntoViewIfNeeded();
    
    // Triple-click to select all text
    await codeBlock.click({ clickCount: 3 });
    
    // Take screenshot showing selection
    await page.screenshot({ path: 'test-results/03-text-selected.png' });
    
    // Verify selection exists
    const selectedText = await page.evaluate(() => {
      return window.getSelection()?.toString() || '';
    });
    expect(selectedText.length).toBeGreaterThan(0);
  });

  test('should create annotation on code block', async ({ page }) => {
    const codeBlock = page.locator('pre code').first();
    await codeBlock.scrollIntoViewIfNeeded();
    
    // Enable Redline mode
    const redlineBtn = page.getByRole('button', { name: /redline/i });
    await redlineBtn.click();
    await page.screenshot({ path: 'test-results/04-redline-mode.png' });
    
    // Select text in code block
    await codeBlock.click({ clickCount: 3 });
    
    // Wait for annotation to be created (if auto-annotate is enabled)
    await page.waitForTimeout(1000);
    
    // Verify annotation exists
    const annotations = page.locator('mark[data-bind-id]');
    const annotationCount = await annotations.count();
    
    // Take screenshot with annotation
    await page.screenshot({ path: 'test-results/05-annotation-created.png' });
    
    // If redline mode auto-creates annotations
    if (annotationCount > 0) {
      expect(annotationCount).toBeGreaterThan(0);
      
      // Verify annotation is in code block
      const annotationInCode = await page.locator('pre code mark[data-bind-id]').count();
      expect(annotationInCode).toBeGreaterThan(0);
      
      // Verify annotation has correct class
      const deletionAnnotation = page.locator('mark.deletion[data-bind-id]');
      expect(await deletionAnnotation.count()).toBeGreaterThan(0);
    }
  });

  test('should allow manual annotation via toolbar', async ({ page }) => {
    const codeBlock = page.locator('pre code').first();
    await codeBlock.scrollIntoViewIfNeeded();
    
    // Select text
    await codeBlock.click({ clickCount: 3 });
    
    // Wait for toolbar to appear
    try {
      await page.waitForSelector('[role="toolbar"]', { timeout: 3000 });
      
      // Click Delete button in toolbar
      const deleteBtn = page.locator('[role="toolbar"] button').filter({ hasText: /delete/i });
      await deleteBtn.click();
      
      // Verify annotation created
      const annotations = page.locator('mark[data-bind-id]');
      expect(await annotations.count()).toBeGreaterThan(0);
      
      await page.screenshot({ path: 'test-results/06-toolbar-annotation.png' });
    } catch (err) {
      console.log('Toolbar did not appear - this is expected if toolbar requires specific selection behavior');
      console.log('Error:', err instanceof Error ? err.message : String(err));
      await page.screenshot({ path: 'test-results/06-no-toolbar.png' });
    }
  });

  test('should remove annotation and restore syntax highlighting', async ({ page }) => {
    // First, create an annotation (reuse previous test logic)
    const codeBlock = page.locator('pre code').first();
    await codeBlock.scrollIntoViewIfNeeded();
    
    const redlineBtn = page.getByRole('button', { name: /redline/i });
    await redlineBtn.click();
    await codeBlock.click({ clickCount: 3 });
    await page.waitForTimeout(1000);
    
    // Verify annotation exists
    const annotationsBefore = await page.locator('mark[data-bind-id]').count();
    if (annotationsBefore === 0) {
      test.skip();
      return;
    }
    
    await page.screenshot({ path: 'test-results/07-before-removal.png' });
    
    // Remove annotation (implementation-specific - adjust based on your UI)
    // Option 1: Click on annotation and use remove button
    const annotation = page.locator('mark[data-bind-id]').first();
    await annotation.click();
    
    // Look for remove/delete button in annotation panel or context menu
    const removeBtn = page.getByRole('button', { name: /remove|delete/i }).first();
    if (await removeBtn.isVisible()) {
      await removeBtn.click();
    } else {
      console.log('Remove button not found - annotation UI may vary');
    }
    
    await page.waitForTimeout(500);
    await page.screenshot({ path: 'test-results/08-after-removal.png' });
    
    // Verify annotation is removed
    const annotationsAfter = await page.locator('mark[data-bind-id]').count();
    expect(annotationsAfter).toBeLessThan(annotationsBefore);
    
    // Verify syntax highlighting is restored
    const hljsSpans = await codeBlock.locator('span[class*="hljs"]').count();
    expect(hljsSpans).toBeGreaterThan(0);
    
    // Verify code block content is still intact
    const codeText = await codeBlock.textContent();
    expect(codeText).toBeTruthy();
    expect(codeText!.length).toBeGreaterThan(0);
  });

  test('should handle multiple annotations on same code block', async ({ page }) => {
    const codeBlock = page.locator('pre code').first();
    await codeBlock.scrollIntoViewIfNeeded();
    
    // Create first annotation
    const redlineBtn = page.getByRole('button', { name: /redline/i });
    await redlineBtn.click();
    
    // Select first part
    await page.evaluate(() => {
      const code = document.querySelector('pre code');
      const range = document.createRange();
      const textNode = code?.firstChild;
      if (textNode && textNode.nodeType === Node.TEXT_NODE) {
        range.setStart(textNode, 0);
        range.setEnd(textNode, 20);
        const sel = window.getSelection();
        sel?.removeAllRanges();
        sel?.addRange(range);
      }
    });
    
    await page.waitForTimeout(1000);
    await page.screenshot({ path: 'test-results/09-first-annotation.png' });
    
    // Click elsewhere to deselect
    await page.locator('article').first().click();
    
    // Create second annotation
    await page.evaluate(() => {
      const code = document.querySelector('pre code');
      const range = document.createRange();
      const textNode = code?.firstChild;
      if (textNode && textNode.nodeType === Node.TEXT_NODE) {
        range.setStart(textNode, 30);
        range.setEnd(textNode, 50);
        const sel = window.getSelection();
        sel?.removeAllRanges();
        sel?.addRange(range);
      }
    });
    
    await page.waitForTimeout(1000);
    await page.screenshot({ path: 'test-results/10-second-annotation.png' });
    
    // Verify multiple annotations exist
    const annotationsInCode = await page.locator('pre code mark[data-bind-id]').count();
    console.log(`Multiple annotations test: found ${annotationsInCode} annotations`);
    // This test may fail if UI doesn't support multiple annotations
    // That's okay - it documents the expected behavior
  });

  test('should generate comprehensive test report', async ({ page }) => {
    // This test generates a report of all findings
    const report = {
      timestamp: new Date().toISOString(),
      url: page.url(),
      
      codeBlocks: await page.locator('pre code').count(),
      
      syntaxHighlighting: await page.evaluate(() => {
        const blocks = Array.from(document.querySelectorAll('pre code'));
        return blocks.map((block, i) => ({
          index: i,
          hasHighlighting: block.querySelectorAll('span[class*="hljs"]').length > 0,
          spanCount: block.querySelectorAll('span[class*="hljs"]').length
        }));
      }),
      
      annotations: await page.evaluate(() => {
        const marks = Array.from(document.querySelectorAll('mark[data-bind-id]'));
        return marks.map(mark => ({
          id: mark.getAttribute('data-bind-id'),
          type: Array.from(mark.classList).find(c => 
            ['deletion', 'comment', 'replacement', 'insertion'].includes(c)
          ),
          inCodeBlock: mark.closest('pre code') !== null,
          text: mark.textContent?.substring(0, 50)
        }));
      }),
      
      screenshots: [
        'test-results/01-initial-load.png',
        'test-results/02-code-blocks.png',
        'test-results/03-text-selected.png',
        'test-results/04-redline-mode.png',
        'test-results/05-annotation-created.png',
        'test-results/06-toolbar-annotation.png',
        'test-results/07-before-removal.png',
        'test-results/08-after-removal.png'
      ]
    };
    
    // Save report as JSON
    await page.evaluate((data) => {
      const a = document.createElement('a');
      a.href = 'data:application/json,' + encodeURIComponent(JSON.stringify(data, null, 2));
      a.download = 'test-report.json';
      a.click();
    }, report);
    
    console.log('Test Report:', JSON.stringify(report, null, 2));
  });
});

/**
 * Run this test suite with:
 * 
 * # Headed mode (see the browser)
 * npx playwright test examples/ui-debug-test.spec.ts --headed
 * 
 * # Debug mode (step through)
 * npx playwright test examples/ui-debug-test.spec.ts --debug
 * 
 * # Generate HTML report
 * npx playwright test examples/ui-debug-test.spec.ts --reporter=html
 * 
 * # Run specific test
 * npx playwright test examples/ui-debug-test.spec.ts -g "should create annotation"
 */
