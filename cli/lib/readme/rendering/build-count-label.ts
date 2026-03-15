export const buildCountLabel = (
  tileCount: number,
  skillCount: number,
): string => {
  if (tileCount > 0 && skillCount > 0)
    return `${tileCount} tile${tileCount !== 1 ? "s" : ""}, ${skillCount} skill${skillCount !== 1 ? "s" : ""}`;
  if (tileCount > 0) return `${tileCount} tile${tileCount !== 1 ? "s" : ""}`;
  return `${skillCount} skill${skillCount !== 1 ? "s" : ""}`;
};
