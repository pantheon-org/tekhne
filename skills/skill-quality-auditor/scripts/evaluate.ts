#!/usr/bin/env bun
/**
 * Evaluate a single skill using the skill-judge framework
 * Usage: bun run evaluate.ts <skill-name> [--json]
 */

import { readFileSync, existsSync, readdirSync, statSync } from "fs"
import { join, dirname } from "path"

interface SkillScore {
  name: string
  dimensions: {
    knowledgeDelta: number
    mindsetProcedures: number
    antiPatternQuality: number
    specificationCompliance: number
    progressiveDisclosure: number
    freedomCalibration: number
    patternRecognition: number
    practicalUsability: number
  }
  total: number
  maxTotal: number
  grade: string
  lines: number
  hasReferences: boolean
  referenceCount: number
}

function findSkillPath(skillName: string): string | null {
  const possiblePaths = [
    `skills/${skillName}/SKILL.md`,
    `.agents/skills/${skillName}/SKILL.md`,
  ]

  for (const path of possiblePaths) {
    if (existsSync(path)) {
      return path
    }
  }
  return null
}

function countLines(content: string): number {
  return content.split("\n").length
}

function hasReferences(skillPath: string): { has: boolean; count: number } {
  const skillDir = dirname(skillPath)
  const refsPath = join(skillDir, "references")

  if (!existsSync(refsPath)) {
    return { has: false, count: 0 }
  }

  try {
    const files = readdirSync(refsPath).filter(
      (f) => f.endsWith(".md") && !f.startsWith(".")
    )
    return { has: true, count: files.length }
  } catch {
    return { has: false, count: 0 }
  }
}

function extractFrontmatter(content: string): Record<string, string> {
  const match = content.match(/^---\n([\s\S]*?)\n---/)
  if (!match) return {}

  const frontmatter: Record<string, string> = {}
  match[1].split("\n").forEach((line) => {
    const [key, ...valueParts] = line.split(":")
    if (key && valueParts.length) {
      frontmatter[key.trim()] = valueParts.join(":").trim()
    }
  })
  return frontmatter
}

function evaluateKnowledgeDelta(content: string): number {
  let score = 15

  const redundantPatterns = [
    /npm install/gi,
    /yarn add/gi,
    /pip install/gi,
    /getting started/gi,
    /introduction/gi,
    /basic syntax/gi,
    /hello world/gi,
  ]

  for (const pattern of redundantPatterns) {
    if (pattern.test(content)) {
      score -= 2
    }
  }

  const expertPatterns = [
    /anti-pattern/gi,
    /NEVER/gi,
    /ALWAYS/gi,
    /production/gi,
    /gotcha/gi,
    /pitfall/gi,
  ]

  for (const pattern of expertPatterns) {
    if (pattern.test(content)) {
      score += 1
    }
  }

  return Math.min(20, Math.max(0, score))
}

function evaluateMindsetProcedures(content: string): number {
  let score = 8

  if (/##\s*(mindset|philosophy|principles)/gi.test(content)) score += 2
  if (/\d+\.\s+/g.test(content)) score += 2
  if (/when to (use|apply)/gi.test(content)) score += 2
  if (/when not to/gi.test(content)) score += 1

  return Math.min(15, Math.max(0, score))
}

function evaluateAntiPatternQuality(content: string): number {
  let score = 8

  const neverCount = (content.match(/NEVER/gi) || []).length
  score += Math.min(3, neverCount)

  if (/❌.*✅/gs.test(content) || /BAD.*GOOD/gis.test(content)) score += 2
  if (/WHY:|why:/gi.test(content)) score += 2

  return Math.min(15, Math.max(0, score))
}

function evaluateSpecificationCompliance(
  content: string,
  frontmatter: Record<string, string>
): number {
  let score = 10

  const description = frontmatter.description || ""
  if (description.length > 100) score += 3
  if (description.length > 200) score += 2
  if (frontmatter.name) score += 0

  return Math.min(15, Math.max(0, score))
}

function evaluateProgressiveDisclosure(
  content: string,
  refsInfo: { has: boolean; count: number }
): number {
  const lines = countLines(content)

  if (refsInfo.has && refsInfo.count > 0) {
    if (lines < 100) return 15
    if (lines < 150) return 13
    if (lines < 200) return 11
    return 10
  }

  if (lines < 200) return 12
  if (lines < 300) return 10
  if (lines < 500) return 7
  return 5
}

function evaluateFreedomCalibration(content: string): number {
  let score = 10

  const neverAlwaysCount = (content.match(/NEVER|ALWAYS/gi) || []).length
  if (neverAlwaysCount > 5) score += 3
  else if (neverAlwaysCount > 2) score += 2

  if (/consider|optionally|may/gi.test(content)) score += 2

  return Math.min(15, Math.max(0, score))
}

function evaluatePatternRecognition(
  frontmatter: Record<string, string>
): number {
  const description = frontmatter.description || ""
  const keywords = description.split(/[\s,]+/).filter((w) => w.length > 3)

  if (keywords.length > 15) return 10
  if (keywords.length > 10) return 9
  if (keywords.length > 5) return 8
  return 6
}

function evaluatePracticalUsability(content: string): number {
  let score = 8

  const codeBlocks = (content.match(/```/g) || []).length / 2
  if (codeBlocks > 5) score += 4
  else if (codeBlocks > 2) score += 2

  if (/\.\//.test(content) || /bun run/.test(content)) score += 2
  if (/```(bash|shell|typescript|javascript)/.test(content)) score += 1

  return Math.min(15, Math.max(0, score))
}

function calculateGrade(total: number): string {
  if (total >= 114) return "A+"
  if (total >= 108) return "A"
  if (total >= 102) return "B+"
  if (total >= 96) return "B"
  if (total >= 90) return "C+"
  if (total >= 84) return "C"
  if (total >= 78) return "D"
  return "F"
}

function evaluateSkill(skillName: string): SkillScore | null {
  const skillPath = findSkillPath(skillName)
  if (!skillPath) {
    console.error(`Skill not found: ${skillName}`)
    return null
  }

  const content = readFileSync(skillPath, "utf-8")
  const frontmatter = extractFrontmatter(content)
  const refsInfo = hasReferences(skillPath)
  const lines = countLines(content)

  const dimensions = {
    knowledgeDelta: evaluateKnowledgeDelta(content),
    mindsetProcedures: evaluateMindsetProcedures(content),
    antiPatternQuality: evaluateAntiPatternQuality(content),
    specificationCompliance: evaluateSpecificationCompliance(
      content,
      frontmatter
    ),
    progressiveDisclosure: evaluateProgressiveDisclosure(content, refsInfo),
    freedomCalibration: evaluateFreedomCalibration(content),
    patternRecognition: evaluatePatternRecognition(frontmatter),
    practicalUsability: evaluatePracticalUsability(content),
  }

  const total = Object.values(dimensions).reduce((a, b) => a + b, 0)

  return {
    name: skillName,
    dimensions,
    total,
    maxTotal: 120,
    grade: calculateGrade(total),
    lines,
    hasReferences: refsInfo.has,
    referenceCount: refsInfo.count,
  }
}

function formatOutput(score: SkillScore, json: boolean): string {
  if (json) {
    return JSON.stringify(score, null, 2)
  }

  return `
Skill: ${score.name}
================================

Dimensions:
  D1: Knowledge Delta         ${score.dimensions.knowledgeDelta}/20
  D2: Mindset + Procedures    ${score.dimensions.mindsetProcedures}/15
  D3: Anti-Pattern Quality    ${score.dimensions.antiPatternQuality}/15
  D4: Specification           ${score.dimensions.specificationCompliance}/15
  D5: Progressive Disclosure  ${score.dimensions.progressiveDisclosure}/15
  D6: Freedom Calibration     ${score.dimensions.freedomCalibration}/15
  D7: Pattern Recognition     ${score.dimensions.patternRecognition}/10
  D8: Practical Usability     ${score.dimensions.practicalUsability}/15

Total: ${score.total}/${score.maxTotal}
Grade: ${score.grade}

Metadata:
  Lines: ${score.lines}
  References: ${score.referenceCount} files
`.trim()
}

const args = process.argv.slice(2)
const jsonOutput = args.includes("--json")
const skillName = args.find((a) => !a.startsWith("--"))

if (!skillName) {
  console.error("Usage: bun run evaluate.ts <skill-name> [--json]")
  process.exit(1)
}

const score = evaluateSkill(skillName)
if (score) {
  console.log(formatOutput(score, jsonOutput))
}
