import { basename, dirname } from "node:path";

export const tileRoot = (manifestPath: string): string => {
  const dir = dirname(manifestPath);
  return basename(dir) === ".tessl-plugin" ? dirname(dir) : dir;
};
