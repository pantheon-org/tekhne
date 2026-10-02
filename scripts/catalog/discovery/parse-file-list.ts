/**
 * Turn the output of `find` (one path per line) into a sorted list.
 *
 * `find` returns paths in whatever order the filesystem enumerates them, which
 * differs between macOS and Linux. The catalogue is generated from these lists,
 * so sort them: the result then does not depend on the machine it ran on, and
 * a CI check can compare it with the committed file. The default sort compares
 * code units, not locale, so it is the same everywhere.
 */
export const parseFileList = (output: string): string[] =>
  output
    .split("\n")
    .map((line) => line.trim())
    .filter(Boolean)
    .sort();
