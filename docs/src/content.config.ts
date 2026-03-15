import { defineCollection, z } from "astro:content";
import { docsSchema } from "@astrojs/starlight/schema";
import { glob } from "astro/loaders";

/**
 * Starlight requires a `title` field. SKILL.md files use `name` (and
 * optionally `metadata.title`) instead. This preprocessor derives `title`
 * from whichever field is available so both native docs pages and skill pages
 * validate correctly.
 */
function withSkillTitle(schema: z.ZodTypeAny) {
  return z.preprocess((data) => {
    if (typeof data !== "object" || data === null) return data;
    const entry = data as Record<string, unknown>;

    if (!entry.title) {
      // Try metadata.title, then name, then fall back to "Untitled".
      // Spread to avoid mutating the original input object.
      const metaTitle =
        typeof entry.metadata === "object" && entry.metadata !== null
          ? (entry.metadata as Record<string, unknown>).title
          : undefined;
      return { ...entry, title: metaTitle ?? entry.name ?? "Untitled" };
    }

    return entry;
  }, schema);
}

export const collections = {
  docs: defineCollection({
    loader: glob({
      base: "src/content/docs",
      // Serve top-level doc pages, SKILL.md files, and reference docs.
      // Excludes templates/, scripts/, schemas/, assets/, AGENTS.md, etc.
      pattern: ["*.{md,mdx}", "**/SKILL.md", "**/references/*.md"],
    }),
    schema: (ctx) => withSkillTitle(docsSchema()(ctx)),
  }),
};
