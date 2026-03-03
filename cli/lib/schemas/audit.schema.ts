import { z } from "zod";

export const DimensionScoreSchema = z.object({
  score: z.number().min(0).max(15),
  rationale: z.string(),
  recommendations: z.array(z.string()).optional(),
});

export const AuditResultSchema = z.object({
  skill: z.string(),
  timestamp: z.string(),
  totalScore: z.number().min(0).max(120),
  grade: z.enum(["A", "B+", "B", "C+", "C", "D", "F"]),
  dimensions: z.object({
    D1_knowledge_delta: DimensionScoreSchema,
    D2_mindset_procedures: DimensionScoreSchema,
    D3_anti_patterns: DimensionScoreSchema,
    D4_specification_compliance: DimensionScoreSchema,
    D5_progressive_disclosure: DimensionScoreSchema,
    D6_freedom_calibration: DimensionScoreSchema,
    D7_pattern_recognition: DimensionScoreSchema,
    D8_practical_usability: DimensionScoreSchema,
  }),
  recommendations: z.array(z.string()).optional(),
});

export type AuditResult = z.infer<typeof AuditResultSchema>;
export type DimensionScore = z.infer<typeof DimensionScoreSchema>;
