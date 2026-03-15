export const extractMultilineDescription = (
  lines: string[],
  startIndex: number,
): string => {
  let description = "";
  for (let i = startIndex; i < lines.length; i++) {
    const line = lines[i];
    if (line.trim() === "---") {
      break;
    }
    const trimmed = line.trim();
    if (trimmed && !trimmed.startsWith("-")) {
      description += (description ? " " : "") + trimmed;
    }
  }
  return description;
};
