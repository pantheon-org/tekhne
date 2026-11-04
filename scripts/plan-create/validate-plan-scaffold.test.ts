import { describe, expect, test } from "bun:test";
import { join } from "node:path";
import { checkPlanScaffold } from "../../skills/project-mgmt/plan-create/scripts/validate-plan-scaffold";
import { registerGoldenTests } from "../validator-recording/golden";

const SCRIPT = join(
  import.meta.dir,
  "../../skills/project-mgmt/plan-create/scripts/validate-plan-scaffold.ts",
);

registerGoldenTests(SCRIPT, join(import.meta.dir, "fixtures"));

describe("checkPlanScaffold", () => {
  const good = {
    plan: {
      filename: "p.md",
      frontmatter: { title: "t", type: "PLAN", status: "DRAFT", date: "d" },
      goal: "g",
      phases: [{ name: "one", tasks: ["a"] }],
      open_questions: ["q"],
    },
  };

  test("returns no error for a good scaffold", () => {
    expect(checkPlanScaffold("f", good)).toEqual([]);
  });

  test("names a missing required field and a missing frontmatter key", () => {
    const bad = {
      plan: {
        ...good.plan,
        goal: undefined,
        frontmatter: { title: "t", type: "PLAN", status: "DRAFT" },
      },
    };
    delete (bad.plan as Record<string, unknown>).goal;
    expect(checkPlanScaffold("f", bad)).toEqual([
      "f: plan missing required field 'goal'",
      "f: frontmatter missing 'date'",
    ]);
  });

  test("counts tasks per phase, naming the phase", () => {
    const nine = Array.from({ length: 9 }, (_, i) => `t${i}`);
    const bad = {
      plan: { ...good.plan, phases: [{ name: "one", tasks: nine }] },
    };
    expect(checkPlanScaffold("f", bad)).toEqual([
      "f: phase 'one' has 9 tasks (max 8)",
    ]);
  });

  test("reports shapes the python crashes on", () => {
    expect(checkPlanScaffold("f", { plan: ["x"] })).toEqual([
      "f: 'plan' must be a mapping (got list)",
    ]);
    expect(
      checkPlanScaffold("f", { plan: { ...good.plan, phases: ["x"] } }),
    ).toEqual(["f: phase 1 must be a mapping (got str)"]);
  });
});
