#!/usr/bin/env bun
import { existsSync, mkdirSync, readdirSync, readFileSync, writeFileSync } from "fs";
import { basename, dirname, join } from "path";

// ============================================================================
// Types
// ============================================================================

interface TemplateSection {
  name: string;
  template: string;
}

interface Template {
  title: string;
  description: string;
  sections: TemplateSection[];
  defaults?: Record<string, unknown>;
}

interface PhaseConfig {
  number: string;
  name: string;
  description: string;
  type: "activities" | "steps";
  items: ItemConfig[];
}

interface ItemConfig {
  number: string;
  name: string;
  description: string;
  subItems?: SubItemConfig[];
}

interface SubItemConfig {
  number: string;
  name: string;
  description: string;
  checklist: string[];
  acceptanceCriteria: string[];
  status?: string;
  dependencies?: string[];
}

interface PlanDefinition {
  phases: PhaseConfig[];
  outputPath: string;
}

// ============================================================================
// Template Engine
// ============================================================================

const parseYamlTemplate = (content: string): Template => {
  const lines = content.split("\n");
  const result: Template = {
    title: "",
    description: "",
    sections: [],
  };

  let currentSection: TemplateSection | null = null;
  let inTemplate = false;
  let templateLines: string[] = [];

  for (const line of lines) {
    // Top-level fields
    if (line.startsWith("title:")) {
      result.title = line
        .replace("title:", "")
        .trim()
        .replace(/^["']|["']$/g, "");
    } else if (line.startsWith("description:")) {
      // Skip multiline description for now
    } else if (line.startsWith("defaults:")) {
      // Skip defaults section header
    } else if (line.match(/^ {2}\w+:/) && !line.startsWith("    ")) {
      // Default value like: "  status: \"Not Started\""
      if (!result.defaults) result.defaults = {};
      const [key, ...valueParts] = line.trim().split(":");
      const value = valueParts
        .join(":")
        .trim()
        .replace(/^["']|["']$/g, "");
      result.defaults[key] = value;
    } else if (line.startsWith("sections:")) {
      // Start sections
    } else if (line.startsWith("  - name:")) {
      // New section
      if (currentSection && templateLines.length > 0) {
        currentSection.template = templateLines.join("\n");
      }
      currentSection = {
        name: line
          .replace("  - name:", "")
          .trim()
          .replace(/^["']|["']$/g, ""),
        template: "",
      };
      result.sections.push(currentSection);
      inTemplate = false;
      templateLines = [];
    } else if (line.startsWith("    template: |")) {
      inTemplate = true;
      templateLines = [];
    } else if (inTemplate && line.startsWith("      ")) {
      templateLines.push(line.substring(6)); // Remove 6-space indent
    } else if (line.startsWith("#")) {
      // Comment - skip
    }
  }

  // Finalize last section
  if (currentSection && templateLines.length > 0) {
    currentSection.template = templateLines.join("\n");
  }

  return result;
};

const loadTemplate = (templatePath: string): Template => {
  const content = readFileSync(templatePath, "utf-8");
  return parseYamlTemplate(content);
};

const kebabCase = (str: string): string =>
  str
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/(^-|-$)/g, "");

const titleCase = (str: string): string => str.charAt(0).toUpperCase() + str.slice(1);

const singularize = (type: "activities" | "steps"): "activity" | "step" =>
  type === "activities" ? "activity" : "step";

interface TemplateContext {
  [key: string]: unknown;
}

const renderTemplate = (template: string, context: TemplateContext): string => {
  let result = template;

  // Simple variable replacement: {{variable}}
  result = result.replace(/\{\{\s*(\w+)\s*\}\}/g, (_, key) => {
    return String(context[key] ?? "");
  });

  // Handlebars-style helpers: {{#each array}}...{{/each}}
  result = result.replace(/\{\{#each\s+(\w+)\}\}([\s\S]*?)\{\{\/each\}\}/g, (_, arrayKey, inner) => {
    const array = context[arrayKey] as unknown[];
    if (!Array || !Array.isArray(array)) return "";

    return array
      .map((item) => {
        let itemResult = inner;
        if (typeof item === "object" && item !== null) {
          // Replace {{key}} with item[key]
          for (const [k, v] of Object.entries(item as Record<string, unknown>)) {
            itemResult = itemResult.replace(new RegExp(`\\{\\{\\s*${k}\\s*\\}\\}`, "g"), String(v ?? ""));
            // Handle {{kebab name}} helper
            itemResult = itemResult.replace(new RegExp(`\\{\\{kebab\\s+${k}\\}\\}`, "g"), kebabCase(String(v ?? "")));
          }
        }
        return itemResult;
      })
      .join("");
  });

  // Handle {{kebab value}} helper
  result = result.replace(/\{\{kebab\s+(\w+)\}\}/g, (_, key) => {
    return kebabCase(String(context[key] ?? ""));
  });

  // Handle {{type_singular|title}} filter syntax
  result = result.replace(/\{\{(\w+)\|title\}\}/g, (_, key) => {
    return titleCase(String(context[key] ?? ""));
  });

  // Handle {{#if condition}}...{{else}}...{{/if}}
  result = result.replace(
    /\{\{#if\s+(\w+)\}\}([\s\S]*?)(\{\{else\}\}([\s\S]*?))?\{\{\/if\}\}/g,
    (_, cond, ifBody, _elsePart, elseBody) => {
      const value = context[cond];
      if (value && (Array.isArray(value) ? value.length > 0 : true)) {
        return ifBody;
      }
      return elseBody ?? "";
    },
  );

  return result.trim();
};

const renderTemplateFile = (template: Template, context: TemplateContext): string => {
  const lines: string[] = [`# ${renderTemplate(template.title, context)}`];

  for (const section of template.sections) {
    lines.push("");
    lines.push(`## ${renderTemplate(section.name, context)}`);
    lines.push(renderTemplate(section.template, context));
  }

  return lines.join("\n");
};

// ============================================================================
// Structure Generator
// ============================================================================

const getTemplatesDir = (): string => {
  const scriptDir = dirname(import.meta.url.replace("file://", ""));
  return join(scriptDir, "..", "templates");
};

const generatePhaseStructure = (basePath: string, phase: PhaseConfig): void => {
  const templatesDir = getTemplatesDir();
  const phaseDir = join(basePath, `phase-${phase.number}-${kebabCase(phase.name)}`);

  console.log(`Creating phase: ${phaseDir}`);
  mkdirSync(phaseDir, { recursive: true });

  // Load templates
  const phaseTemplate = loadTemplate(join(templatesDir, "phase-readme.yaml"));
  const intermediateTemplate = loadTemplate(join(templatesDir, "intermediate-readme.yaml"));
  const groupTemplate = loadTemplate(join(templatesDir, "group-readme.yaml"));
  const stepTemplate = loadTemplate(join(templatesDir, "step-file.yaml"));

  // Phase README
  const typeSingular = singularize(phase.type);
  const phaseContext: TemplateContext = {
    number: phase.number,
    name: phase.name,
    description: phase.description,
    type: phase.type,
    type_singular: typeSingular,
    items: phase.items.map((item) => ({
      number: item.number,
      name: item.name,
      description: item.description,
    })),
  };
  writeFileSync(join(phaseDir, "README.md"), renderTemplateFile(phaseTemplate, phaseContext));

  // Activities or Steps directory
  const typeDir = join(phaseDir, phase.type);
  mkdirSync(typeDir, { recursive: true });

  const intermediateContext: TemplateContext = {
    type: phase.type,
  };
  writeFileSync(join(typeDir, "README.md"), renderTemplateFile(intermediateTemplate, intermediateContext));

  // Group directories
  for (const item of phase.items) {
    const groupDir = join(typeDir, `${typeSingular}-${item.number}-${kebabCase(item.name)}`);

    console.log(`  Creating group: ${groupDir}`);
    mkdirSync(groupDir, { recursive: true });

    // Group README
    const groupContext: TemplateContext = {
      type_singular: typeSingular,
      number: item.number,
      name: item.name,
      description: item.description,
      sub_items_count: item.subItems?.length ?? 0,
      sub_items: item.subItems?.map((sub) => ({
        number: sub.number,
        name: sub.name,
        description: sub.description,
      })),
    };
    writeFileSync(join(groupDir, "README.md"), renderTemplateFile(groupTemplate, groupContext));

    // Leaf files
    if (item.subItems) {
      for (const subItem of item.subItems) {
        const filename = `${typeSingular}-${subItem.number}-${kebabCase(subItem.name)}.md`;
        console.log(`    Creating file: ${filename}`);

        const stepContext: TemplateContext = {
          type_singular: typeSingular,
          number: subItem.number,
          name: subItem.name,
          description: subItem.description,
          checklist: subItem.checklist,
          acceptance_criteria: subItem.acceptanceCriteria,
          status: subItem.status ?? "Not Started",
          dependencies: subItem.dependencies ?? [],
        };
        writeFileSync(join(groupDir, filename), renderTemplateFile(stepTemplate, stepContext));
      }
    }
  }
};

// ============================================================================
// Plan Parsing
// ============================================================================

const parsePlanFile = (filePath: string): PlanDefinition => {
  const content = readFileSync(filePath, "utf-8");

  try {
    return JSON.parse(content) as PlanDefinition;
  } catch {
    throw new Error(`Failed to parse plan file: ${filePath}. Expected JSON format.`);
  }
};

const generateFromPlan = (planPath: string): void => {
  const plan = parsePlanFile(planPath);

  console.log(`Generating structure at: ${plan.outputPath}`);
  mkdirSync(plan.outputPath, { recursive: true });

  for (const phase of plan.phases) {
    generatePhaseStructure(plan.outputPath, phase);
  }

  console.log("\nGeneration complete!");
};

const generateFromExistingPhases = (phasesDir: string): void => {
  console.log(`Scanning existing phases in: ${phasesDir}`);

  if (!existsSync(phasesDir)) {
    console.error(`Directory not found: ${phasesDir}`);
    process.exit(1);
  }

  const entries = readdirSync(phasesDir, { withFileTypes: true });
  const phaseFiles = entries
    .filter((e) => e.isFile() && e.name.match(/^phase-\d+.*\.md$/))
    .map((e) => join(phasesDir, e.name));

  if (phaseFiles.length === 0) {
    console.log("No flat phase files found to split.");
    return;
  }

  console.log(`Found ${phaseFiles.length} phase files to split`);
  console.log("Note: Automatic parsing of markdown requires manual configuration.");
  console.log("Use --plan option with a JSON definition for full automation.");
};

// ============================================================================
// CLI
// ============================================================================

const printUsage = (): void => {
  console.log(`
Usage: 
  bun run generate-structure.ts --plan <plan.json>     Generate from JSON plan
  bun run generate-structure.ts --scan <phases-dir>    Scan existing phases
  bun run generate-structure.ts --example              Print example JSON format

Options:
  --plan <file>    Path to JSON plan definition
  --scan <dir>     Scan directory for existing phase files
  --example        Print example JSON format

Templates are loaded from: ../templates/*.yaml
  - phase-readme.yaml      # Phase-level README
  - intermediate-readme.yaml # activities/steps directory README
  - group-readme.yaml      # Group directory README
  - step-file.yaml         # Step/activity leaf file

Example JSON plan format:
{
  "outputPath": "docs/refactoring/phases",
  "phases": [
    {
      "number": "1",
      "name": "Analysis",
      "description": "Analyze codebase and design architecture",
      "type": "activities",
      "items": [
        {
          "number": "1",
          "name": "Analysis and Design",
          "description": "Complete analysis workflow",
          "subItems": [
            {
              "number": "1.1",
              "name": "Current Codebase Analysis",
              "description": "Map dependencies and identify issues",
              "checklist": ["Map all imports", "Identify coupling"],
              "acceptanceCriteria": ["Analysis report complete"]
            }
          ]
        }
      ]
    }
  ]
}
`);
};

const printExample = (): void => {
  console.log(
    JSON.stringify(
      {
        outputPath: "docs/refactoring/phases",
        phases: [
          {
            number: "1",
            name: "Analysis",
            description: "Analyze codebase and design architecture",
            type: "activities",
            items: [
              {
                number: "1",
                name: "Analysis and Design",
                description: "Complete analysis workflow",
                subItems: [
                  {
                    number: "1.1",
                    name: "Current Codebase Analysis",
                    description: "Map dependencies and identify logic leakage",
                    checklist: [
                      "Map all imports between apps/ and libs/",
                      "Identify Colyseus room logic",
                      "Document Phaser scene logic",
                      "Find shared constants",
                    ],
                    acceptanceCriteria: ["Dependency graph complete", "Logic leakage identified", "Report generated"],
                  },
                  {
                    number: "1.2",
                    name: "Architecture Design",
                    description: "Design new libs/ structure and module boundaries",
                    checklist: [
                      "Define libs/ directory structure",
                      "Specify module boundaries",
                      "Document network protocol",
                    ],
                    acceptanceCriteria: ["Architecture document approved", "Module boundaries defined"],
                  },
                ],
              },
            ],
          },
          {
            number: "2",
            name: "Implementation",
            description: "Extract and refactor game logic",
            type: "steps",
            items: [
              {
                number: "1",
                name: "Extract and Refactor",
                description: "Extract movement logic and refactor rooms",
                subItems: [
                  {
                    number: "1.1",
                    name: "Extract Movement Logic",
                    description: "Move movement functions to libs/game-simulation",
                    checklist: [
                      "Identify all movement functions",
                      "Create libs/game-simulation",
                      "Move and test functions",
                    ],
                    acceptanceCriteria: ["All movement functions extracted", "Tests pass"],
                  },
                ],
              },
            ],
          },
        ],
      },
      null,
      2,
    ),
  );
};

// Main
const args = process.argv.slice(2);

if (args.length === 0 || args.includes("--help") || args.includes("-h")) {
  printUsage();
  process.exit(0);
}

if (args.includes("--example")) {
  printExample();
  process.exit(0);
}

const planIndex = args.indexOf("--plan");
if (planIndex !== -1 && args[planIndex + 1]) {
  generateFromPlan(args[planIndex + 1]);
  process.exit(0);
}

const scanIndex = args.indexOf("--scan");
if (scanIndex !== -1 && args[scanIndex + 1]) {
  generateFromExistingPhases(args[scanIndex + 1]);
  process.exit(0);
}

printUsage();
