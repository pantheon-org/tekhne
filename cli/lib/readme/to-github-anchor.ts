export const toGitHubAnchor = (headingText: string): string => {
  return headingText
    .toLowerCase()
    .replace(/[^a-z0-9\s-]/g, "")
    .replace(/\s+/g, "-");
};
