import { describe, expect, test } from "bun:test";
import { join } from "node:path";
import { checkModelSelection } from "../../skills/project-mgmt/plan-review/scripts/validate-model-selection";
import { registerGoldenTests } from "../validator-recording/golden";

const SCRIPT = join(
  import.meta.dir,
  "../../skills/project-mgmt/plan-review/scripts/validate-model-selection.ts",
);

registerGoldenTests(SCRIPT, join(import.meta.dir, "fixtures-model-selection"));

describe("checkModelSelection", () => {
  test("returns no error for a good selection", () => {
    expect(
      checkModelSelection("f", {
        model_selection: { question_text: "q", options: ["a"] },
      }),
    ).toEqual([]);
  });

  test("names a missing field and an options value that is not a list or is empty", () => {
    expect(
      checkModelSelection("f", { model_selection: { options: "a" } }),
    ).toEqual([
      "f: model_selection missing required field 'question_text'",
      "f: options must be an array",
    ]);
    expect(
      checkModelSelection("f", {
        model_selection: { question_text: "q", options: [] },
      }),
    ).toEqual(["f: options must have at least one entry"]);
  });

  test("reports a selection that is not a mapping", () => {
    expect(checkModelSelection("f", { model_selection: "x" })).toEqual([
      "f: 'model_selection' must be a mapping (got str)",
    ]);
  });
});
