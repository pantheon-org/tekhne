import { describe, expect, test } from "bun:test";
import { join } from "node:path";
import { checkTechEvaluation } from "../../skills/project-mgmt/tech-evaluation/scripts/validate-tech-evaluation";
import { registerGoldenTests } from "../validator-recording/golden";

const SCRIPT = join(
  import.meta.dir,
  "../../skills/project-mgmt/tech-evaluation/scripts/validate-tech-evaluation.ts",
);

registerGoldenTests(SCRIPT, join(import.meta.dir, "fixtures"));

const schema = {
  properties: {
    evaluation: {
      required: ["title", "questions", "recommendation"],
      properties: {
        questions: {
          items: {
            required: ["id", "evidence"],
            properties: {
              evidence: { items: { required: ["claim", "source"] } },
            },
          },
        },
        recommendation: {
          required: ["verdict", "rationale", "single_action"],
          properties: {
            verdict: { enum: ["adopt", "reject"] },
            rationale: { minLength: 5 },
            single_action: { minLength: 3 },
          },
        },
      },
    },
  },
};

const good = {
  evaluation: {
    title: "t",
    questions: [{ id: "q", evidence: [{ claim: "c", source: "s" }] }],
    recommendation: {
      verdict: "adopt",
      rationale: "long enough",
      single_action: "act",
    },
  },
};

describe("checkTechEvaluation", () => {
  test("returns no error for a good evaluation", () => {
    expect(checkTechEvaluation("f", good, schema)).toEqual([]);
  });

  test("says a question with no evidence, with the reason", () => {
    const bad = {
      evaluation: {
        ...good.evaluation,
        questions: [{ id: "q1", evidence: [] }],
      },
    };
    expect(checkTechEvaluation("f", bad, schema)).toEqual([
      "f: questions[0] ('q1') has no evidence -- a verdict without a cited source is a guess, not a finding",
    ]);
  });

  test("checks the verdict enum with python formatting, and the minimum lengths", () => {
    const bad = {
      evaluation: {
        ...good.evaluation,
        recommendation: {
          verdict: "maybe",
          rationale: "x",
          single_action: "y",
        },
      },
    };
    expect(checkTechEvaluation("f", bad, schema)).toEqual([
      "f: recommendation.verdict 'maybe' is not one of ['adopt', 'reject']",
      "f: recommendation.rationale is shorter than 5 characters -- too thin to be a rationale",
      "f: recommendation.single_action is shorter than 3 characters",
    ]);
  });

  test("reports shapes the python crashes on", () => {
    const bad = {
      evaluation: {
        ...good.evaluation,
        recommendation: { ...good.evaluation.recommendation, rationale: 5 },
      },
    };
    expect(checkTechEvaluation("f", bad, schema)).toEqual([
      "f: recommendation.rationale must be a string (got int)",
    ]);
  });
});
