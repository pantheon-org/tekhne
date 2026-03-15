import { dirname } from "node:path";

export const isChildTile = (
  tileDir: string,
  tileDirs: Set<string>,
  tileData: Record<string, unknown>,
): boolean => tileDirs.has(dirname(tileDir)) && tileData.private === true;
