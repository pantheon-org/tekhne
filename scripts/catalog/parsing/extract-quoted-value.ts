export const extractQuotedValue = (value: string): string | null => {
  if (value.startsWith('"') || value.startsWith("'")) {
    const extracted = value.slice(1, -1);
    return extracted.length > 0 ? extracted : null;
  }
  return null;
};
