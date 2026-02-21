#!/usr/bin/env bun
/**
 * Update README.md with skill ratings from audit reports
 * Usage: bun run scripts/update-readme-ratings.ts [--dry-run]
 */

import { existsSync, readdirSync, readFileSync, writeFileSync } from "fs";
import { join } from "path";

const PROJECT_ROOT = join(import.meta.dir, "..");
const README_PATH = join(PROJECT_ROOT, "README.md");
const AUDITS_DIR = join(PROJECT_ROOT, ".context/audits");
const SKILLS_DIR = join(PROJECT_ROOT, "skills");

const DRY_RUN = process.argv.includes("--dry-run");

interface SkillRating {
  score: number;
  max: number;
  grade: string;
}

function extractRatingFromReport(filePath: string): SkillRating | null {
  if (!existsSync(filePath)) return null;

  const content = readFileSync(filePath, "utf-8");

  // Match: | **Total Score** | 83/120 (69.2%) |
  // or: | **Total Score** | 83/120 |
  const scoreMatch = content.match(
    /\*\*Total Score\*\*\s*\|\s*(\d+)\/(\d+)(?:\s*\([^)]+\))?\s*\|/,
  );
  const gradeMatch = content.match(/\*\*Grade\*\*\s*\|\s*([A-F+]+)\s*\|/);

  if (scoreMatch) {
    return {
      score: parseInt(scoreMatch[1], 10),
      max: parseInt(scoreMatch[2], 10),
      grade: gradeMatch ? gradeMatch[1] : "N/A",
    };
  }

  return null;
}

function getLatestRating(skillName: string): SkillRating | null {
  if (!existsSync(AUDITS_DIR)) return null;

  const files = readdirSync(AUDITS_DIR)
    .filter((f) => f.startsWith(`${skillName}-skill-quality-audit-`))
    .sort()
    .reverse();

  for (const file of files) {
    const rating = extractRatingFromReport(join(AUDITS_DIR, file));
    if (rating) return rating;
  }

  return null;
}

function getSkillBadge(rating: SkillRating | null): string {
  if (!rating) return "N/A";

  const gradeColors: Record<string, string> = {
    "A+": "brightgreen",
    A: "brightgreen",
    "B+": "green",
    B: "green",
    "C+": "yellowgreen",
    C: "yellowgreen",
    D: "orange",
    F: "red",
  };

  const color = gradeColors[rating.grade] || "lightgrey";
  return `![${rating.grade}](https://img.shields.io/badge/Rating-${rating.score}%2F${rating.max}-${color})`;
}

function updateReadme(dryRun: boolean): void {
  const readmeContent = readFileSync(README_PATH, "utf-8");
  const lines = readmeContent.split("\n");

  // Find the skills table
  const tableStartIndex = lines.findIndex(
    (line) => line.includes("| Skill") && line.includes("| Description"),
  );

  if (tableStartIndex === -1) {
    console.error("Could not find skills table in README.md");
    process.exit(1);
  }

  // Check if Rating column already exists
  const headerLine = lines[tableStartIndex];
  const hasRatingColumn = headerLine.includes("Rating");

  const result: string[] = [];
  let inTable = false;
  let tableEndFound = false;

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];

    // Check if we're at the table start
    if (i === tableStartIndex) {
      inTable = true;

      if (!hasRatingColumn) {
        // Add Rating column to header
        result.push(line.replace(/\|$/, " | Rating |"));
      } else {
        result.push(line);
      }
      continue;
    }

    // Check if this is the separator line (after header)
    if (inTable && i === tableStartIndex + 1) {
      if (!hasRatingColumn) {
        // Add separator for Rating column
        result.push(line.replace(/\|$/, " | ------ |"));
      } else {
        result.push(line);
      }
      continue;
    }

    // Process table rows
    if (inTable && line.startsWith("| `") && !tableEndFound) {
      // Extract skill name from: | `skill-name` | Description |
      const match = line.match(/\| `([^`]+)` \|/);
      if (match) {
        const skillName = match[1];
        const rating = getLatestRating(skillName);
        const badge = getSkillBadge(rating);

        if (!hasRatingColumn) {
          // Add rating to end of row
          result.push(line.replace(/\|$/, ` | ${badge} |`));
        } else {
          // Update existing rating column
          const parts = line.split("|").map((p) => p.trim());
          if (parts.length >= 4) {
            // parts[0] is empty, parts[1] is skill name, parts[2] is description, parts[3] is rating (if exists)
            if (parts.length >= 5) {
              parts[3] = badge;
              result.push("| " + parts.slice(1).join(" | ") + " |");
            } else {
              result.push(line.replace(/\|$/, ` | ${badge} |`));
            }
          } else {
            result.push(line);
          }
        }
        continue;
      }

      // If we hit a row that doesn't match skill pattern, table has ended
      tableEndFound = true;
    }

    // Check for end of table (empty line or non-table content)
    if (inTable && !line.startsWith("|") && line.trim() !== "") {
      inTable = false;
      tableEndFound = true;
    }

    result.push(line);
  }

  const updatedContent = result.join("\n");

  if (dryRun) {
    console.log("=== DRY RUN - Changes that would be made ===\n");
    // Show diff
    const originalLines = readmeContent.split("\n");
    const updatedLines = updatedContent.split("\n");

    let changes = 0;
    for (
      let i = 0;
      i < Math.max(originalLines.length, updatedLines.length);
      i++
    ) {
      if (originalLines[i] !== updatedLines[i]) {
        changes++;
        if (changes <= 10) {
          console.log(`Line ${i + 1}:`);
          console.log(`  - ${originalLines[i] || "(added)"}`);
          console.log(`  + ${updatedLines[i] || "(removed)"}`);
        }
      }
    }

    console.log(`\nTotal changes: ${changes} lines`);
    console.log("\nTo apply changes, run without --dry-run");
  } else {
    writeFileSync(README_PATH, updatedContent, "utf-8");
    console.log("✅ README.md updated with skill ratings");
  }
}

// Main
updateReadme(DRY_RUN);
