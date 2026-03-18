import { defineCollection, z } from "astro:content";
import { glob } from "astro/loaders";

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

const skillRefSchema = z.object({
  slug: z.string(),
  title: z.string(),
});

const skillAuditSchema = z.object({
  skill: z.string().optional(),
  dimensions: z.record(z.string(), z.number()).optional(),
  total: z.number().optional(),
  maxTotal: z.number().optional(),
  grade: z.string().optional(),
  lines: z.number().optional(),
  hasReferences: z.boolean().optional(),
  referenceCount: z.number().optional(),
});

const skillAuditSnapshotSchema = skillAuditSchema.extend({
  date: z.string().optional(),
  auditDate: z.string().optional(),
  totalScore: z.number().optional(),
  fileSize: z.number().optional(),
});

const skillEvalScenarioSchema = z.object({
  id: z.string(),
  capability: z.string().optional(),
  feasible: z.boolean().optional(),
  reason: z.string().optional(),
});

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
    .passthrough()
    .optional(),
  skillRefs: z.array(skillRefSchema).optional(),
  skillAudit: skillAuditSchema.optional(),
  skillAudits: z.array(skillAuditSnapshotSchema).optional(),
  skillEvals: z.array(skillEvalScenarioSchema).optional(),
  tilePublishedUrl: z.string().optional(),
  tileVersion: z.string().optional(),
  tileName: z.string().optional(),
  sidebar: z
    .object({
      label: z.string().optional(),
      order: z.number().optional(),
      hidden: z.boolean().optional(),
    })
    .optional(),
});

const docsBaseSchema = z.object({
  title: z.string(),
  description: z.string().optional(),
});

export const collections = {
  docs: defineCollection({
    loader: glob({
      base: "src/content/docs",
      pattern: ["**/SKILL.mdx", "**/references/*.md"],
    }),
    schema: withSkillTitle(docsBaseSchema.merge(skillExtraSchema)),
  }),
};
