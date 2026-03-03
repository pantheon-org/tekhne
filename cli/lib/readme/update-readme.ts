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
import { getTesslStatus } from "./tessl-status";

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

async function generateDomainTables(skills: SkillEntry[]): Promise<string> {
  let output = "";

  for (const domainInfo of DOMAINS) {
    const domainSkills = skills.filter((s) => s.domain === domainInfo.key);

    if (domainSkills.length === 0) {
      continue;
    }

    output += `\n## ${domainInfo.title} (${domainSkills.length} skills)\n\n`;
    output += `${domainInfo.description}\n\n`;
    output += "| Skill | Description | Rating | Audit | Tessl |\n";
    output += "| --- | --- | --- | --- | --- |\n";

    for (const skill of domainSkills) {
      const displayName = getSkillDisplayName(skill.relativePath);
      const description = parseSkillDescription(`skills/${skill.relativePath}`);
      const auditInfo = await getLatestAuditInfo(skill.relativePath);

      let badge: string;
      let auditLink: string;

      if (auditInfo) {
        badge = getBadgeMarkdown(auditInfo.grade);
        auditLink = getAuditLink(auditInfo.date, auditInfo.path);
      } else {
        badge = "![?](https://img.shields.io/badge/Rating-?-lightgrey)";
        auditLink = "-";
      }

      const tesslStatus = await getTesslStatus(skill.relativePath);

      output += `| [${displayName}](skills/${skill.relativePath}/SKILL.md) | ${description} | ${badge} | ${auditLink} | ${tesslStatus} |\n`;
    }
  }

  return output;
}

export async function updateReadme(options: UpdateOptions): Promise<void> {
  const readmePath = "README.md";

  if (!existsSync(readmePath)) {
    throw new FileNotFoundError(readmePath);
  }

  logger.info("Finding all skills...");
  const skills = await findAllSkills();
  logger.info(`Found ${skills.length} skills`);

  logger.info("Generating domain tables...");
  const newTables = await generateDomainTables(skills);

  logger.info("Updating README.md...");
  const content = readFileSync(readmePath, "utf-8");
  const lines = content.split("\n");

  const beforeSkills: string[] = [];
  const afterSkills: string[] = [];
  let inSkillsSection = false;
  let foundEndOfSkills = false;

  const domainHeaders = DOMAINS.map((d) => d.title);

  for (const line of lines) {
    if (
      !inSkillsSection &&
      (domainHeaders.some((h) => line.startsWith(`## ${h}`)) ||
        line.match(/^\| Skill \| Description/))
    ) {
      inSkillsSection = true;
      continue;
    }

    if (
      inSkillsSection &&
      !foundEndOfSkills &&
      line.startsWith("## ") &&
      !domainHeaders.some((h) => line.startsWith(`## ${h}`))
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

  const newContent = `${beforeSkills.join("\n") + newTables}\n${afterSkills.join("\n")}`;

  if (options.dryRun) {
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
  } else {
    writeFileSync(readmePath, newContent);
    logger.success("README.md updated with 12 domain-organized skill tables");
  }
}
