export const getGradeColor = (grade: string): string => {
  const colorMap: Record<string, string> = {
    "A+": "brightgreen",
    A: "green",
    "B+": "yellowgreen",
    B: "yellow",
    "C+": "orange",
    C: "red",
    D: "red",
    F: "purple",
  };
  return colorMap[grade] || "lightgrey";
};
