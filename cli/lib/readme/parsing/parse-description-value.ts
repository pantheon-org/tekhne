import { extractMultilineDescription } from "./extract-multiline-description";
import { extractQuotedValue } from "./extract-quoted-value";
import { isMultilineDescriptionStart } from "./is-multiline-description-start";

export const parseDescriptionValue = (
  value: string,
  lines: string[],
  lineIndex: number,
): string => {
  const quoted = extractQuotedValue(value);
  if (quoted !== null) {
    return quoted;
  }

  if (value.startsWith('"') || value.startsWith("'")) {
    return "";
  }

  if (isMultilineDescriptionStart(value)) {
    return extractMultilineDescription(lines, lineIndex + 1);
  }

  return value;
};
