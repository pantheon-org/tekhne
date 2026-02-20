#!/usr/bin/env bun
import { existsSync, readdirSync, readFileSync, statSync } from "fs";
import { basename, dirname, join, relative } from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const SCHEMAS_DIR = join(__dirname, "..", "schemas");

interface ValidationResult {
  valid: boolean;
  errors: ValidationError[];
  warnings: ValidationWarning[];
  stats: ValidationStats;
}

interface ValidationError {
  path: string;
  message: string;
  type:
    | "missing_readme"
    | "invalid_structure"
    | "broken_link"
    | "invalid_name"
    | "missing_content"
    | "schema_validation";
}

interface ValidationWarning {
  path: string;
  message: string;
  type: "empty_readme" | "incomplete_content" | "suggested_improvement";
}

interface ValidationStats {
  phases: number;
  groups: number;
  leafFiles: number;
  readmes: number;
  totalFiles: number;
  schemaValidated: number;
}

interface ParsedStepFile {
  title: string;
  description: string;
  checklist: string[];
  acceptanceCriteria: string[];
  status: "pending" | "in-progress" | "completed" | "blocked";
  dependencies?: string[];
  notes?: string;
}

interface ParsedReadmeFile {
  title: string;
  phaseHeader?: {
    name: string;
    objectives: string;
  };
  activities?: Array<{ name: string; description: string; path: string }>;
  steps?: Array<{ name: string; description: string; path: string }>;
  files?: Array<{ name: string; description: string; path: string }>;
  prerequisites?: string[];
  successCriteria?: string[];
  rollbackProcedure?: string;
}

const VALID_PATTERNS = {
  phaseDir: /^phase-\d+(?:\.\d+)*-[a-z0-9-]+$/,
  activityDir: /^activity-\d+(?:\.\d+)*-[a-z0-9-]+$/,
  stepDir: /^step-\d+(?:\.\d+)*-[a-z0-9-]+$/,
  leafFile: /^(?:activity|step)-\d+(?:\.\d+)+-[a-z0-9-]+\.md$/,
  readmeFile: /^README\.md$/,
};

const loadSchema = (name: string): object => {
  const schemaPath = join(SCHEMAS_DIR, name);
  if (!existsSync(schemaPath)) {
    throw new Error(`Schema not found: ${schemaPath}`);
  }
  return JSON.parse(readFileSync(schemaPath, "utf-8"));
};

const parseStepFile = (content: string): ParsedStepFile => {
  const result: ParsedStepFile = {
    title: "",
    description: "",
    checklist: [],
    acceptanceCriteria: [],
    status: "pending",
  };

  const lines = content.split("\n");
  let currentSection = "";
  let sectionContent: string[] = [];

  const flushSection = () => {
    const text = sectionContent.join("\n").trim();
    switch (currentSection) {
      case "description":
        result.description = text;
        break;
      case "checklist":
        result.checklist = text
          .split("\n")
          .filter((l) => l.match(/^\s*- \[[ x]\]/))
          .map((l) => l.replace(/^\s*- \[[ x]\]\s*/, "").trim());
        break;
      case "acceptance":
        result.acceptanceCriteria = text
          .split("\n")
          .filter((l) => l.match(/^\s*- /))
          .map((l) => l.replace(/^\s*-\s*/, "").trim());
        break;
      case "status": {
        const statusMatch = text.match(/status:\s*(pending|in-progress|completed|blocked)/i);
        if (statusMatch) {
          result.status = statusMatch[1].toLowerCase() as ParsedStepFile["status"];
        }
        break;
      }
      case "dependencies":
        result.dependencies = text
          .split("\n")
          .filter((l) => l.match(/^\s*- /))
          .map((l) => l.replace(/^\s*-\s*/, "").trim());
        break;
      case "notes":
        result.notes = text;
        break;
    }
    sectionContent = [];
  };

  for (const line of lines) {
    if (line.startsWith("# ")) {
      result.title = line.replace(/^#\s*/, "").trim();
    } else if (line.match(/^##\s+Description/i)) {
      flushSection();
      currentSection = "description";
    } else if (line.match(/^##\s+Checklist/i)) {
      flushSection();
      currentSection = "checklist";
    } else if (line.match(/^##\s+Acceptance\s+Criteria/i)) {
      flushSection();
      currentSection = "acceptance";
    } else if (line.match(/^##\s+Status/i)) {
      flushSection();
      currentSection = "status";
    } else if (line.match(/^##\s+Dependencies/i)) {
      flushSection();
      currentSection = "dependencies";
    } else if (line.match(/^##\s+Notes/i)) {
      flushSection();
      currentSection = "notes";
    } else if (currentSection) {
      sectionContent.push(line);
    }
  }
  flushSection();

  return result;
};

const parseReadmeFile = (content: string, type: "phase" | "group"): ParsedReadmeFile => {
  const result: ParsedReadmeFile = {
    title: "",
  };

  const lines = content.split("\n");
  let currentSection = "";
  let sectionContent: string[] = [];

  const flushSection = () => {
    const text = sectionContent.join("\n").trim();
    switch (currentSection) {
      case "header": {
        const nameMatch = text.match(/\*\*Phase Name\*\*:\s*(.+)/i);
        const objMatch = text.match(/\*\*Objectives\*\*:\s*(.+)/i);
        if (nameMatch) result.phaseHeader = { name: nameMatch[1].trim(), objectives: objMatch?.[1]?.trim() || "" };
        break;
      }
      case "activities":
      case "steps": {
        const items = text
          .split("\n")
          .filter((l: string) => l.match(/^\s*\|/))
          .slice(2);
        const parsed = items.map((l: string) => {
          const cols = l.split("|").map((c: string) => c.trim());
          return {
            name: cols[1] || "",
            description: cols[2] || "",
            path: cols[3]?.replace(/^\[.*\]\((.*)\)$/, "$1") || "",
          };
        });
        if (currentSection === "activities") result.activities = parsed;
        else result.steps = parsed;
        break;
      }
      case "files":
        result.files = text
          .split("\n")
          .filter((l: string) => l.match(/^\s*- /))
          .map((l: string) => {
            const match = l.match(/^\s*-\s+\[([^\]]+)\]\(([^)]+)\)\s*-?\s*(.*)/);
            return match ? { name: match[1], path: match[2], description: match[3].trim() } : null;
          })
          .filter(Boolean) as ParsedReadmeFile["files"];
        break;
      case "prerequisites":
        result.prerequisites = text
          .split("\n")
          .filter((l: string) => l.match(/^\s*- /))
          .map((l: string) => l.replace(/^\s*-\s*/, "").trim());
        break;
      case "success":
        result.successCriteria = text
          .split("\n")
          .filter((l: string) => l.match(/^\s*- /))
          .map((l: string) => l.replace(/^\s*-\s*/, "").trim());
        break;
      case "rollback":
        result.rollbackProcedure = text;
        break;
    }
    sectionContent = [];
  };

  for (const line of lines) {
    if (line.startsWith("# ")) {
      result.title = line.replace(/^#\s*/, "").trim();
    } else if (line.match(/^##\s+Phase\s+Header/i)) {
      flushSection();
      currentSection = "header";
    } else if (line.match(/^##\s+Activities/i)) {
      flushSection();
      currentSection = "activities";
    } else if (line.match(/^##\s+Steps/i)) {
      flushSection();
      currentSection = "steps";
    } else if (line.match(/^##\s+Files/i)) {
      flushSection();
      currentSection = "files";
    } else if (line.match(/^##\s+Prerequisites/i)) {
      flushSection();
      currentSection = "prerequisites";
    } else if (line.match(/^##\s+Success\s+Criteria/i)) {
      flushSection();
      currentSection = "success";
    } else if (line.match(/^##\s+Rollback/i)) {
      flushSection();
      currentSection = "rollback";
    } else if (currentSection) {
      sectionContent.push(line);
    }
  }
  flushSection();

  return result;
};

interface SchemaDefinition {
  required?: string[];
  properties?: Record<string, SchemaProperty>;
}

interface SchemaProperty {
  type?: string;
  minLength?: number;
  items?: unknown;
  enum?: string[];
}

interface ParsedData {
  [key: string]: unknown;
}

const validateAgainstSchema = (data: unknown, schema: object, _filePath: string): string[] => {
  const errors: string[] = [];
  const schemaObj = schema as SchemaDefinition;
  const dataObj = data as ParsedData;

  if (schemaObj.required) {
    for (const field of schemaObj.required) {
      if (!(field in dataObj)) {
        errors.push(`Missing required field: ${field}`);
      }
    }
  }

  if (schemaObj.properties) {
    for (const [key, propSchema] of Object.entries(schemaObj.properties)) {
      const value = dataObj[key];

      if (value === undefined) continue;

      if (propSchema.type === "string" && typeof value !== "string") {
        errors.push(`Field '${key}' must be a string`);
      } else if (propSchema.type === "array") {
        if (!Array.isArray(value)) {
          errors.push(`Field '${key}' must be an array`);
        } else if (propSchema.minLength && value.length < propSchema.minLength) {
          errors.push(`Field '${key}' must have at least ${propSchema.minLength} items`);
        }
      } else if (propSchema.type === "object" && typeof value !== "object") {
        errors.push(`Field '${key}' must be an object`);
      }

      if (propSchema.enum && typeof value === "string" && !propSchema.enum.includes(value)) {
        errors.push(`Field '${key}' must be one of: ${propSchema.enum.join(", ")}`);
      }
    }
  }

  return errors;
};

const validateStructure = (rootPath: string): ValidationResult => {
  const errors: ValidationError[] = [];
  const warnings: ValidationWarning[] = [];
  const stats: ValidationStats = { phases: 0, groups: 0, leafFiles: 0, readmes: 0, totalFiles: 0, schemaValidated: 0 };

  if (!existsSync(rootPath)) {
    errors.push({ path: rootPath, message: "Root path does not exist", type: "invalid_structure" });
    return { valid: false, errors, warnings, stats };
  }

  let stepSchema: object;
  let readmeSchema: object;

  try {
    stepSchema = loadSchema("step-file.schema.json");
    readmeSchema = loadSchema("readme-file.schema.json");
  } catch (e) {
    errors.push({
      path: SCHEMAS_DIR,
      message: `Failed to load schemas: ${(e as Error).message}`,
      type: "schema_validation",
    });
    return { valid: false, errors, warnings, stats };
  }

  const phaseDirs = getDirectories(rootPath).filter((d) => VALID_PATTERNS.phaseDir.test(d));
  stats.phases = phaseDirs.length;

  for (const phaseDir of phaseDirs) {
    const phasePath = join(rootPath, phaseDir);
    validatePhase(phasePath, errors, warnings, stats, stepSchema, readmeSchema);
  }

  return {
    valid: errors.length === 0,
    errors,
    warnings,
    stats,
  };
};

const validatePhase = (
  phasePath: string,
  errors: ValidationError[],
  warnings: ValidationWarning[],
  stats: ValidationStats,
  stepSchema: object,
  readmeSchema: object,
): void => {
  const readmePath = join(phasePath, "README.md");
  if (!existsSync(readmePath)) {
    errors.push({ path: phasePath, message: "Missing README.md", type: "missing_readme" });
  } else {
    stats.readmes++;
    const content = readFileSync(readmePath, "utf-8");
    const parsed = parseReadmeFile(content, "phase");
    const schemaErrors = validateAgainstSchema(parsed, readmeSchema, readmePath);
    if (schemaErrors.length > 0) {
      stats.schemaValidated++;
      for (const err of schemaErrors) {
        errors.push({ path: readmePath, message: err, type: "schema_validation" });
      }
    } else {
      stats.schemaValidated++;
    }
  }

  const activitiesPath = join(phasePath, "activities");
  const stepsPath = join(phasePath, "steps");

  if (existsSync(activitiesPath)) {
    validateContainer(activitiesPath, "activity", errors, warnings, stats, stepSchema, readmeSchema);
  } else if (existsSync(stepsPath)) {
    validateContainer(stepsPath, "step", errors, warnings, stats, stepSchema, readmeSchema);
  } else {
    errors.push({
      path: phasePath,
      message: "Missing 'activities' or 'steps' directory",
      type: "invalid_structure",
    });
  }
};

const validateContainer = (
  containerPath: string,
  type: "activity" | "step",
  errors: ValidationError[],
  warnings: ValidationWarning[],
  stats: ValidationStats,
  stepSchema: object,
  readmeSchema: object,
): void => {
  const readmePath = join(containerPath, "README.md");
  if (!existsSync(readmePath)) {
    errors.push({ path: containerPath, message: "Missing README.md", type: "missing_readme" });
  } else {
    stats.readmes++;
  }

  const pattern = type === "activity" ? VALID_PATTERNS.activityDir : VALID_PATTERNS.stepDir;
  const groupDirs = getDirectories(containerPath).filter((d) => pattern.test(d));

  for (const groupDir of groupDirs) {
    stats.groups++;
    const groupPath = join(containerPath, groupDir);
    validateGroup(groupPath, type, errors, warnings, stats, stepSchema, readmeSchema);
  }
};

const validateGroup = (
  groupPath: string,
  type: "activity" | "step",
  errors: ValidationError[],
  warnings: ValidationWarning[],
  stats: ValidationStats,
  stepSchema: object,
  readmeSchema: object,
): void => {
  const readmePath = join(groupPath, "README.md");
  if (!existsSync(readmePath)) {
    errors.push({ path: groupPath, message: "Missing README.md", type: "missing_readme" });
  } else {
    stats.readmes++;
    const content = readFileSync(readmePath, "utf-8");
    const parsed = parseReadmeFile(content, "group");
    const schemaErrors = validateAgainstSchema(parsed, readmeSchema, readmePath);
    for (const err of schemaErrors) {
      errors.push({ path: readmePath, message: err, type: "schema_validation" });
    }
    validateLinks(readmePath, groupPath, errors);
  }

  const leafFiles = getFiles(groupPath).filter((f) => VALID_PATTERNS.leafFile.test(f) && f !== "README.md");

  for (const leafFile of leafFiles) {
    stats.leafFiles++;
    stats.totalFiles++;
    const leafPath = join(groupPath, leafFile);
    validateLeafFile(leafPath, errors, warnings, stepSchema, stats);
  }
};

const validateLeafFile = (
  filePath: string,
  errors: ValidationError[],
  warnings: ValidationWarning[],
  schema: object,
  stats: ValidationStats,
): void => {
  if (!existsSync(filePath)) return;

  const content = readFileSync(filePath, "utf-8");
  const parsed = parseStepFile(content);
  const schemaErrors = validateAgainstSchema(parsed, schema, filePath);

  stats.schemaValidated++;

  if (schemaErrors.length > 0) {
    for (const err of schemaErrors) {
      errors.push({ path: filePath, message: err, type: "schema_validation" });
    }
  }

  if (parsed.checklist.length === 0) {
    warnings.push({
      path: filePath,
      message: "Checklist section exists but has no items",
      type: "incomplete_content",
    });
  }

  if (parsed.acceptanceCriteria.length === 0) {
    warnings.push({
      path: filePath,
      message: "Acceptance Criteria section exists but has no items",
      type: "incomplete_content",
    });
  }
};

const validateLinks = (readmePath: string, basePath: string, errors: ValidationError[]): void => {
  if (!existsSync(readmePath)) return;

  const content = readFileSync(readmePath, "utf-8");
  const linkPattern = /\[([^\]]+)\]\(([^)]+)\)/g;
  let match: RegExpExecArray | null;

  match = linkPattern.exec(content);
  while (match !== null) {
    const linkPath = match[2];

    if (linkPath.startsWith("http") || linkPath.startsWith("#")) {
      match = linkPattern.exec(content);
      continue;
    }

    const fullPath = join(basePath, linkPath);

    if (!existsSync(fullPath)) {
      errors.push({
        path: readmePath,
        message: `Broken link: [${match[1]}](${linkPath})`,
        type: "broken_link",
      });
    }
    match = linkPattern.exec(content);
  }
};

const validateNaming = (rootPath: string): ValidationError[] => {
  const errors: ValidationError[] = [];

  const validateDirNames = (dir: string): void => {
    const entries = readdirSync(dir, { withFileTypes: true });

    for (const entry of entries) {
      if (!entry.isDirectory()) continue;

      const name = entry.name;

      if (name.match(/^(?:phase|activity|step)-\d+$/)) {
        errors.push({
          path: join(dir, name),
          message: `Directory name lacks description: ${name}`,
          type: "invalid_name",
        });
      }

      validateDirNames(join(dir, name));
    }
  };

  validateDirNames(rootPath);
  return errors;
};

const getDirectories = (path: string): string[] =>
  readdirSync(path, { withFileTypes: true })
    .filter((e) => e.isDirectory())
    .map((e) => e.name);

const getFiles = (path: string): string[] =>
  readdirSync(path, { withFileTypes: true })
    .filter((e) => e.isFile())
    .map((e) => e.name);

const printReport = (result: ValidationResult, rootPath: string): void => {
  console.log("\n" + "=".repeat(60));
  console.log("IMPLEMENTATION PLAN STRUCTURE VALIDATION REPORT");
  console.log("=".repeat(60));
  console.log(`\nRoot: ${rootPath}`);
  console.log(`Status: ${result.valid ? "✅ VALID" : "❌ INVALID"}`);

  console.log("\n📊 Statistics:");
  console.log(`   Phases:           ${result.stats.phases}`);
  console.log(`   Groups:           ${result.stats.groups}`);
  console.log(`   Leaf Files:       ${result.stats.leafFiles}`);
  console.log(`   READMEs:          ${result.stats.readmes}`);
  console.log(`   Schema Validated: ${result.stats.schemaValidated}`);
  console.log(`   Total Files:      ${result.stats.totalFiles}`);

  if (result.errors.length > 0) {
    console.log("\n❌ Errors:");
    for (const error of result.errors) {
      const relPath = relative(rootPath, error.path) || error.path;
      console.log(`   [${error.type}] ${relPath}`);
      console.log(`      ${error.message}`);
    }
  }

  if (result.warnings.length > 0) {
    console.log("\n⚠️  Warnings:");
    for (const warning of result.warnings) {
      const relPath = relative(rootPath, warning.path) || warning.path;
      console.log(`   [${warning.type}] ${relPath}`);
      console.log(`      ${warning.message}`);
    }
  }

  console.log("\n" + "=".repeat(60));

  if (result.valid) {
    console.log("✅ Structure is valid!");
  } else {
    console.log(`❌ Found ${result.errors.length} error(s) that need to be fixed.`);
  }
  console.log("=".repeat(60) + "\n");
};

const printUsage = (): void => {
  console.log(`
Usage:
  bun run validate-structure.ts <phases-directory>
  bun run validate-structure.ts --help

Options:
  <phases-directory>   Path to the phases directory to validate
  --help, -h           Show this help message
  --schema             Show schema information

Examples:
  bun run validate-structure.ts docs/refactoring/phases
  bun run validate-structure.ts ./phases

Validation checks:
  ✓ Every directory has a README.md
  ✓ Directory names include descriptions (e.g., step-1-extract-logic)
  ✓ Leaf files validated against step-file.schema.json
  ✓ README files validated against readme-file.schema.json
  ✓ Required fields: title, description, checklist, acceptanceCriteria, status
  ✓ All links in READMEs resolve
  ✓ Proper hierarchy (phase → activities/steps → groups → leaf files)
`);
};

const printSchemaInfo = (): void => {
  console.log(`
Schema Files:
  ${join(SCHEMAS_DIR, "step-file.schema.json")}
    - Validates step/activity markdown files
    - Required: title, description, checklist, acceptanceCriteria, status

  ${join(SCHEMAS_DIR, "readme-file.schema.json")}
    - Validates README.md files at each level
    - Phase README: phaseHeader, activities/steps, successCriteria
    - Group README: files, prerequisites
`);
};

const args = process.argv.slice(2);

if (args.length === 0 || args.includes("--help") || args.includes("-h")) {
  printUsage();
  process.exit(0);
}

if (args.includes("--schema")) {
  printSchemaInfo();
  process.exit(0);
}

const rootPath = args[0];
console.log(`Validating structure at: ${rootPath}`);

const result = validateStructure(rootPath);

const namingErrors = validateNaming(rootPath);
result.errors.push(...namingErrors);
result.valid = result.errors.length === 0;

printReport(result, rootPath);

process.exit(result.valid ? 0 : 1);
