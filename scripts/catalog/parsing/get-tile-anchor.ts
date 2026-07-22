export const getTileAnchor = (shortName: string): string => {
  return shortName.toLowerCase().replace(/\s+/g, "-");
};
