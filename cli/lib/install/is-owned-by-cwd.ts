export const isOwnedByCwd = (
  sourcePath: string,
  resolvedCwd: string,
): boolean =>
  sourcePath.startsWith(`${resolvedCwd}/`) || sourcePath === resolvedCwd;
