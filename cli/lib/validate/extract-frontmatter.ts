/**
 * Extract raw YAML frontmatter string from a Markdown file.
 * Returns null if no frontmatter block is found.
 * Exported for testing.
 */
export const extractFrontmatter = (content: string): string | null => {
  // Match opening ---, capture body, then closing --- which may be followed by
  // a newline or appear at end-of-file (no trailing newline required).
  const match = content.match(/^---\r?\n([\s\S]*?)\r?\n---(?:\r?\n|$)/);
  return match ? match[1] : null;
};
