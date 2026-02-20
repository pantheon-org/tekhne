import { mkdirSync, readFileSync, writeFileSync } from "fs";
import { nanoid } from "nanoid";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const slugify = (text: string): string => {
  return text
    .toLowerCase()
    .replace(/[^a-z0-9\s-]/g, "")
    .replace(/\s+/g, "-")
    .replace(/-+/g, "-")
    .trim();
};

const generateThreeWordId = (): string => {
  const adjectives = ["happy", "blue", "quick", "smart", "green", "fast", "red", "cool", "warm", "bright"];
  const nouns = ["moon", "sun", "star", "planet", "comet", "galaxy", "cloud", "ocean", "forest", "mountain"];

  const randomAdjective = adjectives[Math.floor(Math.random() * adjectives.length)];
  const randomNoun = nouns[Math.floor(Math.random() * nouns.length)];
  const randomNumber = Math.floor(Math.random() * 1000);

  return `${randomAdjective}-${randomNoun}-${randomNumber}`;
};

const createContextFile = async (type: string, slug: string, content: string) => {
  const validTypes = ["plan", "justification", "scratch"];

  if (!validTypes.includes(type)) {
    console.error(`Error: Invalid type. Must be one of: ${validTypes.join(", ")}`);
    process.exit(1);
  }

  if (!slug || !content) {
    console.error("Error: Both slug and content are required");
    process.exit(1);
  }

  const threeWordId = generateThreeWordId();
  const formattedSlug = slugify(slug);
  const fileName = `${threeWordId}-${formattedSlug}.md`;

  const directories = {
    plan: ".context/plans",
    justification: ".context/justifications",
    scratch: ".context/scratches",
  };

  const targetDir = directories[type as keyof typeof directories];
  const targetPath = join(__dirname, "..", targetDir, fileName);

  const frontmatter = `---
date: ${new Date().toISOString().split("T")[0]}
title: ${slug
    .split("-")
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(" ")}
---`;

  const fullContent = `${frontmatter}

${content}`;

  try {
    mkdirSync(join(__dirname, "..", targetDir), { recursive: true });
    writeFileSync(targetPath, fullContent, "utf8");

    console.log(`Created ${type} file: ${targetPath}`);
    console.log(`Three-word ID: ${threeWordId}`);
  } catch (error) {
    console.error(`Error creating file: ${error}`);
    process.exit(1);
  }
};

const main = async () => {
  const args = process.argv.slice(2);

  if (args.length < 3) {
    console.log(`Usage: bunx .agents/skills/create-context-file/scripts/create-context-file.ts --type [plan|justification|scratch] "slug" "content"`);
    console.log(`
Examples:`);
    console.log(`  bunx .agents/skills/create-context-file/scripts/create-context-file.ts --type plan "feature-name" "Plan content here"`);
    console.log(
      `  bunx .agents/skills/create-context-file/scripts/create-context-file.ts --type justification "decision-name" "Justification content here"`,
    );
    console.log(`  bunx .agents/skills/create-context-file/scripts/create-context-file.ts --type scratch "note-name" "Scratch content here"`);
    process.exit(1);
  }

  const type = args[1];
  const slug = args[2];
  const content = args.slice(3).join(" ");

  await createContextFile(type, slug, content);
};

if (import.meta.main) {
  main().catch(console.error);
}