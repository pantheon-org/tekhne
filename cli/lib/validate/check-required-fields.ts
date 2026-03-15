const REQUIRED_FIELDS = ["name", "description"] as const;

export interface Frontmatter {
  name?: unknown;
  description?: unknown;
  [key: string]: unknown;
}

/**
 * Validate required fields on an already-parsed frontmatter object.
 * Uses an explicit null/undefined/empty check, including whitespace-only
 * strings, to catch fields that are technically present but provide no value.
 * Exported for testing.
 */
export const checkRequiredFields = (
  filePath: string,
  parsed: Frontmatter,
): string[] => {
  const missing = REQUIRED_FIELDS.filter((f) => {
    const v = parsed[f];
    return v == null || (typeof v === "string" && v.trim() === "");
  });
  if (missing.length > 0) {
    return [
      `${filePath}: frontmatter missing required fields: ${missing.join(", ")}`,
    ];
  }
  return [];
};
