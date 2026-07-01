import { existsSync } from "node:fs";
import { join } from "node:path";

export interface SkillManifest {
  name: string;
  version: string;
  summary: string;
  private: boolean;
  skills: string[];
}

export const readManifest = async (
  skillPath: string,
): Promise<SkillManifest | null> => {
  const pluginJson = join(skillPath, ".tessl-plugin", "plugin.json");
  const tileJson = join(skillPath, "tile.json");
  const manifestPath = existsSync(pluginJson)
    ? pluginJson
    : existsSync(tileJson)
      ? tileJson
      : null;
  if (!manifestPath) return null;

  const raw = (await Bun.file(manifestPath).json()) as Record<string, unknown>;
  const skills: string[] = [];
  const rawSkills = raw.skills;

  if (Array.isArray(rawSkills)) {
    for (const entry of rawSkills) {
      if (typeof entry === "string") {
        skills.push(entry);
      } else if (entry && typeof entry === "object") {
        const path = (entry as Record<string, unknown>).path;
        if (typeof path === "string") skills.push(path);
      }
    }
  } else if (rawSkills && typeof rawSkills === "object") {
    for (const def of Object.values(rawSkills) as Array<
      Record<string, unknown>
    >) {
      if (def?.path && typeof def.path === "string") skills.push(def.path);
    }
  }

  return {
    name: (raw.name as string) ?? "",
    version: (raw.version as string) ?? "0.0.0",
    private: raw.private !== false,
    skills,
    summary: (raw.description as string) ?? (raw.summary as string) ?? "",
  };
};
