import { existsSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { $ } from "bun";
import { FileNotFoundError } from "../utils/errors";
import { logger } from "../utils/logger";
import {
  getAuditLink,
  getBadgeMarkdown,
  getLatestAuditInfo,
} from "./audit-info";
import { DOMAINS } from "./domain-config";
import { getSkillDisplayName, parseSkillDescription } from "./skill-parser";
import { findAllTiles, getTileTessl, type TileEntry } from "./tile-parser";

interface UpdateOptions {
  dryRun?: boolean;
}

interface SkillEntry {
  domain: string;
  relativePath: string;
}

async function findAllSkills(): Promise<SkillEntry[]> {
  const output = await $`find skills -name "SKILL.md" -type f`.text();
  const files = output.trim().split("\n").filter(Boolean);

  return files.map((file) => {
    const relativePath = file.replace("skills/", "").replace("/SKILL.md", "");
    const domain = relativePath.split("/")[0];
    return { domain, relativePath };
  });
}

function findUntiledSkills(
  allSkills: SkillEntry[],
  tiles: TileEntry[],
): SkillEntry[] {
  const tiledSkillDirs = new Set(
    tiles.flatMap((t) => t.skills.map((s) => s.skillDir)),
  );
  return allSkills.filter(
    (skill) => !tiledSkillDirs.has(`skills/${skill.relativePath}`),
  );
}

function formatSummary(summary: string): string {
  return summary.replace(/\n/g, " ").trim();
}

function getEvalCount(skillDir: string): number {
  let count = 0;

  // Convention 1: evals/scenario-N/ subdirectories
  const evalsDir = join(skillDir, "evals");
  if (existsSync(evalsDir)) {
    count += readdirSync(evalsDir).filter((f) => f.startsWith("scenario-")).length;
  }

  // Convention 2: evaluation-scenarios/scenario-NN.md files
  const evalScenariosDir = join(skillDir, "evaluation-scenarios");
  if (existsSync(evalScenariosDir)) {
    count += readdirSync(evalScenariosDir).filter((f) =>
      f.startsWith("scenario-") && f.endsWith(".md"),
    ).length;
  }

  return count;
}

async function generateTileSection(tile: TileEntry): Promise<string> {
  const tileLink = `[${tile.shortName}](${tile.tileDir})`;
  const description = formatSummary(tile.summary);
  const tesslStatus = getTileTessl(tile);

  const versionTag = tile.isPublic && tile.version ? `v${tile.version}` : "";
  const meta = tile.isPublic
    ? ` · ${[versionTag, tesslStatus].filter(Boolean).join(" · ")}`
    : "";
  let output = `\n### ${tileLink}\n\n${description}${meta}\n\n`;
  output += "| Skill | Rating | Audit | Evals |\n";
  output += "| --- | --- | --- | --- |\n";

  for (const skill of tile.skills) {
    const skillLink = `[${skill.name}](${skill.skillDir}/SKILL.md)`;
    const auditInfo = await getLatestAuditInfo(skill.auditRelPath);
    const evalCount = getEvalCount(skill.skillDir);
    const evalsCell = evalCount > 0 ? String(evalCount) : "-";

    if (auditInfo) {
      const badge = getBadgeMarkdown(auditInfo.grade);
      const auditLink = getAuditLink(auditInfo.date, auditInfo.path);
      output += `| ${skillLink} | ${badge} | ${auditLink} | ${evalsCell} |\n`;
    } else {
      output += `| ${skillLink} | ![?](https://img.shields.io/badge/Rating-?-lightgrey) | - | ${evalsCell} |\n`;
    }
  }

  return output;
}

async function generateUntiledSkillSection(skill: SkillEntry): Promise<string> {
  const displayName = getSkillDisplayName(skill.relativePath);
  const description = parseSkillDescription(`skills/${skill.relativePath}`);
  const auditInfo = await getLatestAuditInfo(skill.relativePath);
  const skillDir = `skills/${skill.relativePath}`;
  const evalCount = getEvalCount(skillDir);
  const evalsCell = evalCount > 0 ? String(evalCount) : "-";

  let output = `\n### ${displayName} _(no tile)_\n\n${description}\n\n`;
  output += "| Skill | Rating | Audit | Evals |\n";
  output += "| --- | --- | --- | --- |\n";

  const skillLink = `[${displayName}](${skillDir}/SKILL.md)`;
  if (auditInfo) {
    const badge = getBadgeMarkdown(auditInfo.grade);
    const auditLink = getAuditLink(auditInfo.date, auditInfo.path);
    output += `| ${skillLink} | ${badge} | ${auditLink} | ${evalsCell} |\n`;
  } else {
    output += `| ${skillLink} | ![?](https://img.shields.io/badge/Rating-?-lightgrey) | - | ${evalsCell} |\n`;
  }

  return output;
}

async function generateDomainTables(
  tiles: TileEntry[],
  untiledSkills: SkillEntry[],
): Promise<string> {
  let output = "";

  for (const domainInfo of DOMAINS) {
    const domainTiles = tiles.filter((t) => t.domain === domainInfo.key);
    const domainUntiledSkills = untiledSkills.filter(
      (s) => s.domain === domainInfo.key,
    );

    if (domainTiles.length === 0 && domainUntiledSkills.length === 0) continue;

    const tileCount = domainTiles.length;
    const skillCount = domainUntiledSkills.length;
    let countLabel: string;
    if (tileCount > 0 && skillCount > 0) {
      countLabel = `${tileCount} tile${tileCount !== 1 ? "s" : ""}, ${skillCount} skill${skillCount !== 1 ? "s" : ""}`;
    } else if (tileCount > 0) {
      countLabel = `${tileCount} tile${tileCount !== 1 ? "s" : ""}`;
    } else {
      countLabel = `${skillCount} skill${skillCount !== 1 ? "s" : ""}`;
    }

    output += `\n## ${domainInfo.title} (${countLabel})\n\n`;
    output += `${domainInfo.description}\n`;

    for (const tile of domainTiles) {
      output += await generateTileSection(tile);
    }

    for (const skill of domainUntiledSkills) {
      output += await generateUntiledSkillSection(skill);
    }
  }

  return output;
}

interface ReadmeSections {
  beforeSkills: string[];
  afterSkills: string[];
}

function isSkillSectionStart(line: string, domainHeaders: string[]): boolean {
  return domainHeaders.some((h) => line.startsWith(`## ${h}`));
}

function isSkillSectionEnd(line: string, domainHeaders: string[]): boolean {
  return (
    line.startsWith("## ") &&
    !domainHeaders.some((h) => line.startsWith(`## ${h}`))
  );
}

function parseReadmeSections(
  lines: string[],
  domainHeaders: string[],
): ReadmeSections {
  const beforeSkills: string[] = [];
  const afterSkills: string[] = [];
  let inSkillsSection = false;
  let foundEndOfSkills = false;

  for (const line of lines) {
    if (!inSkillsSection && isSkillSectionStart(line, domainHeaders)) {
      inSkillsSection = true;
      continue;
    }

    if (
      inSkillsSection &&
      !foundEndOfSkills &&
      isSkillSectionEnd(line, domainHeaders)
    ) {
      foundEndOfSkills = true;
      afterSkills.push(line);
      continue;
    }

    if (!inSkillsSection) {
      beforeSkills.push(line);
    } else if (foundEndOfSkills) {
      afterSkills.push(line);
    }
  }

  return { beforeSkills, afterSkills };
}

async function showDryRunDiff(
  readmePath: string,
  newContent: string,
): Promise<void> {
  logger.info("=== DRY RUN - Changes that would be made ===\n");

  const tmpFile = "/tmp/readme-new.md";
  writeFileSync(tmpFile, newContent);

  try {
    const diff = await $`diff -u ${readmePath} ${tmpFile}`.text();
    console.log(diff);
  } catch (error: unknown) {
    const err = error as { stdout?: Buffer };
    if (err.stdout) {
      console.log(err.stdout.toString());
    }
  }

  logger.info("\nTo apply changes, run without --dry-run");
}

export async function updateReadme(options: UpdateOptions): Promise<void> {
  const readmePath = "README.md";

  if (!existsSync(readmePath)) {
    throw new FileNotFoundError(readmePath);
  }

  logger.info("Finding all tiles...");
  const tiles = await findAllTiles();
  logger.info(`Found ${tiles.length} tiles`);

  logger.info("Finding all skills...");
  const allSkills = await findAllSkills();

  const untiledSkills = findUntiledSkills(allSkills, tiles);
  logger.info(`Found ${untiledSkills.length} untiled skills`);

  logger.info("Generating domain tables...");
  const newTables = await generateDomainTables(tiles, untiledSkills);

  logger.info("Updating README.md...");
  const content = readFileSync(readmePath, "utf-8");
  const lines = content.split("\n");
  const domainHeaders = DOMAINS.map((d) => d.title);

  const { beforeSkills, afterSkills } = parseReadmeSections(
    lines,
    domainHeaders,
  );

  const newContent = `${beforeSkills.join("\n") + newTables}\n${afterSkills.join("\n")}`;

  if (options.dryRun) {
    await showDryRunDiff(readmePath, newContent);
  } else {
    writeFileSync(readmePath, newContent);
    logger.success(
      `README.md updated with ${tiles.length} tiles across domain-organized tables`,
    );
  }
}
