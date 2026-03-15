const CATALOG_URL = "https://pantheon-org.github.io/tekhne/tiles/";

export const buildReadmeCatalogLine = (
  tileCount: number,
  skillCount: number,
): string => {
  return `Browse all **${skillCount} skills across ${tileCount} tiles** in the [Skill Catalog](${CATALOG_URL}).`;
};
