export const isMultilineDescriptionStart = (value: string): boolean => {
  return value.startsWith("|") || value.startsWith(">");
};
