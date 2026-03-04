import { existsSync, readFileSync, writeFileSync } from "node:fs";
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
  const cleaned = summary.replace(/\|/g, "\\|").replace(/\n/g, " ").trim();
  return cleaned.length > 80 ? `${cleaned.substring(0, 80)}...` : cleaned;
}

async function buildSkillCell(
  skillDir: string,
  skillName: string,
  auditRelPath: string,
): Promise<string> {
  const skillMdLink = `[${skillName}](${skillDir}/SKILL.md)`;
  const auditInfo = await getLatestAuditInfo(auditRelPath);

  if (auditInfo) {
    const badge = getBadgeMarkdown(auditInfo.grade);
    const auditLink = getAuditLink(auditInfo.date, auditInfo.path);
    return `${skillMdLink} ${badge} ${auditLink}`;
  }

  return `${skillMdLink} ![?](https://img.shields.io/badge/Rating-?-lightgrey)`;
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
    output += `${domainInfo.description}\n\n`;
    output += "| Tile | Description | Skills | Tessl |\n";
    output += "| --- | --- | --- | --- |\n";

    for (const tile of domainTiles) {
      const tileLink = `[${tile.shortName}](${tile.tileDir})`;
      const description = formatSummary(tile.summary);
      const tesslStatus = getTileTessl(tile);

      const skillCells = await Promise.all(
        tile.skills.map((s) => buildSkillCell(s.skillDir, s.name, s.auditRelPath)),
      );
      const skillsCell = skillCells.join("<br>");

      output += `| ${tileLink} | ${description} | ${skillsCell} | ${tesslStatus} |\n`;
    }

    for (const skill of domainUntiledSkills) {
      const displayName = getSkillDisplayName(skill.relativePath);
      const description = parseSkillDescription(`skills/${skill.relativePath}`);
      const auditInfo = await getLatestAuditInfo(skill.relativePath);

      let skillCell: string;
      if (auditInfo) {
        const badge = getBadgeMarkdown(auditInfo.grade);
        const auditLink = getAuditLink(auditInfo.date, auditInfo.path);
        skillCell = `[${displayName}](skills/${skill.relativePath}/SKILL.md) ${badge} ${auditLink}`;
      } else {
        skillCell = `[${displayName}](skills/${skill.relativePath}/SKILL.md) ![?](https://img.shields.io/badge/Rating-?-lightgrey)`;
      }

      output += `| ${displayName} _(no tile)_ | ${description} | ${skillCell} | - |\n`;
    }
  }

  return output;
}

interface ReadmeSections {
  beforeSkills: string[];
  afterSkills: string[];
}

function isSkillSectionStart(line: string, domainHeaders: string[]): boolean {
  return (
    domainHeaders.some((h) => line.startsWith(`## ${h}`)) ||
    line.match(/^\| (Skill|Tile) \| Description/) !== null
  );
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
