export const collectSkillFiles = async (
  repoRoot: string,
): Promise<string[]> => {
  const glob = new Bun.Glob("skills/**/SKILL.md");
  const files: string[] = [];
  for await (const file of glob.scan({ cwd: repoRoot, absolute: true })) {
    files.push(file);
  }
  return files.sort();
};
