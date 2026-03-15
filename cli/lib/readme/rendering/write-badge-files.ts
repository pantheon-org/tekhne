import { mkdirSync, writeFileSync } from "node:fs";
import { getBadgeFilename } from "./get-badge-filename";
import { makeBadgeSvg } from "./make-badge-svg";

const BADGE_DIR = ".github/badges";
const GRADES = ["A+", "A", "B+", "B", "C+", "C", "D", "F", "?"];

export const writeBadgeFiles = (): void => {
  mkdirSync(BADGE_DIR, { recursive: true });
  for (const grade of GRADES) {
    const filename = getBadgeFilename(grade);
    const svg = makeBadgeSvg(grade);
    writeFileSync(`${BADGE_DIR}/${filename}.svg`, svg);
  }
};
