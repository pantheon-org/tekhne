export const formatDescription = (description: string): string => {
  const cleaned = description.replace(/\|/g, "\\|");
  return cleaned.length > 80 ? `${cleaned.substring(0, 80)}...` : cleaned;
};
