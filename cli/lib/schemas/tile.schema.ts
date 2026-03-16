import { z } from "zod";

export const TileSkillSchema = z.object({
  path: z.string().min(1),
});

export const TileSchema = z.object({
  name: z
    .string()
    .min(1)
    .regex(/^[\w-]+\/[\w-]+$/, "Tile name must be in format: workspace/tile"),
  version: z.string().regex(/^\d+\.\d+\.\d+$/, "Version must be semver"),
  private: z.boolean().optional(),
  summary: z.string().min(1),
  skills: z.record(z.string(), TileSkillSchema).optional(),
});

export type TileData = z.infer<typeof TileSchema>;
export type TileSkillData = z.infer<typeof TileSkillSchema>;
