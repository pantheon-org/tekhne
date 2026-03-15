import { mkdirSync, writeFileSync } from "node:fs";
import { getBadgeFilename } from "./get-badge-filename";
import { makeBadgeSvg } from "./make-badge-svg";

const BADGE_DIRS = [".github/badges", "docs/public/.github/badges"];
const GRADES = ["A+", "A", "B+", "B", "C+", "C", "D", "F", "?"];

export const writeBadgeFiles = (): void => {
  for (const dir of BADGE_DIRS) {
    mkdirSync(dir, { recursive: true });
    for (const grade of GRADES) {
      const filename = getBadgeFilename(grade);
      const svg = makeBadgeSvg(grade);
      writeFileSync(`${dir}/${filename}.svg`, svg);
    }
  }
};
