export const parseShortName = (fullName: string): string =>
  fullName.includes("/") ? fullName.split("/").slice(1).join("/") : fullName;
