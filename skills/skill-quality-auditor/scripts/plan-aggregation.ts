#!/usr/bin/env bun
/**
 * Plan aggregation for a skill family
 * Usage: bun run plan-aggregation.ts --family <prefix>
 */

import { readdirSync, existsSync, readFileSync, statSync } from "fs"
import { join } from "path"

interface SkillInfo {
  name: string
  path: string
  lines: number
  description: string
  consolidates?: string
}

interface AggregationPlan {
  family: string
  sourceSkills: SkillInfo[]
  totalLines: number
  proposedCategories: string[]
  estimatedHubLines: number
  estimatedReferences: number
  estimatedReduction: string
  recommendations: string[]
}

function findSkillsByFamily(prefix: string, skillsDir: string): SkillInfo[] {
  if (!existsSync(skillsDir)) {
    return []
  }

  const skills: SkillInfo[] = []
  const entries = readdirSync(skillsDir)

  for (const entry of entries) {
    const skillPath = join(skillsDir, entry, "SKILL.md")
    if (
      existsSync(skillPath) &&
      (entry.startsWith(prefix + "-") || entry === prefix)
    ) {
      const content = readFileSync(skillPath, "utf-8")
      const lines = content.split("\n").length

      const descMatch = content.match(/^description:\s*(.+)$/m)
      const description = descMatch ? descMatch[1] : ""

      const consMatch = content.match(/^consolidates:\s*(.+)$/m)
      const consolidates = consMatch ? consMatch[1] : undefined

      skills.push({
        name: entry,
        path: skillPath,
        lines,
        description,
        consolidates,
      })
    }
  }

  return skills.sort((a, b) => a.name.localeCompare(b.name))
}

function analyzeDuplication(skills: SkillInfo[]): number {
  if (skills.length < 2) return 0

  const contents = skills.map((s) => {
    const content = readFileSync(s.path, "utf-8")
    return new Set(
      content
        .toLowerCase()
        .split("\n")
        .filter((l) => l.trim().length > 10)
    )
  })

  let totalOverlap = 0
  let pairCount = 0

  for (let i = 0; i < contents.length; i++) {
    for (let j = i + 1; j < contents.length; j++) {
      const intersection = [...contents[i]].filter((line) =>
        contents[j].has(line)
      )
      const avgSize = (contents[i].size + contents[j].size) / 2
      if (avgSize > 0) {
        totalOverlap += (intersection.length / avgSize) * 100
        pairCount++
      }
    }
  }

  return pairCount > 0 ? Math.round(totalOverlap / pairCount) : 0
}

function proposeCategories(skills: SkillInfo[]): string[] {
  const categories = new Set<string>()

  for (const skill of skills) {
    const nameParts = skill.name.split("-").slice(1)
    if (nameParts.length > 0) {
      categories.add(nameParts[0])
    }
  }

  if (categories.size === 0) {
    categories.add("core")
    categories.add("advanced")
  }

  return [...categories].sort()
}

function generatePlan(
  family: string,
  skills: SkillInfo[],
  duplication: number
): AggregationPlan {
  const totalLines = skills.reduce((sum, s) => sum + s.lines, 0)
  const proposedCategories = proposeCategories(skills)

  const estimatedHubLines = 65
  const estimatedReferences = Math.ceil(totalLines / 350)

  const estimatedNewTotal = estimatedHubLines + estimatedReferences * 200
  const reduction = ((1 - estimatedNewTotal / totalLines) * 100).toFixed(1)

  const recommendations: string[] = []

  if (skills.length < 3) {
    recommendations.push(
      "Only 1-2 skills found - consider if aggregation is necessary"
    )
  } else if (skills.length >= 6) {
    recommendations.push(
      `${skills.length} skills found - strong candidate for aggregation`
    )
  }

  if (duplication > 30) {
    recommendations.push(
      `High duplication (${duplication}%) - high priority for consolidation`
    )
  } else if (duplication > 20) {
    recommendations.push(
      `Moderate duplication (${duplication}%) - good candidate for consolidation`
    )
  } else {
    recommendations.push(
      `Low duplication (${duplication}%) - review if skills are truly related`
    )
  }

  if (totalLines > 2000) {
    recommendations.push(
      `Large collection (${totalLines} lines) - significant reduction potential`
    )
  }

  recommendations.push("Follow aggregation-implementation.md 6-step process")
  recommendations.push(
    "Create navigation hub (SKILL.md) with priority categories"
  )
  recommendations.push(
    "Extract content to references/ directory by category"
  )

  return {
    family,
    sourceSkills: skills,
    totalLines,
    proposedCategories,
    estimatedHubLines,
    estimatedReferences,
    estimatedReduction: `${reduction}%`,
    recommendations,
  }
}

function formatPlan(plan: AggregationPlan): string {
  const lines: string[] = []

  lines.push(`# Aggregation Plan: ${plan.family}`)
  lines.push("")
  lines.push("## Summary")
  lines.push("")
  lines.push(`- **Source Skills**: ${plan.sourceSkills.length}`)
  lines.push(`- **Total Lines**: ${plan.totalLines}`)
  lines.push(`- **Estimated Hub**: ~${plan.estimatedHubLines} lines`)
  lines.push(`- **Estimated References**: ~${plan.estimatedReferences} files`)
  lines.push(`- **Estimated Reduction**: ${plan.estimatedReduction}`)
  lines.push("")

  lines.push("## Source Skills")
  lines.push("")
  lines.push("| Skill | Lines | Description |")
  lines.push("|-------|-------|-------------|")
  for (const skill of plan.sourceSkills) {
    const shortDesc =
      skill.description.slice(0, 50) + (skill.description.length > 50 ? "..." : "")
    lines.push(`| ${skill.name} | ${skill.lines} | ${shortDesc} |`)
  }
  lines.push("")

  lines.push("## Proposed Categories")
  lines.push("")
  lines.push("| Priority | Category | Prefix | Suggested Content |")
  lines.push("|----------|----------|--------|-------------------|")
  const priorities = ["CRITICAL", "HIGH", "MEDIUM", "LOW"]
  plan.proposedCategories.forEach((cat, i) => {
    const priority = priorities[Math.min(i, priorities.length - 1)]
    lines.push(`| ${priority} | ${cat} | ${cat}- | TBD |`)
  })
  lines.push("")

  lines.push("## Implementation Steps")
  lines.push("")
  lines.push("1. **Design Structure** - Finalize category organization")
  lines.push("2. **Create Hub** - Write SKILL.md navigation (60-90 lines)")
  lines.push("3. **Create AGENTS.md** - Reference guide with file listing")
  lines.push("4. **Extract References** - Move content to categorized files")
  lines.push("5. **Deprecate Sources** - Move to .deprecated/")
  lines.push("6. **Verify** - Run skill-judge evaluation")
  lines.push("")

  lines.push("## Recommendations")
  lines.push("")
  for (let i = 0; i < plan.recommendations.length; i++) {
    lines.push(`${i + 1}. ${plan.recommendations[i]}`)
  }
  lines.push("")

  lines.push("## Next Actions")
  lines.push("")
  lines.push("- [ ] Review this plan with team")
  lines.push("- [ ] Refine category structure")
  lines.push("- [ ] Estimate effort (hours)")
  lines.push("- [ ] Schedule implementation")
  lines.push("- [ ] Execute aggregation-implementation.md")
  lines.push("")

  lines.push("---")
  lines.push("")
  lines.push(`*Generated by skill-quality-auditor*`)

  return lines.join("\n")
}

const args = process.argv.slice(2)
const familyIndex = args.indexOf("--family")
const family = familyIndex !== -1 ? args[familyIndex + 1] : null

if (!family) {
  console.error("Usage: bun run plan-aggregation.ts --family <prefix>")
  console.error("Example: bun run plan-aggregation.ts --family bdd")
  process.exit(1)
}

const skillsDirs = ["skills", ".agents/skills"]
let allSkills: SkillInfo[] = []

for (const dir of skillsDirs) {
  const skills = findSkillsByFamily(family, dir)
  allSkills = allSkills.concat(skills)
}

if (allSkills.length === 0) {
  console.error(`No skills found with prefix: ${family}`)
  process.exit(1)
}

const duplication = analyzeDuplication(allSkills)
const plan = generatePlan(family, allSkills, duplication)

console.log(formatPlan(plan))
