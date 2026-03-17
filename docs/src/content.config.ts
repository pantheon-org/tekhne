import { defineCollection, z } from "astro:content";
import { docsSchema } from "@astrojs/starlight/schema";
import { glob } from "astro/loaders";

/**
 * Starlight requires a `title` field. SKILL.md files use `name` (and
 * optionally `metadata.title` / `metadata.displayTitle`) instead.
 *
 * This preprocessor derives `title` from the best available source:
 *   1. metadata.displayTitle — H1 text extracted by prelink.mjs (most readable)
 *   2. metadata.title        — explicit override in frontmatter
 *   3. name                  — kebab-case skill name
 *   4. "Untitled"            — last resort
 */
function withSkillTitle(schema: z.ZodTypeAny) {
  return z.preprocess((data) => {
    if (typeof data !== "object" || data === null) return data;
    const entry = data as Record<string, unknown>;

    if (!entry.title) {
      const meta =
        typeof entry.metadata === "object" && entry.metadata !== null
          ? (entry.metadata as Record<string, unknown>)
          : {};

      const bestTitle =
        (meta.displayTitle as string | undefined) ??
        (meta.title as string | undefined) ??
        (entry.name as string | undefined) ??
        "Untitled";

      return { ...entry, title: bestTitle };
    }

    return entry;
  }, schema);
}

/** Extra frontmatter fields used by SKILL.md files. */
const skillExtraSchema = z.object({
  name: z.string().optional(),
  description: z.string().optional(),
  metadata: z
    .object({
      displayTitle: z.string().optional(),
      title: z.string().optional(),
      version: z.string().optional(),
      grade: z.string().optional(),
      tags: z.array(z.string()).optional(),
      category: z.string().optional(),
    })
    .passthrough() // allow any additional metadata fields
    .optional(),
});

export const collections = {
  docs: defineCollection({
    loader: glob({
      base: "src/content/docs",
      // Serve top-level doc pages, SKILL.md files, and reference docs.
      // Excludes templates/, scripts/, schemas/, assets/, AGENTS.md, etc.
      pattern: ["*.{md,mdx}", "**/SKILL.mdx", "**/references/*.md"],
    }),
    schema: (ctx) => withSkillTitle(docsSchema()(ctx).merge(skillExtraSchema)),
  }),
};
