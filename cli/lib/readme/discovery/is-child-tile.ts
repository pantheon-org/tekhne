import { dirname } from "node:path";

export const isChildTile = (
  tileDir: string,
  tileDirs: Set<string>,
  isPrivate: boolean,
): boolean => tileDirs.has(dirname(tileDir)) && isPrivate;
