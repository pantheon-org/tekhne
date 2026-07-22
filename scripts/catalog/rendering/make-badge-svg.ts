import { makeBadge } from "badge-maker";
import { getGradeColor } from "../parsing";

export const makeBadgeSvg = (grade: string): string => {
  const color = getGradeColor(grade);
  return makeBadge({ label: "Rating", message: grade, color });
};
