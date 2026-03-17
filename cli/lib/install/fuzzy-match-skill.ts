export const fuzzyMatchSkill = (name: string, pattern: string): boolean => {
  const haystack = name.toLowerCase();
  const needle = pattern.toLowerCase();
  let hi = 0;
  for (let ni = 0; ni < needle.length; ni++) {
    const found = haystack.indexOf(needle[ni], hi);
    if (found === -1) return false;
    hi = found + 1;
  }
  return true;
};
