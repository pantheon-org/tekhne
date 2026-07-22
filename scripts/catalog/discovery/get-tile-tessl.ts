import type { TileEntry } from "../types";

export const getTileTessl = (tile: TileEntry): string => {
  if (!tile.isPublic) return "-";
  if (tile.fullName) {
    return `[Public](https://tessl.io/registry/skills/${tile.fullName})`;
  }
  return "Public";
};
